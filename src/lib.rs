use std::io::{Error, ErrorKind, Result};

const DECODED_BIT_LEN: usize = 5;
const BYTE_BIT_LEN: usize = 8;

pub fn decode_to_string<'a, I>(input: I) -> Result<String>
where
    I: IntoIterator<Item = &'a u8>,
{
    let mut dest = String::new();
    append_decoded_to_string(&mut dest, input)?;
    Ok(dest)
}

pub fn decode_to_vec<'a, I>(input: I) -> Result<Vec<u8>>
where
    I: IntoIterator<Item = &'a u8>,
{
    let mut dest = Vec::new();
    append_decoded_to_vec(&mut dest, input)?;
    Ok(dest)
}

pub fn encode_to_string<'a, I>(input: I) -> String
where
    I: IntoIterator<Item = &'a u8>,
{
    let mut dest = String::new();
    append_encoded_to_string(&mut dest, input);
    dest
}

pub fn encode_to_vec<'a, I>(input: I) -> Vec<u8>
where
    I: IntoIterator<Item = &'a u8>,
{
    let mut dest = Vec::new();
    append_encoded_to_vec(&mut dest, input);
    dest
}

pub fn append_decoded_to_string<'a, I>(dest: &mut String, input: I) -> Result<()>
where
    I: IntoIterator<Item = &'a u8>,
{
    for b in DecodeIter::new(input.into_iter()) {
        dest.push(b? as char);
    }
    Ok(())
}

pub fn append_decoded_to_vec<'a, I>(dest: &mut Vec<u8>, input: I) -> Result<()>
where
    I: IntoIterator<Item = &'a u8>,
{
    for b in DecodeIter::new(input.into_iter()) {
        dest.push(b?);
    }
    Ok(())
}

pub fn append_encoded_to_string<'a, I>(dest: &mut String, input: I)
where
    I: IntoIterator<Item = &'a u8>,
{
    for b in FiveBitsIter::new(input.into_iter()) {
        dest.push(ENCODE_SYMBOLS[b as usize] as char);
    }
}

pub fn append_encoded_to_vec<'a, I>(dest: &mut Vec<u8>, input: I)
where
    I: IntoIterator<Item = &'a u8>,
{
    for b in FiveBitsIter::new(input.into_iter()) {
        dest.push(ENCODE_SYMBOLS[b as usize]);
    }
}

struct DecodeIter<I> {
    input: I,

    // bit_count is effective bits count in buffer
    bit_count: usize,

    // buffer is keeping the `bit_count` bits from MSB to LSB.
    buffer: u8,
}

impl<I> DecodeIter<I> {
    fn new(input: I) -> Self {
        Self {
            input,
            bit_count: 0,
            buffer: 0,
        }
    }
}

impl<'a, I> Iterator for DecodeIter<I>
where
    I: Iterator<Item = &'a u8>,
{
    type Item = Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(b) = self.input.next() {
            let s = DECODE_SYMBOLS[*b as usize];
            if s < 0 {
                return Some(Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("invalid symbol value {:}", *b as char),
                )));
            }
            if self.bit_count + DECODED_BIT_LEN >= BYTE_BIT_LEN {
                self.bit_count = self.bit_count + DECODED_BIT_LEN - BYTE_BIT_LEN;
                let output = self.buffer | ((s as u8) >> self.bit_count);
                self.buffer = if self.bit_count > 0 {
                    (s as u8) << (BYTE_BIT_LEN - self.bit_count)
                } else {
                    0
                };
                return Some(Ok(output));
            } else {
                self.buffer |= (s as u8) << (BYTE_BIT_LEN - DECODED_BIT_LEN - self.bit_count);
                self.bit_count += DECODED_BIT_LEN;
            }
        }
        None
    }
}

struct FiveBitsIter<I> {
    input: I,

    // bit_count is effective bits count in buffer
    bit_count: usize,

    // buffer is keeping the `bit_count` bits from MSB to LSB.
    buffer: u8,
}

impl<I> FiveBitsIter<I> {
    fn new(input: I) -> Self {
        Self {
            input,
            bit_count: 0,
            buffer: 0,
        }
    }
}

impl<'a, I> Iterator for FiveBitsIter<I>
where
    I: Iterator<Item = &'a u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let b1 = if self.bit_count == 0 {
            self.bit_count += BYTE_BIT_LEN;
            *self.input.next()?
        } else {
            self.buffer
        };

        let output = if self.bit_count >= DECODED_BIT_LEN {
            self.buffer = b1 << DECODED_BIT_LEN;
            self.bit_count -= DECODED_BIT_LEN;
            b1 >> (BYTE_BIT_LEN - DECODED_BIT_LEN)
        } else {
            let (b2, eof) = self.input.next().map_or((0, true), |b| (*b, false));
            let output = (b1 | b2 >> self.bit_count) >> (BYTE_BIT_LEN - DECODED_BIT_LEN);
            if eof {
                self.bit_count = 0;
            } else {
                self.bit_count += BYTE_BIT_LEN - DECODED_BIT_LEN;
                self.buffer = b2 << (BYTE_BIT_LEN - self.bit_count);
            }
            output
        };
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
        plain: &'a str,
        encoded: &'a str,
    }

    const CASES: [TestCase; 6] = [
        TestCase {
            plain: "foobar",
            encoded: "CSQPYRK1E8",
        },
        TestCase {
            plain: "Hello, world!",
            encoded: "91JPRV3F5GG7EVVJDHJ22",
        },
        TestCase {
            plain: "The quick brown fox jumps over the lazy dog.",
            encoded: "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPEBG",
        },
        TestCase {
            plain: "Wow, it really works!",
            encoded: "AXQQEB10D5T20WK5C5P6RY90EXQQ4TVK44",
        },
        TestCase {
            plain: "f",
            encoded: "CR",
        },
        TestCase {
            plain: "f0",
            encoded: "CRR0",
        },
    ];

    #[test]
    fn test_encode_to_string() {
        for c in CASES.iter() {
            assert_eq!(encode_to_string(c.plain.as_bytes()), c.encoded);
        }
    }

    #[test]
    fn test_encode_to_vec() {
        for c in CASES.iter() {
            assert_eq!(encode_to_vec(c.plain.as_bytes()), c.encoded.as_bytes());
        }
    }

    #[test]
    fn test_decode_to_string() {
        for c in CASES.iter() {
            let ret = decode_to_string(c.encoded.as_bytes());
            assert!(ret.is_ok());
            assert_eq!(ret.ok().unwrap(), c.plain);
        }
    }

    #[test]
    fn test_decode_to_vec() {
        for c in CASES.iter() {
            let ret = decode_to_vec(c.encoded.as_bytes());
            assert!(ret.is_ok());
            assert_eq!(ret.ok().unwrap(), c.plain.as_bytes());
        }
    }

    #[test]
    fn test_decode_corner_cases() {
        const CORNER_CASES: [TestCase; 3] = [
            TestCase {
                plain: "",
                encoded: "C",
            },
            TestCase {
                plain: "f",
                encoded: "CR",
            },
            TestCase {
                plain: "f",
                encoded: "CR0",
            },
        ];
        for c in CORNER_CASES.iter() {
            let ret = decode_to_string(c.encoded.as_bytes());
            assert!(ret.is_ok());
            assert_eq!(ret.ok().unwrap(), c.plain);
        }
    }

    #[test]
    fn test_decode_invalid_char() {
        let res = decode_to_string(b"U");
        assert!(res.is_err());
        let err = res.as_ref().err().unwrap();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(format!("{}", err), "invalid symbol value U");

        let res = decode_to_string(b"confuse");
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
