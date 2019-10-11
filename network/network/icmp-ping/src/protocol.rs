use crate::{checksum, ZERO_SUM};
use log::{debug, warn};
use std::ops::Deref;

pub mod consts {
    pub const ECHO_TYPE: u8 = 8;
    pub const ECHO_REPLY_TYPE: u8 = 0;
    pub const ECHO_CODE: u8 = 0;

    pub const TYPE_LEN: usize = 1;
    pub const CODE_LEN: usize = 1;
    pub const CHECKSUM_LEN: usize = 2;
    pub const IDENT_LEN: usize = 2;
    pub const SEQ_LEN: usize = 2;
    pub const DATA_LEN: usize = 32;

    pub const TYPE_OFFSET: usize = 0;
    pub const CODE_OFFSET: usize = TYPE_OFFSET + TYPE_LEN;
    pub const CHECKSUM_OFFSET: usize = CODE_OFFSET + CODE_LEN;
    pub const IDENT_OFFSET: usize = CHECKSUM_OFFSET + CHECKSUM_LEN;
    pub const SEQ_OFFSET: usize = IDENT_OFFSET + IDENT_LEN;
    pub const DATA_OFFSET: usize = SEQ_OFFSET + SEQ_LEN;

    pub const PACKET_LEN: usize = DATA_OFFSET + DATA_LEN;

    pub const PAYLOAD: &[u8; DATA_LEN] = b"abcdefghijklmnopqrstuvwabcdefghi";
}

use consts::*;

//     0                   1                   2                   3
//     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |     Type      |     Code      |          Checksum             |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |           Identifier          |        Sequence Number        |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |     Data ...
//    +-+-+-+-+-
pub struct ICMPEcho {
    packet: [u8; PACKET_LEN],
}

impl ICMPEcho {
    pub fn new(ident: u16, seq: u16) -> Self {
        let mut packet = [0; PACKET_LEN];
        packet[TYPE_OFFSET] = ECHO_TYPE;
        packet[CODE_OFFSET] = ECHO_CODE;
        packet[IDENT_OFFSET..IDENT_OFFSET + IDENT_LEN].copy_from_slice(&ident.to_be_bytes());
        packet[SEQ_OFFSET..SEQ_OFFSET + SEQ_LEN].copy_from_slice(&seq.to_be_bytes());
        packet[DATA_OFFSET..DATA_OFFSET + DATA_LEN].copy_from_slice(PAYLOAD);

        let checksum_bytes = checksum(&packet);
        packet[CHECKSUM_OFFSET..CHECKSUM_OFFSET + CHECKSUM_LEN].copy_from_slice(&checksum_bytes);
        assert_eq!(checksum(&packet), ZERO_SUM);

        Self { packet }
    }

    pub fn from_reply(reply: &[u8]) -> Option<Self> {
        if reply.len() != PACKET_LEN {
            warn!(
                "Echo reply truncated or extended, expecting length: {}, received length: {}",
                PACKET_LEN,
                reply.len()
            );
            debug!("{:?}", reply);
            return None;
        }

        if checksum(reply) != ZERO_SUM {
            warn!("Echo reply corrupted");
            return None;
        }

        if reply[TYPE_OFFSET] != ECHO_REPLY_TYPE || reply[CODE_OFFSET] != ECHO_CODE {
            warn!("Wrong echo reply type / code");
            return None;
        }

        let mut packet = [0; PACKET_LEN];
        packet.copy_from_slice(reply);

        Some(Self { packet })
    }

    pub fn ident(&self) -> u16 {
        let mut bytes = [0; IDENT_LEN];
        bytes.copy_from_slice(&self.packet[IDENT_OFFSET..IDENT_OFFSET + IDENT_LEN]);
        u16::from_be_bytes(bytes)
    }

    pub fn seq(&self) -> u16 {
        let mut bytes = [0; SEQ_LEN];
        bytes.copy_from_slice(&self.packet[SEQ_OFFSET..SEQ_OFFSET + SEQ_LEN]);
        u16::from_be_bytes(bytes)
    }
}

impl Deref for ICMPEcho {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.packet
    }
}
