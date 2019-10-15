//! implementation details:\
//! - mpsc::channel as event pipe and unreliable channel
//! - Receiver::recv_timeout the duration between now and a deadline updated from time to time
//! - sender, receiver and channel all live in their own threads
//! - channel takes one mpsc::Sender and one mpsc::Receiver, pipe item between them unreliably in a separated thread
//! may better to do this all in nightly with async / await but a few components of nightly are broken now

pub mod channel;
pub mod protocol;

pub use checksum::checksum;