#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate libc;

mod mm;
mod wrappers;

use libc::*;
use std::ffi::CString;
use std::env;
use std::mem;
use std::num::Wrapping;
use std::cmp::max;
use mm::*;

use wrappers::*;

fn main() {
    mm_test();
}

unsafe fn mmapcopy(path: &CString) {
    let mut buf = mem::uninitialized();
    Stat(path, &mut buf);
    let size = buf.st_size as size_t;
    let fd = Open(path, O_RDONLY);
    let fp = Mmap(size, PROT_READ, MAP_PRIVATE, fd, 0);
    Write(STDOUT_FILENO, fp, size);
    Close(fd);
}

fn problem_9_5() {
    let path = CString::new(env::args().nth(1).unwrap()).unwrap();
    unsafe {
        mmapcopy(&path);
    }
}

const VPO_LEN: usize = 6;
const VPN_LEN: usize = 8;
const TLBT_LEN: usize = 6;
const TLBI_LEN: usize = 2;
const CO_LEN: usize = 2;
const CI_LEN: usize = 4;
const CT_LEN: usize = 6;

fn masking(n: usize, len: usize, lo: usize) -> usize {
    let ones = Wrapping(usize::max_value());
    let mask = !(ones << len) << lo;
    ((Wrapping(n) & mask) >> lo).0
}

fn decode_vaddr(vaddr: usize) {
    let vpn = masking(vaddr, VPN_LEN, VPO_LEN);
    let vpo = masking(vaddr, VPO_LEN, 0);
    let tlbt = masking(vpn, TLBT_LEN, TLBI_LEN);
    let tlbi = masking(vpn, TLBI_LEN, 0);

    println!("0b{:06b}-{:02b}-{:06b}", tlbt, tlbi, vpo);
    println!("VPN:\t\t0x{:x}", vpn);
    println!("TLBI:\t\t0x{:x}", tlbi);
    println!("TLBT:\t\t0x{:x}", tlbt);
}

fn decode_paddr(vaddr: usize, ppn: usize) {
    let ppo = masking(vaddr, VPO_LEN, 0);
    let paddr = ppo | (ppn << VPO_LEN);
    let co = masking(paddr, CO_LEN, 0);
    let ci = masking(paddr, CI_LEN, CO_LEN);
    let ct = masking(paddr, CT_LEN, CO_LEN + CI_LEN);

    println!("0b{:06b}-{:04b}-{:02b}", ct, ci, co);
    println!("CO:\t\t0x{:x}", co);
    println!("CI:\t\t0x{:x}", ci);
    println!("CT:\t\t0x{:x}", ct);
}

fn problem_9_11_vaddr() {
    decode_vaddr(0x027c);
    decode_vaddr(0x03a9);
    decode_vaddr(0x0040);
}

fn problem_9_11_paddr() {
    decode_paddr(0x027c, 0x17);
    decode_paddr(0x03a9, 0x11);
}

fn problem_9_14() {
    unsafe {
        let path = CString::new("./hello.txt").expect("CString const error");
        let mut buf: stat = mem::uninitialized();
        Stat(&path, &mut buf);
        let fd = Open(&path, O_RDWR);
        let page = Mmap(buf.st_size as size_t, PROT_WRITE, MAP_SHARED, fd, 0);
        *(page as *mut c_char) = 'J' as c_char;
        Close(fd);
    }
}

fn problem_9_15() {
    const WSIZE: usize = 4;
    const DSIZE: usize = 8;
    fn malloc_size(size: usize) -> (usize, usize) {
        assert!(size > 0);
        let asize = (size + WSIZE + DSIZE - 1) & !(DSIZE - 1);
        (asize, asize | 0x1)
    }

    fn report(size: usize) {
        let (bsize, header) = malloc_size(size);
        println!("{}\t0x{:08x}", bsize, header);
    }

    report(3);
    report(11);
    report(20);
    report(21);
}

fn problem_9_16() {
    const WSIZE: usize = 4;
    const DSIZE: usize = 8;
    const PRED: usize = WSIZE;
    const SUCC: usize = WSIZE;
    const HEADER: usize = WSIZE;
    const FOOTER: usize = WSIZE;
    fn aligned(size: usize, align: usize) -> usize {
        assert!(align.is_power_of_two());
        (size + align - 1) & !(align - 1)
    }
    fn min_block_size(align: usize, has_footer: bool) -> usize {
        let alloc_size = HEADER + FOOTER * (has_footer as usize) + 1;
        let free_size = PRED + SUCC + HEADER + FOOTER;
        max(aligned(alloc_size, align), aligned(free_size, align))
    }

    println!("{}", min_block_size(WSIZE, true));
    println!("{}", min_block_size(WSIZE, false));
    println!("{}", min_block_size(DSIZE, true));
    println!("{}", min_block_size(DSIZE, false));
}
