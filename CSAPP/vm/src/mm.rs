use libc::*;
use std::cmp::max;

use wrappers::*;

const MAX_HEAP: size_t = 1 << 24;
const WSIZE: isize = 4;
const DSIZE: isize = 8;
const CHUNKSIZE: isize = 1 << 12;

static mut MEM_HEAP: *mut c_char = NULL as *mut c_char;
static mut MEM_BRK: *mut c_char = NULL as *mut c_char;
static mut MEM_MAX_ADDR: *mut c_char = NULL as *mut c_char;
static mut HEAP_LISTP: *mut c_char = NULL as *mut c_char;
static mut PREV_LISTP: *mut c_char = NULL as *mut c_char;

fn PACK(size: c_uint, prev_alloc: c_uint, alloc: c_uint) -> c_uint {
    size | (prev_alloc << 1) | alloc
}

unsafe fn GET<T>(p: *const T) -> c_uint {
    *(p as *const c_uint)
}

unsafe fn PUT<T>(p: *mut T, val: c_uint) {
    // println!("0x{:016x}", p as usize);
    // println!(
    //     "{}, {}, {}",
    //     GET_SIZE(&val),
    //     GET_PREV_ALLOC(&val),
    //     GET_ALLOC(&val)
    // );
    *(p as *mut c_uint) = val;
}

unsafe fn GET_SIZE<T>(p: *const T) -> c_uint {
    GET(p) & !0x7
}

unsafe fn GET_ALLOC<T>(p: *const T) -> c_uint {
    GET(p) & 0x1
}

unsafe fn GET_PREV_ALLOC<T>(p: *const T) -> c_uint {
    (GET(p) & 0x2) >> 1
}

unsafe fn HDRP<T>(bp: *const T) -> *mut c_char {
    (bp as *mut c_char).offset(-WSIZE)
}

unsafe fn FTRP<T>(bp: *const T) -> *mut c_char {
    (bp as *mut c_char).offset(GET_SIZE(HDRP(bp)) as isize - DSIZE)
}

unsafe fn NEXT_BLKP<T>(bp: *const T) -> *mut c_char {
    let cbp = bp as *mut c_char;
    cbp.offset(GET_SIZE(cbp.offset(-WSIZE)) as isize)
}

unsafe fn PREV_BLKP<T>(bp: *const T) -> *mut c_char {
    let cbp = bp as *mut c_char;
    cbp.offset(-(GET_SIZE(cbp.offset(-DSIZE)) as isize))
}

unsafe fn mem_init() {
    MEM_HEAP = Malloc(MAX_HEAP) as *mut c_char;
    MEM_BRK = MEM_HEAP;
    MEM_MAX_ADDR = MEM_HEAP.offset(MAX_HEAP as isize);
}

unsafe fn mem_sbrk(incr: c_int) -> *mut c_void {
    let old_brk = MEM_BRK;
    if incr < 0 || MEM_BRK.offset(incr as isize) > MEM_MAX_ADDR {
        *errno() = ENOMEM;
        eprintln!("ERROR: mem_sbrk failed: Ran out of memory");
        return -1isize as *mut c_void;
    }
    MEM_BRK = MEM_BRK.offset(incr as isize);
    return old_brk as *mut c_void;
}

pub unsafe fn mm_init() -> c_int {
    mem_init();
    HEAP_LISTP = mem_sbrk(4 * WSIZE as c_int) as *mut c_char;
    if HEAP_LISTP == -1isize as *mut c_char {
        return -1;
    }
    let wsize = WSIZE as c_uint;
    PUT(HEAP_LISTP.offset(0 * WSIZE), 0); // alignment
    PUT(HEAP_LISTP.offset(1 * WSIZE), PACK(2 * wsize, 1, 1)); // prologue header
    PUT(HEAP_LISTP.offset(2 * WSIZE), PACK(2 * wsize, 1, 1)); // prologue footer
    PUT(HEAP_LISTP.offset(3 * WSIZE), PACK(0, 1, 1)); // epilogue header
    HEAP_LISTP = HEAP_LISTP.offset(2 * WSIZE); // initially points to word next to prologue header
    PREV_LISTP = HEAP_LISTP;

    if extend_heap((CHUNKSIZE / WSIZE) as size_t).is_null() {
        return -1;
    }
    return 0;
}

