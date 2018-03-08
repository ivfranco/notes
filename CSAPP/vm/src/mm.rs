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

fn PACK(size: c_uint, alloc: c_uint) -> c_uint {
    size | alloc
}

unsafe fn GET<T>(p: *const T) -> c_uint {
    *(p as *const c_uint)
}

unsafe fn PUT<T>(p: *mut T, val: c_uint) {
    *(p as *mut c_uint) = val;
}

unsafe fn GET_SIZE<T>(p: *const T) -> c_uint {
    GET(p) & !0x7
}

unsafe fn GET_ALLOC<T>(p: *const T) -> c_uint {
    GET(p) & 0x1
}

unsafe fn HDRP<T>(bp: *const T) -> *mut c_char {
    (bp as *mut c_char).offset(-WSIZE)
}

unsafe fn FTRP<T>(bp: *const T) -> *mut c_char {
    (bp as *mut c_char).offset(GET_SIZE(bp) as isize - DSIZE)
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
    PUT(HEAP_LISTP, 0); // alignment padding
    PUT(HEAP_LISTP.offset(1 * WSIZE), PACK(DSIZE as c_uint, 1)); // prologue header
    PUT(HEAP_LISTP.offset(2 * WSIZE), PACK(DSIZE as c_uint, 1)); // prologue footer
    PUT(HEAP_LISTP.offset(3 * WSIZE), PACK(0, 1)); // epilogue header
    HEAP_LISTP = HEAP_LISTP.offset(2 * WSIZE); // initially points to prologue footer

    if extend_heap((CHUNKSIZE / WSIZE) as size_t).is_null() {
        return -1;
    }
    return 0;
}

unsafe fn extend_heap(words: size_t) -> *const c_void {
    // maintains alignment
    let size = if words % 2 == 0 {
        words * WSIZE as size_t
    } else {
        (words + 1) * WSIZE as size_t
    };

    let bp = mem_sbrk(size as c_int);
    if bp as c_long == -1 {
        return NULL;
    }

    PUT(HDRP(bp), PACK(size as c_uint, 0)); // free block header
    PUT(FTRP(bp), PACK(size as c_uint, 0)); // free block footer
    PUT(HDRP(NEXT_BLKP(bp)), PACK(0, 1)); // safe to write the new epilogue header at HDRP(NEXT_BLKP(bp))
                                          // while NEXT_BLKP(bp) may not be safe
                                          // invariant: MEM_BRK points to the epilogue header

    return coalesce(bp);
}

pub unsafe fn mm_free(bp: *const c_void) {
    let size = GET_SIZE(HDRP(bp));
    PUT(HDRP(bp), PACK(size, 0));
    PUT(FTRP(bp), PACK(size, 0));
    coalesce(bp);
}

unsafe fn coalesce(bp: *const c_void) -> *const c_void {
    let prev_alloc = GET_ALLOC(FTRP(PREV_BLKP(bp))) == 0x1;
    let next_alloc = GET_ALLOC(HDRP(NEXT_BLKP(bp))) == 0x1;
    let mut size = GET_SIZE(HDRP(bp));

    if prev_alloc && next_alloc {
        return bp;
    } else if prev_alloc && !next_alloc {
        size += GET_SIZE(HDRP(NEXT_BLKP(bp)));
        PUT(HDRP(bp), PACK(size, 0));
        PUT(FTRP(bp), PACK(size, 0));
        return bp;
    } else if !prev_alloc && next_alloc {
        size += GET_SIZE(HDRP(PREV_BLKP(bp)));
        PUT(HDRP(PREV_BLKP(bp)), PACK(size, 0));
        PUT(FTRP(bp), PACK(size, 0));
        return PREV_BLKP(bp) as *const c_void;
    } else {
        size += GET_SIZE(HDRP(PREV_BLKP(bp)));
        size += GET_SIZE(HDRP(NEXT_BLKP(bp)));
        PUT(HDRP(PREV_BLKP(bp)), PACK(size, 0));
        PUT(FTRP(NEXT_BLKP(bp)), PACK(size, 0));
        return PREV_BLKP(bp) as *const c_void;
    }
}

unsafe fn find_fit(asize: size_t) -> *const c_void {
    let mut p = HEAP_LISTP;
    while p < MEM_BRK {
        if GET_SIZE(HDRP(p)) as size_t >= asize {
            return p as *const c_void;
        }
        p = NEXT_BLKP(p);
    }
    return NULL;
}

unsafe fn place(bp: *const c_void, asize: size_t) {
    let bsize = GET_SIZE(HDRP(bp));
    if bsize as size_t >= asize + 2 * DSIZE as size_t {
        PUT(HDRP(bp), PACK(asize as c_uint, 1));
        PUT(FTRP(bp), PACK(asize as c_uint, 1));
        let ssize = bsize - asize as c_uint;
        PUT(HDRP(NEXT_BLKP(bp)), PACK(ssize, 0));
        PUT(FTRP(NEXT_BLKP(bp)), PACK(ssize, 0));
    } else {
        PUT(HDRP(bp), PACK(bsize, 1));
        PUT(FTRP(bp), PACK(bsize, 1));
    }
}

pub unsafe fn mm_malloc(size: size_t) -> *mut c_void {
    if size == 0 {
        return NULL;
    }

    let dsize = DSIZE as size_t;
    let asize = if size <= dsize {
        2 * dsize
    } else {
        dsize * ((size + 2 * dsize - 1) / dsize)
    };

    let mut bp = find_fit(asize);
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
