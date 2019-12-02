use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

type PageNo = usize;
type FrameNo = PageNo;
pub type Addr = usize;

const TLB_SIZE: usize = 16;
const N_PAGES: usize = 256;
const PAGE_SIZE: usize = 256;
const N_FRAMES: usize = 128;
const MEM_SIZE: usize = N_FRAMES * PAGE_SIZE;

struct CacheEntry {
    frame: PageNo,
    last_accessed: usize,
}

struct Cache {
    size: usize,
    entries: HashMap<PageNo, CacheEntry>,
}

impl Cache {
    fn new(size: usize) -> Self {
        Self {
            size,
            entries: HashMap::new(),
        }
    }

    fn get(&self, page: PageNo) -> Option<FrameNo> {
        self.entries.get(&page).map(|entry| entry.frame)
    }

    // LRU page replacement
    fn replace_with(&mut self, page: PageNo, frame: FrameNo, curr_time: usize) {
        if self.entries.len() >= self.size {
            let replaced_page = *self
                .entries
                .iter()
                .min_by_key(|(_, entry)| entry.last_accessed)
                .expect("Cache::replace: cache size must be positive")
                .0;
            self.entries.remove(&replaced_page);
        }

        self.entries.insert(
            page,
            CacheEntry {
                frame,
                last_accessed: curr_time,
            },
        );
    }
}

#[derive(Clone, Default)]
struct PageTableEntry {
    valid: bool,
    frame: PageNo,
    last_accessed: usize,
}

struct PageTable {
    entries: Vec<PageTableEntry>,
    free_frames: Vec<FrameNo>,
}

impl PageTable {
    fn new(frames: usize) -> Self {
        Self {
            entries: vec![PageTableEntry::default(); N_PAGES],
            free_frames: (0..frames).collect::<Vec<_>>(),
        }
    }

    fn get(&self, page: PageNo) -> Option<FrameNo> {
        let entry = &self.entries[page as usize];
        if entry.valid {
            Some(entry.frame)
        } else {
            None
        }
    }

    fn free_frame(&mut self) -> FrameNo {
        if let Some(frame) = self.free_frames.pop() {
            frame
        } else {
            self.replace()
        }
    }

    fn replace(&mut self) -> FrameNo {
        let replaced_entry = self
            .entries
            .iter_mut()
            .filter(|entry| entry.valid)
            .min_by_key(|entry| entry.last_accessed)
            .expect("PageTable::replace: only called when all frames are assigned");

        replaced_entry.valid = false;
        replaced_entry.frame
    }

    fn update(&mut self, page: PageNo, frame: FrameNo, curr_time: usize) {
        let entry = &mut self.entries[page as usize];
        entry.valid = true;
        entry.frame = frame;
        entry.last_accessed = curr_time;
    }
}

pub struct MMU {
    // tlb is a cache of page table
    tlb: Cache,
    // main memory is a cache of backing store
    page_table: PageTable,
    main_memory: [u8; MEM_SIZE],
    backing_store: File,
}

pub enum MMUResponse {
    TLBHit(u8),
    PageTableHit(u8),
    PageFault(u8),
}

use MMUResponse::*;

impl MMU {
    pub fn new(backing_store: File) -> Self {
        Self {
            tlb: Cache::new(TLB_SIZE),
            page_table: PageTable::new(N_FRAMES),
            main_memory: [0; MEM_SIZE],
            backing_store,
        }
    }

    fn memory_access(&self, addr: Addr) -> u8 {
        self.main_memory[addr as usize]
    }

    pub fn access(&mut self, addr: Addr, curr_time: usize) -> io::Result<MMUResponse> {
        let (page, offset) = divide(addr);

        if let Some(frame) = self.tlb.get(page) {
            let physical_addr = combine(frame, offset);
            Ok(TLBHit(self.memory_access(physical_addr)))
        } else if let Some(frame) = self.page_table.get(page) {
            let physical_addr = combine(frame, offset);
            self.tlb.replace_with(page, frame, curr_time);
            Ok(PageTableHit(self.memory_access(physical_addr)))
        } else {
            // page fault
            let frame = self.page_table.free_frame();
            self.swap_in(page, frame)?;
            self.page_table.update(page, frame, curr_time);
            self.tlb.replace_with(page, frame, curr_time);
            let physical_addr = combine(frame, offset);
            Ok(PageFault(self.memory_access(physical_addr)))
        }
    }

    fn swap_in(&mut self, page: PageNo, frame: FrameNo) -> io::Result<()> {
        let frame_ptr = &mut self.main_memory[frame * PAGE_SIZE..(frame + 1) * PAGE_SIZE];
        self.backing_store
            .seek(SeekFrom::Start((page * PAGE_SIZE) as u64))?;
        self.backing_store.read_exact(frame_ptr)?;
        Ok(())
    }
}

fn divide(addr: Addr) -> (PageNo, usize) {
    let page = ((addr & 0xff00) >> 8) as PageNo;
    let offset = addr & 0x00ff;
    (page, offset)
}

fn combine(frame: PageNo, offset: usize) -> Addr {
    (frame << 8) | offset
}
