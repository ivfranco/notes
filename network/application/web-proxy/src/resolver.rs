//! A wrapper dns resolver over functionality of trust-dns-resolver.

use crate::{Error, Result};
use std::net::Ipv4Addr;
use trust_dns_resolver::{error::ResolveError, Resolver};

/// A DNS resolver backed by google dns services.
pub struct DNSResolver {
    inner: Resolver,
}

impl DNSResolver {
    /// Construct and configure a new dns resolver with google services.
    pub fn spawn() -> Result<Self> {
        let inner = Resolver::default()?;
        Ok(Self { inner })
    }

    /// Lookup an IPv4 address of the given domain.\
    /// Always retrieve the first entry in answers when available.
    pub fn lookup(&self, domain: &str) -> Result<Ipv4Addr> {
        let lookup = self.inner.ipv4_lookup(domain)?;
        lookup
            .into_iter()
            .next()
            .ok_or_else(|| ResolveError::from("No records found"))
            .map_err(Error::from)
    }
}
