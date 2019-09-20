use crate::{Error, Result};
use std::net::Ipv4Addr;
use trust_dns_resolver::{error::ResolveError, Resolver};

pub struct DNSResolver {
    inner: Resolver,
}

impl DNSResolver {
    /// Construct and configure a new dns resolver with google services, spawn background process
    pub fn spawn() -> Result<Self> {
        let inner = Resolver::default()?;
        Ok(Self { inner })
    }

    pub fn lookup(&self, domain: &str) -> Result<Ipv4Addr> {
        let lookup = self.inner.ipv4_lookup(domain)?;
        lookup
            .into_iter()
            .next()
            .ok_or_else(|| ResolveError::from("No records found"))
            .map_err(Error::from)
    }
}
