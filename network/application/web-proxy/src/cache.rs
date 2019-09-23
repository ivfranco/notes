//! In-memory cache of HTTP response bodies.\
//! WIP, not yet integrated into the proxy server.

use crate::GMTDateTime;
use std::collections::HashMap;

/// An entry in the HTTP cache.
pub struct CacheEntry {
    /// Last-Modified: date upon dispose
    pub last_modified: GMTDateTime,
    /// Whether the stored body has Transfer-Encoding: chunked set
    pub is_chunked: bool,
    /// Stored body in bytes
    pub body: Vec<u8>,
}

impl CacheEntry {
    /// Construct a new cache entry from fields.
    pub fn new(last_modified: GMTDateTime, is_chunked: bool, body: Vec<u8>) -> Self {
        Self {
            last_modified,
            is_chunked,
            body,
        }
    }

    /// Length of the stored body in bytes.
    pub fn content_length(&self) -> usize {
        self.body.len()
    }
}

/// In-memory cache of HTTP response bodies.
#[derive(Default)]
pub struct Cache {
    entries: HashMap<String, CacheEntry>,
}

impl Cache {
    /// Construct a new empty in-memory cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an entry into the cache, possibly overwrite an older cache.\
    /// TODO: should compare last_modified of both and insert the more recent.
    pub fn insert(&mut self, host: &str, path: &str, entry: CacheEntry) {
        let key = construct_key(host, path);
        self.entries.insert(key, entry);
    }

    /// Retrieve or invalidate an cache entry based on the Last-Modified: header from server.
    pub fn get_or_invalidate(
        &mut self,
        host: &str,
        path: &str,
        last_modified: &GMTDateTime,
    ) -> Option<&CacheEntry> {
        let key = construct_key(host, path);
        let outdated = self.entries.get(&key)?.last_modified < *last_modified;

        if outdated {
            self.entries.remove(&key);
            None
        } else {
            self.entries.get(&key)
        }
    }
}

fn construct_key(host: &str, path: &str) -> String {
    host.to_string() + path
}
