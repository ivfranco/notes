use sequence_trie::SequenceTrie;

type Bit = u8;

#[derive(PartialEq)]
pub struct Record {
    index: u32,
    tail: Option<Bit>,
}

impl Record {
    pub fn new(index: u32, tail: Option<Bit>) -> Self {
        Self { index, tail }
    }
}

impl std::fmt::Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let last_symbol = match self.tail {
            None => '_',
            Some(0) => '0',
            Some(_) => '1',
        };

        write!(f, "({}, {})", self.index, last_symbol)
    }
}

pub struct RecordBuilder<I> {
    trie: SequenceTrie<Bit, u32>,
    bits: I,
    next_index: u32,
}

impl<I> RecordBuilder<I> {
    fn new(bits: I) -> Self {
        RecordBuilder {
            trie: SequenceTrie::new(),
            bits,
            next_index: 1,
        }
    }
}

impl<I> RecordBuilder<I> where I: Iterator<Item = Bit> {}

impl<I> Iterator for RecordBuilder<I>
where
    I: Iterator<Item = Bit>,
{
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        let mut prefix_index: Option<u32> = None;
        let mut tail: Option<Bit> = None;
        let mut trie = &mut self.trie;

        while let Some(bit) = self.bits.next() {
            // a simple `let Some =` causes double mutable borrow of trie
            if trie.get(Some(&bit)).is_some() {
                trie = trie.get_node_mut(Some(&bit)).unwrap();
                prefix_index = trie.value().copied();
            } else {
                trie.insert_owned(Some(bit), self.next_index);
                tail = Some(bit);
                self.next_index += 1;
                break;
            }
        }

        if prefix_index == None && tail == None {
            // self.bits iterator exhausted
            None
        } else {
            Some(Record {
                index: prefix_index.unwrap_or(0),
                tail,
            })
        }
    }
}

pub fn encode<I>(iter: I) -> RecordBuilder<I::IntoIter>
where
    I: IntoIterator<Item = Bit>,
{
    RecordBuilder::new(iter.into_iter())
}

pub fn decode<'a, I>(iter: I) -> Vec<Bit>
where
    I: IntoIterator<Item = &'a Record>,
{
    let mut prefixes: Vec<(usize, usize)> = vec![(0, 0)];
    let mut bits: Vec<Bit> = vec![];

    for Record { index, tail } in iter {
        let (prefix_idx, prefix_len) = prefixes[*index as usize];
        let parse_len = if tail.is_some() {
            prefix_len + 1
        } else {
            prefix_len
        };

        let bits_old_len = bits.len();
        bits.resize(bits_old_len + parse_len, 0);
        bits.copy_within(prefix_idx..prefix_idx + prefix_len, bits_old_len);

        if let Some(bit) = tail {
            // a parse has length at least 1, bits cannot be empty
            *bits.last_mut().unwrap() = *bit;
        }

        prefixes.push((bits_old_len, parse_len));
    }

    bits
}

#[test]
fn encode_decode_test() {
    let bits = [
        0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1,
    ];

    let records = [
        Record::new(0, Some(0)),
        Record::new(1, Some(0)),
        Record::new(2, Some(0)),
        Record::new(0, Some(1)),
        Record::new(4, Some(0)),
        Record::new(5, Some(1)),
        Record::new(3, Some(0)),
        Record::new(1, Some(1)),
        Record::new(6, Some(0)),
        Record::new(4, None),
    ];

    let encoding: Vec<_> = encode(bits.iter().copied()).collect();
    assert_eq!(&records[..], encoding.as_slice());

    let decoding = decode(records.iter());
    assert_eq!(&bits[..], decoding.as_slice());
}
