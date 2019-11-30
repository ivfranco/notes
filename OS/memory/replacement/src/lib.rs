use std::collections::{HashMap, VecDeque};

pub trait Replacement {
    /// Return false if the page accessed is not in physical memory.
    fn access(&mut self, references: &[u32], idx: usize) -> bool;
    fn replace(&mut self, references: &[u32], idx: usize);
    /// Return true if a page fault happened.
    fn allocate(&mut self, references: &[u32], idx: usize) -> bool {
        assert!(idx < references.len());
        if self.access(references, idx) {
            false
        } else {
            self.replace(references, idx);
            true
        }
    }
}

pub struct FIFO {
    frames: usize,
    pages: VecDeque<u32>,
}

impl FIFO {
    pub fn new(frames: usize) -> Self {
        Self {
            frames,
            pages: VecDeque::with_capacity(frames),
        }
    }
}

impl Replacement for FIFO {
    fn access(&mut self, references: &[u32], idx: usize) -> bool {
        self.pages.contains(&references[idx])
    }

    fn replace(&mut self, references: &[u32], idx: usize) {
        let reference = references[idx];
        if self.pages.len() >= self.frames {
            self.pages.pop_back();
        }
        self.pages.push_front(reference);
    }
}

struct PageTableEntry {
    page: u32,
    last_accessed: usize,
}

pub struct LRU {
    frames: usize,
    pages: VecDeque<PageTableEntry>,
}

impl LRU {
    pub fn new(frames: usize) -> Self {
        Self {
            frames,
            pages: VecDeque::with_capacity(frames),
        }
    }
}

impl Replacement for LRU {
    fn access(&mut self, references: &[u32], idx: usize) -> bool {
        let page = references[idx];
        if let Some(entry) = self.pages.iter_mut().find(|entry| entry.page == page) {
            entry.last_accessed = idx;
            true
        } else {
            false
        }
    }

    fn replace(&mut self, reference: &[u32], idx: usize) {
        // break tie by FIFO, VedDeque::iter iterates the deque from front to back
        if self.pages.len() >= self.frames {
            let (replaced_idx, _) = self
                .pages
                .iter()
                .enumerate()
                .min_by_key(|(_, entry)| entry.last_accessed)
                .expect("LRU::replace: Frame number must be positive");
            self.pages.remove(replaced_idx);
        }

        let page = reference[idx];

        self.pages.push_front(PageTableEntry {
            page,
            last_accessed: idx,
        });
    }
}

pub struct OPT {
    frames: usize,
    pages: VecDeque<u32>,
}

impl OPT {
    pub fn new(frames: usize) -> Self {
        Self {
            frames,
            pages: VecDeque::with_capacity(frames),
        }
    }
}

impl Replacement for OPT {
    fn access(&mut self, references: &[u32], idx: usize) -> bool {
        self.pages.contains(&references[idx])
    }

    fn replace(&mut self, references: &[u32], idx: usize) {
        // break tie by FIFO
        if self.pages.len() >= self.frames {
            let (replaced_idx, _) = self
                .pages
                .iter()
                .enumerate()
                .max_by_key(|(_, &page)| next_access(references, idx, page))
                .expect("OPT::replace: frame number must be positive");
            self.pages.remove(replaced_idx);
        }

        let page = references[idx];
        self.pages.push_front(page);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NextAccess {
    FromNow(usize),
    Never,
}

fn next_access(references: &[u32], idx: usize, page: u32) -> NextAccess {
    references[idx..]
        .iter()
        .position(|&access| access == page)
        .map_or(NextAccess::Never, NextAccess::FromNow)
}

#[derive(Clone)]
struct FrameCounter {
    loaded: Option<u32>,
    counter: usize,
}

impl FrameCounter {
    fn new() -> Self {
        Self {
            loaded: None,
            counter: 0,
        }
    }
}

pub struct CNT {
    frames: Vec<FrameCounter>,
    assoc: HashMap<u32, usize>,
}

impl CNT {
    pub fn new(n_frames: usize) -> Self {
        Self {
            frames: vec![FrameCounter::new(); n_frames],
            assoc: HashMap::new(),
        }
    }
}

impl Replacement for CNT {
    fn access(&mut self, references: &[u32], idx: usize) -> bool {
        let page = references[idx];
        self.frames.iter().any(|frame| frame.loaded == Some(page))
    }

    fn replace(&mut self, references: &[u32], page_idx: usize) {
        let page = references[page_idx];

        // disassociate this page from the frame recently stored it if any
        if let Some(frame_idx) = self.assoc.get(&page) {
            self.frames[*frame_idx].counter -= 1;
            self.assoc.remove(&page);
        }

        // None is smaller than Some(i) for any i, empty frame is replaced first
        let (frame_idx, replaced_frame) = self
            .frames
            .iter_mut()
            .enumerate()
            .min_by_key(|(_, frame)| frame.loaded)
            .expect("CNT::replace: frame number must be positive");

        replaced_frame.loaded = Some(page);
        replaced_frame.counter += 1;
        self.assoc.insert(page, frame_idx);
    }
}