unsafe fn extend_heap(words: size_t) -> *const c_void {
    // maintains alignment
    let size = aligned(words, 2) * WSIZE as size_t;
    let bp = mem_sbrk(size as c_int);
    if bp as c_long == -1 {
        return NULL;
    }

    // whether the block right before epilogue is allocated
    let prev_alloc = GET_PREV_ALLOC(HDRP(bp));

    PUT(HDRP(bp), PACK(size as c_uint, prev_alloc, 0)); // free block header
    PUT(FTRP(bp), PACK(size as c_uint, prev_alloc, 0)); // free block footer

    // safe to write the new epilogue header at HDRP(NEXT_BLKP(bp))
    // while NEXT_BLKP(bp) may not be safe
    // invariant: HDRP(MEM_BRK) points to the epilogue header
    PUT(HDRP(NEXT_BLKP(bp)), PACK(0, 0, 1));

    return coalesce(bp);
}

unsafe fn adjust_next_block(bp: *const c_void) {
    let alloc = GET_ALLOC(HDRP(bp));
    let next_block = NEXT_BLKP(bp);
    let nsize = GET_SIZE(HDRP(next_block));
    let nalloc = GET_ALLOC(HDRP(next_block));
    PUT(HDRP(next_block), PACK(nsize, alloc, nalloc));
    if nsize != 0 && nalloc == 0 {
        PUT(FTRP(next_block), PACK(nsize, alloc, nalloc));
    }
}

pub unsafe fn mm_free(bp: *const c_void) {
    let size = GET_SIZE(HDRP(bp));
    let prev_alloc = GET_PREV_ALLOC(HDRP(bp));
    PUT(HDRP(bp), PACK(size, prev_alloc, 0));
    PUT(FTRP(bp), PACK(size, prev_alloc, 0));

    adjust_next_block(bp);
    coalesce(bp);
}

unsafe fn coalesce(bp: *const c_void) -> *const c_void {
    let prev_alloc = GET_PREV_ALLOC(HDRP(bp)) == 0x1;
    let next_alloc = GET_ALLOC(HDRP(NEXT_BLKP(bp))) == 0x1;
    let mut size = GET_SIZE(HDRP(bp));

    if prev_alloc && next_alloc {
        return bp;
    } else if prev_alloc && !next_alloc {
        size += GET_SIZE(HDRP(NEXT_BLKP(bp)));
        PUT(HDRP(bp), PACK(size, 1, 0));
        PUT(FTRP(bp), PACK(size, 1, 0));
        return bp;
    } else if !prev_alloc && next_alloc {
        size += GET_SIZE(HDRP(PREV_BLKP(bp)));
        let pp_alloc = GET_PREV_ALLOC(HDRP(PREV_BLKP(bp)));
        PUT(HDRP(PREV_BLKP(bp)), PACK(size, pp_alloc, 0));
        PUT(FTRP(bp), PACK(size, pp_alloc, 0));
        return PREV_BLKP(bp) as *const c_void;
    } else {
        size += GET_SIZE(HDRP(PREV_BLKP(bp)));
        size += GET_SIZE(HDRP(NEXT_BLKP(bp)));
        let pp_alloc = GET_PREV_ALLOC(HDRP(PREV_BLKP(bp)));
        PUT(HDRP(PREV_BLKP(bp)), PACK(size, pp_alloc, 0));
        PUT(FTRP(NEXT_BLKP(bp)), PACK(size, pp_alloc, 0));
        return PREV_BLKP(bp) as *const c_void;
    }
}

