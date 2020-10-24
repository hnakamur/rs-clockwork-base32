const ENCODED_BIT_LEN: usize = 8;
const DECODED_BIT_LEN: usize = 5;
const BYTE_BIT_LEN: usize = 8;

pub fn encode(dest: &mut Vec<u8>, input: &[u8]) {
    let capacity = input.len() * ENCODED_BIT_LEN / DECODED_BIT_LEN;
    dest.reserve(capacity);
    let mut dest_byte: u8;
    let mut dest_pos: usize = 0;
    for (i, input_byte) in input.iter().enumerate() {
        println!("i={}, input_byte={:?}", i, input_byte);
    }
}

pub fn decode(dest: &mut Vec<u8>, input: &[u8]) {}

struct FiveBitsIter<'a> {
    input: &'a [u8],
    byte_index: usize,
    bit_offset: usize,
}

fn iter_5bits<'a>(input: &'a [u8]) -> FiveBitsIter<'a> {
    FiveBitsIter {
        input,
        byte_index: 0,
        bit_offset: 0,
    }
}

impl<'a> Iterator for FiveBitsIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_index >= self.input.len() - 1 && self.bit_offset >= BYTE_BIT_LEN {
            return None;
        }

        if self.bit_offset + DECODED_BIT_LEN <= BYTE_BIT_LEN {
            let mut output =
                self.input[self.byte_index] >> (BYTE_BIT_LEN - (self.bit_offset + DECODED_BIT_LEN));
            if self.bit_offset != 0 {
                let mask = (1 << (BYTE_BIT_LEN - self.bit_offset)) - 1;
                output &= mask;
            }
            Some(output)
        } else {
            Some(0)
        }
    }
}

const ENCODE_SYMBOLS: [u8; 32] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'J', b'K', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'V', b'W', b'X', b'Y', b'Z',
];

const DECODE_SYMBOLS: [i8; 256] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 0-9 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 10-19 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 20-29 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 30-39 */
    -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, /* 40-49 */
    2, 3, 4, 5, 6, 7, 8, 9, 0, -1, /* 50-59 */
    -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, /* 60-69 */
    15, 16, 17, 1, 18, 19, 1, 20, 21, 0, /* 70-79 */
    22, 23, 24, 25, 26, -2, 27, 28, 29, 30, /* 80-89 */
    31, -1, -1, -1, -1, -1, -1, 10, 11, 12, /* 90-99 */
    13, 14, 15, 16, 17, 1, 18, 19, 1, 20, /* 100-109 */
    21, 0, 22, 23, 24, 25, 26, -1, 27, 28, /* 110-119 */
    29, 30, 31, -1, -1, -1, -1, -1, -1, -1, /* 120-129 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 130-109 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 140-109 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 150-109 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 160-109 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 170-109 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 180-109 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 190-109 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 200-209 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 210-209 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 220-209 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 230-209 */
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, /* 240-209 */
    -1, -1, -1, -1, -1, -1, /* 250-256 */
];

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        plain: &'a [u8],
        encoded: &'a [u8],
    }

    const CASES: [TestCase; 4] = [
        TestCase {
            plain: b"foobar",
            encoded: b"CSQPYRK1E8",
        },
        TestCase {
            plain: b"Hello, world!",
            encoded: b"91JPRV3F5GG7EVVJDHJ22",
        },
        TestCase {
            plain: b"The quick brown fox jumps over the lazy dog.",
            encoded: b"AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPEBG",
        },
        TestCase {
            plain: b"Wow, it really works!",
            encoded: b"AXQQEB10D5T20WK5C5P6RY90EXQQ4TVK44",
        },
    ];

    #[test]
    fn test_encode() {
        for c in CASES.iter() {
            let mut dest = Vec::new();
            encode(&mut dest, c.plain);
            assert_eq!(&dest, c.encoded);
        }
    }
}
