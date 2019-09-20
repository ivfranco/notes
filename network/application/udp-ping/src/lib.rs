use internet_checksum::checksum;
use std::io::{self, ErrorKind};

pub mod consts {
    pub const TYPE_LEN: usize = 1;
    pub const CODE_LEN: usize = 1;
    pub const CHECKSUM_LEN: usize = 2;
    pub const IDENTIFIER_LEN: usize = 2;
    pub const SEQUENCE_NUMBER_LEN: usize = 2;
    pub const PAYLOAD_LEN: usize = 32;
    pub const PACKET_LEN: usize =
        TYPE_LEN + CODE_LEN + CHECKSUM_LEN + IDENTIFIER_LEN + SEQUENCE_NUMBER_LEN + PAYLOAD_LEN;

    pub const TYPE_OFFSET: usize = 0;
    pub const CODE_OFFSET: usize = TYPE_OFFSET + TYPE_LEN;
    pub const CHECKSUM_OFFSET: usize = CODE_OFFSET + CODE_LEN;
    pub const IDENTIFIER_OFFSET: usize = CHECKSUM_OFFSET + CHECKSUM_LEN;
    pub const SEQUENCE_NUMBER_OFFSET: usize = IDENTIFIER_OFFSET + IDENTIFIER_LEN;
    pub const PAYLOAD_OFFSET: usize = SEQUENCE_NUMBER_OFFSET + SEQUENCE_NUMBER_LEN;

    pub const TYPE_IPV4_REQUEST: u8 = 8;
    pub const TYPE_IPV4_REPLY: u8 = 0;
    pub const CODE: u8 = 0;
    pub const PAYLOAD: &[u8; PAYLOAD_LEN] = b"abcdefghijklmnopqrstuvwabcdefghi";
}

pub enum PingError {
    InvalidChecksum,
    IdentifierMismatch,
}

use PingError::*;

impl PingError {
    pub fn into_io_error(self) -> io::Error {
        self.into()
    }

    fn message(&self) -> &'static str {
        match self {
            InvalidChecksum => "Ping: invalid checksum",
            IdentifierMismatch => "Ping: identifier or sequence number mismatch",
        }
    }
}

impl Into<io::Error> for PingError {
    fn into(self) -> io::Error {
        io::Error::new(ErrorKind::Other, self.message())
    }
}

use consts::*;

pub enum Type {
    Request,
    Reply,
}

use Type::*;

// https://tools.ietf.org/html/rfc792#page-14
//
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |     Type      |     Code      |          Checksum             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           Identifier          |        Sequence Number        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |     Data ...
// +-+-+-+-+-
pub struct PingPacket {
    packet: [u8; PACKET_LEN],
}

impl PingPacket {
    pub fn new(ty: Type, identifier: u16, sequence_number: u16) -> Self {
        let mut ping_packet = PingPacket {
            packet: [0; PACKET_LEN],
        };

        let ty_byte = match ty {
            Request => TYPE_IPV4_REQUEST,
            Reply => TYPE_IPV4_REPLY,
        };
        ping_packet
            .type_bytes()
            .copy_from_slice(&ty_byte.to_be_bytes());
        ping_packet
            .code_bytes()
            .copy_from_slice(&CODE.to_be_bytes());
        ping_packet
            .identifier_bytes()
            .copy_from_slice(&identifier.to_be_bytes());
        ping_packet
            .sequence_number_bytes()
            .copy_from_slice(&sequence_number.to_be_bytes());
        ping_packet.payload_bytes().copy_from_slice(PAYLOAD);

        let checksum = checksum(&ping_packet.packet);
        ping_packet.checksum_bytes().copy_from_slice(&checksum);
        ping_packet
    }

    fn type_bytes(&mut self) -> &mut [u8] {
        &mut self.packet[TYPE_OFFSET..TYPE_OFFSET + TYPE_LEN]
    }

    fn code_bytes(&mut self) -> &mut [u8] {
        &mut self.packet[CODE_OFFSET..CODE_OFFSET + CODE_LEN]
    }

    fn checksum_bytes(&mut self) -> &mut [u8] {
        &mut self.packet[CHECKSUM_OFFSET..CHECKSUM_OFFSET + CHECKSUM_LEN]
    }

    fn identifier_bytes(&mut self) -> &mut [u8] {
        &mut self.packet[IDENTIFIER_OFFSET..IDENTIFIER_OFFSET + IDENTIFIER_LEN]
    }

    fn sequence_number_bytes(&mut self) -> &mut [u8] {
        &mut self.packet[SEQUENCE_NUMBER_OFFSET..SEQUENCE_NUMBER_OFFSET + SEQUENCE_NUMBER_LEN]
    }

    fn payload_bytes(&mut self) -> &mut [u8] {
        &mut self.packet[PAYLOAD_OFFSET..PAYLOAD_OFFSET + PAYLOAD_LEN]
    }

    pub fn packet(&self) -> &[u8] {
        &self.packet
    }

    pub fn from_be_bytes(packet: [u8; PACKET_LEN]) -> Result<Self, PingError> {
        let mut ping_packet = PingPacket { packet };
        let mut checksum_bytes = [0u8; 2];
        checksum_bytes.copy_from_slice(ping_packet.checksum_bytes());
        ping_packet.checksum_bytes().copy_from_slice(&[0, 0]);
        let checksum = checksum(ping_packet.packet());
        if checksum == checksum_bytes {
            Ok(ping_packet)
        } else {
            Err(PingError::InvalidChecksum)
        }
    }

    pub fn identifier(&self) -> u16 {
        let mut identifier_bytes = [0; IDENTIFIER_LEN];
        identifier_bytes
            .copy_from_slice(&self.packet[IDENTIFIER_OFFSET..IDENTIFIER_OFFSET + IDENTIFIER_LEN]);
        u16::from_be_bytes(identifier_bytes)
    }

    pub fn sequence_number(&self) -> u16 {
        let mut sequence_number_bytes = [0; SEQUENCE_NUMBER_LEN];
        sequence_number_bytes.copy_from_slice(
            &self.packet[SEQUENCE_NUMBER_OFFSET..SEQUENCE_NUMBER_OFFSET + SEQUENCE_NUMBER_LEN],
        );
        u16::from_be_bytes(sequence_number_bytes)
    }
}
