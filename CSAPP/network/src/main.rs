#![allow(dead_code)]

extern crate regex;

use std::net::Ipv4Addr;
use std::env;
use regex::Regex;

fn main() {}

fn inet_aton(cp: &str) -> Option<Ipv4Addr> {
    let re = Regex::new(r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$").expect("Invalid regex");
    re.captures(cp).map(|cap| {
        let a = u8::from_str_radix(&cap[1], 10).unwrap();
        let b = u8::from_str_radix(&cap[2], 10).unwrap();
        let c = u8::from_str_radix(&cap[3], 10).unwrap();
        let d = u8::from_str_radix(&cap[4], 10).unwrap();
        Ipv4Addr::new(a, b, c, d)
    })
}

fn inet_ntoa(ip: Ipv4Addr) -> String {
    format!("{}", ip)
}

fn problem_11_1() {
    println!("{}", Ipv4Addr::from(0x0));
    println!("{}", Ipv4Addr::from(0xffffffff));
    println!("{}", Ipv4Addr::from(0x7f000001));
    println!("0x{:x}", u32::from(Ipv4Addr::new(205, 188, 160, 21)));
    println!("0x{:x}", u32::from(Ipv4Addr::new(64, 12, 149, 13)));
    println!("0x{:x}", u32::from(Ipv4Addr::new(205, 188, 146, 23)));
}

fn problem_11_2() {
    let hex = env::args().nth(1).expect("First argument missing");
    println!(
        "{}",
        Ipv4Addr::from(u32::from_str_radix(&hex[2..], 16).unwrap())
    );
}

fn problem_11_3() {
    let addr = env::args().nth(1).expect("First argument missing");
    if let Some(ipv4) = inet_aton(&addr) {
        println!("0x{:x}", u32::from(ipv4));
    } else {
        eprintln!("Bad format");
    }
}