unsafe fn fit(asize: size_t, bp: *const c_char) -> bool {
    GET_ALLOC(HDRP(bp)) == 0 && GET_SIZE(HDRP(bp)) as size_t >= asize
}

unsafe fn find_fit(asize: size_t) -> *const c_void {
    let mut p = HEAP_LISTP;
    while GET_SIZE(HDRP(p)) > 0 {
        if fit(asize, p) {
            return p as *const c_void;
        }
        p = NEXT_BLKP(p);
    }
    return NULL;
}

unsafe fn find_next_fit(asize: size_t) -> *const c_void {
    let mut bp = PREV_LISTP;
    if fit(asize, bp) {
        return bp as *const c_void;
    }

    bp = NEXT_BLKP(bp);
    if GET_SIZE(HDRP(bp)) == 0 {
        bp = HEAP_LISTP;
    }

    while bp != PREV_LISTP {
        if fit(asize, bp) {
            PREV_LISTP = bp;
            return bp as *const c_void;
        }
        bp = NEXT_BLKP(bp);
        if GET_SIZE(HDRP(bp)) == 0 {
            bp = HEAP_LISTP;
        }
    }

    return NULL;
}

unsafe fn place(bp: *const c_void, asize: size_t) {
    let bsize = GET_SIZE(HDRP(bp));
    let prev_alloc = GET_PREV_ALLOC(HDRP(bp));
    let tail_block;
    if bsize as size_t >= asize + 2 * DSIZE as size_t {
        PUT(HDRP(bp), PACK(asize as c_uint, prev_alloc, 1));
        let ssize = bsize - asize as c_uint;
        PUT(HDRP(NEXT_BLKP(bp)), PACK(ssize, 1, 0));
        PUT(FTRP(NEXT_BLKP(bp)), PACK(ssize, 1, 0));
        tail_block = NEXT_BLKP(bp) as *const c_void;
    } else {
        PUT(HDRP(bp), PACK(bsize, prev_alloc, 1));
        PUT(FTRP(bp), PACK(bsize, prev_alloc, 1));
        tail_block = bp;
    }

    adjust_next_block(tail_block);
}

fn aligned(size: size_t, align: size_t) -> size_t {
    assert!(align.is_power_of_two());
    (size + align - 1) & !(align - 1)
}

pub unsafe fn mm_malloc(size: size_t) -> *mut c_void {
    if size == 0 {
        return NULL;
    }

    let asize = aligned(size + WSIZE as size_t, DSIZE as size_t);

    let mut bp = find_next_fit(asize);
    if !bp.is_null() {
        place(bp, asize);
        return bp as *mut c_void;
    }

    let extendsize = max(asize, CHUNKSIZE as size_t);
    bp = extend_heap(extendsize / WSIZE as size_t);
    if bp.is_null() {
        return NULL;
    }
    place(bp, asize);
    return bp as *mut c_void;
}

pub unsafe fn mm_report() {
    let mut bp = NEXT_BLKP(HEAP_LISTP);
    let mut idx = 0;
    while GET_SIZE(HDRP(bp)) > 0 {
        println!("No:           {}", idx);
        println!("Address:      0x{:016x}", bp as usize);
        println!("Size:         {}", GET_SIZE(HDRP(bp)));
        println!("Prev alloc:   {}", GET_PREV_ALLOC(HDRP(bp)) != 0);
        println!("alloc:        {}\n", GET_ALLOC(HDRP(bp)) != 0);
        bp = NEXT_BLKP(bp);
        idx += 1;
    }
    if idx == 0 {
        println!("Empty heap");
    }
}

pub fn mm_test() {
    unsafe {
        mm_init();
        let p1 = mm_malloc(2);
        let p2 = mm_malloc(4);
        let p3 = mm_malloc(8);
        println!("Alloc report:");
        mm_report();
        mm_free(p1);
        mm_free(p2);
        mm_free(p3);
        println!("Free report:");
        mm_report();
    }
}
