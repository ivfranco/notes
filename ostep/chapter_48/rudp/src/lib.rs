use std::{
    io::{self, Cursor, Read, Write},
    mem,
};

pub mod client;
pub mod server;

#[derive(Debug)]
pub(crate) struct Packet {
    sequential_number: u32,
    fragment_identifer: u32,
    fragment_total: u32,
    fragment_index: u32,
    datagram: Vec<u8>,
}

/// https://stackoverflow.com/questions/1098897/
const MAX_SAFE_DATAGRAM_SIZE: usize = 508;

impl Packet {
    const HEADER_SIZE: usize = mem::size_of::<u32>() * 4;
    const MAX_SAFE_SIZE: usize = MAX_SAFE_DATAGRAM_SIZE - Self::HEADER_SIZE;

    fn new(input: &[u8], sn: u32) -> Self {
        assert!(input.len() <= Self::MAX_SAFE_SIZE);

        Self {
            sequential_number: sn,
            fragment_identifer: 0,
            fragment_total: 0,
            fragment_index: 0,
            datagram: input.to_vec(),
        }
    }

    fn fragment(
        input: &[u8],
        starting_sn: u32,
        fragment_id: u32,
    ) -> impl Iterator<Item = Self> + '_ {
        input
            .chunks(Self::MAX_SAFE_SIZE)
            .zip(0u32..)
            .map(move |(chunk, i)| Self {
                sequential_number: starting_sn + i,
                fragment_identifer: fragment_id,
                fragment_total: input.len() as u32,
                fragment_index: Self::MAX_SAFE_SIZE as u32 * i,
                datagram: chunk.to_vec(),
            })
    }

    fn len(&self) -> usize {
        Self::HEADER_SIZE + self.datagram.len()
    }

    fn write_to(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut cursor = Cursor::new(buf);
        cursor.write_all(&self.sequential_number.to_be_bytes())?;
        cursor.write_all(&self.fragment_identifer.to_be_bytes())?;
        cursor.write_all(&self.fragment_total.to_be_bytes())?;
        cursor.write_all(&self.fragment_index.to_be_bytes())?;
        cursor.write_all(&self.datagram)?;
        Ok(self.len())
    }

    fn read_from(&self, buf: &[u8]) -> io::Result<Self> {
        fn read_be_u32<R: Read>(mut reader: R) -> io::Result<u32> {
            let mut buf = [0u8; mem::size_of::<u32>()];
            reader.read_exact(&mut buf)?;
            Ok(u32::from_be_bytes(buf))
        }

        let mut cursor = Cursor::new(buf);
        let sequential_number = read_be_u32(&mut cursor)?;
        let fragment_identifer = read_be_u32(&mut cursor)?;
        let fragment_total = read_be_u32(&mut cursor)?;
        let fragment_index = read_be_u32(&mut cursor)?;

        let mut datagram = Vec::new();
        cursor.read_to_end(&mut datagram)?;

        Ok(Self {
            sequential_number,
            fragment_identifer,
            fragment_total,
            fragment_index,
            datagram,
        })
    }
}
