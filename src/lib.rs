use std::io::{Error, ErrorKind, Result};
use std::slice::Iter;

const ENCODED_BIT_LEN: usize = 8;
const DECODED_BIT_LEN: usize = 5;
const BYTE_BIT_LEN: usize = 8;

pub fn encode(dest: &mut Vec<u8>, input: &[u8]) {
    let capacity = (input.len() * ENCODED_BIT_LEN + DECODED_BIT_LEN - 1) / DECODED_BIT_LEN;
    dest.reserve(capacity);
    for b in FiveBitsIter::new(input.iter()) {
        dest.push(ENCODE_SYMBOLS[b as usize]);
    }
}

pub fn decode(dest: &mut Vec<u8>, input: &[u8]) -> Result<()> {
    let mut bit_count: usize = 0;
    let mut buffer: u8 = 0;
    for b in input.iter() {
        let s = DECODE_SYMBOLS[*b as usize];
        if s < 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("invalid symbol value {:}", *b as char),
            ));
        }
        if bit_count + DECODED_BIT_LEN >= BYTE_BIT_LEN {
            bit_count = bit_count + DECODED_BIT_LEN - BYTE_BIT_LEN;
            let output = buffer | ((s as u8) >> bit_count);
            dest.push(output);
            buffer = if bit_count > 0 {
                (s as u8) << (BYTE_BIT_LEN - bit_count)
            } else {
                0
            };
        } else {
            buffer |= (s as u8) << (BYTE_BIT_LEN - DECODED_BIT_LEN - bit_count);
            bit_count += DECODED_BIT_LEN;
        }
    }
    Ok(())
}

struct FiveBitsIter<'a> {
    input: Iter<'a, u8>,

    // bit_count is effective bits count in buffer
    bit_count: usize,

    // buffer is keeping the `bit_count` bits from MSB to LSB.
    buffer: u8,
}

impl<'a> FiveBitsIter<'a> {
    fn new(input: Iter<'a, u8>) -> Self {
        Self {
            input,
            bit_count: 0,
            buffer: 0,
        }
    }
}

impl<'a> Iterator for FiveBitsIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let b1 = if self.bit_count == 0 {
            self.bit_count += BYTE_BIT_LEN;
            *self.input.next()?
        } else {
            self.buffer
        };
        if self.bit_count >= DECODED_BIT_LEN {
            let rest = BYTE_BIT_LEN - DECODED_BIT_LEN;
            let output = b1 >> (BYTE_BIT_LEN - DECODED_BIT_LEN);
            if rest > 0 {
                self.buffer = b1 << DECODED_BIT_LEN;
            }
            self.bit_count -= DECODED_BIT_LEN;
            return Some(output);
        }

        let (b2, eof) = self.input.next().map_or((0, true), |b| (*b, false));
        let output = (b1 | b2 >> self.bit_count) >> (BYTE_BIT_LEN - DECODED_BIT_LEN);
        if eof {
            self.bit_count = 0;
        } else {
            self.bit_count += BYTE_BIT_LEN - DECODED_BIT_LEN;
            self.buffer = b2 << (BYTE_BIT_LEN - self.bit_count);
        }
        Some(output)
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

    #[test]
    fn test_decode() {
        for c in CASES.iter() {
            let mut dest = Vec::new();
            assert!(decode(&mut dest, c.encoded).is_ok());
            assert_eq!(&dest, c.plain);
        }
    }

    #[test]
    fn test_decode_corner_cases() {
        const CORNER_CASES: [TestCase; 3] = [
            TestCase {
                plain: b"",
                encoded: b"C",
            },
            TestCase {
                plain: b"f",
                encoded: b"CR",
            },
            TestCase {
                plain: b"f",
                encoded: b"CR0",
            },
        ];
        for c in CORNER_CASES.iter() {
            let mut dest = Vec::new();
            assert!(decode(&mut dest, c.encoded).is_ok());
            assert_eq!(&dest, c.plain);
        }
    }

    #[test]
    fn test_decode_invalid_char() {
        let mut dest = Vec::new();

        let res = decode(&mut dest, b"U");
        assert!(res.is_err());
        let err = res.as_ref().err().unwrap();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(format!("{}", err), "invalid symbol value U");
        
        let res = decode(&mut dest, b"confuse");
        assert!(res.is_err());
        let err = res.as_ref().err().unwrap();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(format!("{}", err), "invalid symbol value u");
    }


    #[test]
    fn test_5bits_iter() {
        const INPUT: &[u8] = &[0b1101_0011, 0b1011_1001, 0b1000_0001];
        let mut it = FiveBitsIter::new(INPUT.iter());
        assert_eq!(it.next(), Some(0b11010));
        assert_eq!(it.next(), Some(0b01110));
        assert_eq!(it.next(), Some(0b11100));
        assert_eq!(it.next(), Some(0b11000));
        assert_eq!(it.next(), Some(0b00010));
        assert!(it.next().is_none());
    }
}
