use crate::{MailError, Result};
use std::net::Ipv4Addr;
use trust_dns_resolver::Resolver;

/// Two-step resolve of a mail domain:
/// 1. MX query to find the canonical names
/// 2. A query to find the IP
///
/// this function always choose the first entry in the answer,
/// the query result may not be a valid mail server.
pub fn resolve(domain: &str) -> Result<Ipv4Addr> {
    if domain == "localhost" {
        return Ok(Ipv4Addr::LOCALHOST);
    }

    let resolver = Resolver::default()?;
    let mx = resolver
        .mx_lookup(domain)?
        .into_iter()
        .next()
        .ok_or(MailError::DomainNotFound)?;
    let ip = resolver
        .ipv4_lookup(&mx.exchange().to_ascii())?
        .into_iter()
        .next()
        .ok_or(MailError::DomainNotFound)?;

    Ok(ip)
}

#[test]
fn resolve_test() -> Result<()> {
    let _ip = resolve("google.com")?;
    // println!("{:?}", _ip);
    Ok(())
}
