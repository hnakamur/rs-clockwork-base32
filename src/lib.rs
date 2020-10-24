//! Clockwork Base32 decoding and encoding
//!
//! This module contains high level functions and low level functions
//! for decoding and encoding bytes with
//! [`Clockwork Base32`](https://gist.github.com/szktty/228f85794e4187882a77734c89c384a8).
//!
//! ## Examples
//!
//! You can encode bytes to a [`String`] with [`encode_to_string`]:
//!
//! ```
//! use clockwork_base32 as base32;
//! let encoded = base32::encode_to_string(b"Hello, world!");
//! assert_eq!(&encoded, "91JPRV3F5GG7EVVJDHJ22");
//! ```
//!
//! You can decode bytes to a [`String`] with [`decode_to_string`]:
//!
//! ```
//! # fn main() -> std::io::Result<()> {
//! use clockwork_base32 as base32;
//! let decoded = base32::decode_to_string(b"91JPRV3F5GG7EVVJDHJ22")?;
//! assert_eq!(&decoded, "Hello, world!");
//! # Ok(())
//! # }
//! ```
//!
//!
//! # High level functions
//! These functions decode/encode bytes and return a new [`String`] or [`Vec<u8>`] as the result.
//! * [`decode_to_string`]
//! * [`encode_to_string`]
//! * [`decode_to_vec`]
//! * [`encode_to_vec`]
//!
//! # Low level functions
//! These functions take a [`String`] or [`Vec<u8>`] argument for the destination
//! and append the decoded/encoded result to it.
//! * [`append_decoded_to_string`]
//! * [`append_decoded_to_vec`]
//! * [`append_encoded_to_string`]
//! * [`append_encoded_to_vec`]
//!
//! These functions can be used to calculate the capacity for the decode/encode result
//! beforehand.
//! * [`capacity_hint_for_decode`]
//! * [`capacity_hint_for_encode`]

use std::io::{Error, ErrorKind, Result};

const DECODED_BIT_LEN: usize = 5;
const BYTE_BIT_LEN: usize = 8;

/// Decodes bytes and returns the result as a new [`String`].
///
/// # Errors
/// Returns [`Err`] if the input contains a invalid byte.
///
/// # Examples
/// Basic usage:
/// ```
/// # fn main() -> std::io::Result<()> {
/// use clockwork_base32::decode_to_string;
/// let decoded = decode_to_string(b"91JPRV3F5GG7EVVJDHJ22")?;
/// assert_eq!(&decoded, "Hello, world!");
/// # Ok(())
/// # }
/// ```
/// If your input is a [`str`], you can convert it to bytes using [`str::as_bytes`].
/// ```
/// # fn main() -> std::io::Result<()> {
/// use clockwork_base32 as base32;
/// let input = String::from("91JPRV3F5GG7EVVJDHJ22");
/// let decoded = base32::decode_to_string(input.as_bytes())?;
/// assert_eq!(&decoded, "Hello, world!");
/// # Ok(())
/// # }
/// ```
pub fn decode_to_string<'a, I>(input: I) -> Result<String>
where
    I: IntoIterator<Item = &'a u8>,
{
    let it = input.into_iter();
    let mut dest = String::with_capacity(capacity_hint_for_decode(it.size_hint().0));
    append_decoded_to_string(&mut dest, it)?;
    Ok(dest)
}

/// Decodes bytes and returns the result as a new [`Vec<u8>`].
///
/// # Errors
/// Returns [`Err`] if the input contains a invalid byte.
///
/// # Examples
/// ```
/// # fn main() -> std::io::Result<()> {
/// use clockwork_base32 as base32;
/// let decoded = base32::decode_to_vec(b"91JPRV3F5GG7EVVJDHJ22")?;
/// assert_eq!(&decoded, b"Hello, world!");
/// # Ok(())
/// # }
/// ```
pub fn decode_to_vec<'a, I>(input: I) -> Result<Vec<u8>>
where
    I: IntoIterator<Item = &'a u8>,
{
    let it = input.into_iter();
    let mut dest = Vec::with_capacity(capacity_hint_for_decode(it.size_hint().0));
    append_decoded_to_vec(&mut dest, it)?;
    Ok(dest)
}

/// Encodes bytes and returns the result as a new [`String`].
///
/// # Examples
/// Basic usage:
/// ```
/// use clockwork_base32 as base32;
/// let encoded = base32::encode_to_string(b"Hello, world!");
/// assert_eq!(&encoded, "91JPRV3F5GG7EVVJDHJ22");
/// ```
/// If your input is a [`str`], you can convert it to bytes using [`str::as_bytes`].
/// ```
/// use clockwork_base32 as base32;
/// let input = String::from("Hello, world!");
/// let encoded = base32::encode_to_string(input.as_bytes());
/// assert_eq!(&encoded, "91JPRV3F5GG7EVVJDHJ22");
/// ```
pub fn encode_to_string<'a, I>(input: I) -> String
where
    I: IntoIterator<Item = &'a u8>,
{
    let it = input.into_iter();
    let mut dest = String::with_capacity(capacity_hint_for_encode(it.size_hint().0));
    append_encoded_to_string(&mut dest, it);
    dest
}

/// Encodes bytes and returns the result as a new [`Vec<u8>`].
///
/// # Examples
/// ```
/// use clockwork_base32 as base32;
/// let encoded = base32::encode_to_vec(b"Hello, world!");
/// assert_eq!(&encoded, b"91JPRV3F5GG7EVVJDHJ22");
/// ```
pub fn encode_to_vec<'a, I>(input: I) -> Vec<u8>
where
    I: IntoIterator<Item = &'a u8>,
{
    let it = input.into_iter();
    let mut dest = Vec::with_capacity(capacity_hint_for_encode(it.size_hint().0));
    append_encoded_to_vec(&mut dest, it);
    dest
}

/// Returns a hint for the capacity needed for the decoded result.
/// # Examples
/// Basic usage:
/// ```
/// use clockwork_base32 as base32;
/// let capacity = base32::capacity_hint_for_decode(21);
/// assert_eq!(capacity, 13);
/// ```
/// You can reserve the needed capacity before decoding:
/// ```
/// # fn main() -> std::io::Result<()> {
/// use clockwork_base32 as base32;
/// let input = b"91JPRV3F5GG7EVVJDHJ22";
/// let capacity = base32::capacity_hint_for_encode(input.len());
/// let mut dest = String::with_capacity(capacity);
/// base32::append_decoded_to_string(&mut dest, input.into_iter())?;
/// assert_eq!(&dest, "Hello, world!");
/// # Ok(())
/// # }
/// ```
pub fn capacity_hint_for_decode(input_byte_len: usize) -> usize {
    input_byte_len * DECODED_BIT_LEN / BYTE_BIT_LEN
}

/// Returns a hint for the capacity needed for the encoded result.
/// # Examples
/// Basic usage:
/// ```
/// use clockwork_base32 as base32;
/// let capacity = base32::capacity_hint_for_encode(13);
/// assert_eq!(capacity, 21);
/// ```
/// You can reserve the needed capacity before encoding:
/// ```
/// use clockwork_base32 as base32;
/// let input = b"Hello, world!";
/// let capacity = base32::capacity_hint_for_encode(input.len());
/// let mut dest = String::with_capacity(capacity);
/// base32::append_encoded_to_string(&mut dest, input.into_iter());
/// assert_eq!(&dest, "91JPRV3F5GG7EVVJDHJ22");
/// ```
pub fn capacity_hint_for_encode(input_byte_len: usize) -> usize {
    (input_byte_len * BYTE_BIT_LEN + (DECODED_BIT_LEN - 1)) / DECODED_BIT_LEN
}

/// Decodes bytes and append the result to `dest`.
///
/// # Errors
/// Returns [`Err`] if the input contains a invalid byte.
///
/// # Examples
/// Basic usage:
/// ```
/// # fn main() -> std::io::Result<()> {
/// use clockwork_base32 as base32;
/// let mut dest = String::new();
/// base32::append_decoded_to_string(&mut dest, b"91JPRV3F5GG7EVVJDHJ22".into_iter())?;
/// assert_eq!(&dest, "Hello, world!");
/// # Ok(())
/// # }
/// ```
/// If your input is a [`str`], you can convert it to bytes using [`str::as_bytes`].
/// ```
/// # fn main() -> std::io::Result<()> {
/// use clockwork_base32 as base32;
/// let mut dest = String::new();
/// let input = String::from("91JPRV3F5GG7EVVJDHJ22");
/// base32::append_decoded_to_string(&mut dest, input.as_bytes().into_iter())?;
/// assert_eq!(&dest, "Hello, world!");
/// # Ok(())
/// # }
/// ```
/// You can reserve the needed capacity before decoding:
/// ```
/// # fn main() -> std::io::Result<()> {
/// use clockwork_base32 as base32;
/// let input = b"91JPRV3F5GG7EVVJDHJ22";
/// let capacity = base32::capacity_hint_for_decode(input.len());
/// let mut dest = String::with_capacity(capacity);
/// base32::append_decoded_to_string(&mut dest, input.into_iter())?;
/// assert_eq!(&dest, "Hello, world!");
/// # Ok(())
/// # }
/// ```
pub fn append_decoded_to_string<'a, I>(dest: &mut String, input: I) -> Result<()>
where
    I: Iterator<Item = &'a u8>,
{
    for b in DecodeIter::new(input) {
        dest.push(b? as char);
    }
    Ok(())
}

/// Decodes bytes and append the result to `dest`.
///
/// # Errors
/// Returns [`Err`] if the input contains a invalid byte.
///
/// # Examples
/// Basic usage:
/// ```
/// # fn main() -> std::io::Result<()> {
/// use clockwork_base32 as base32;
/// let mut dest = Vec::new();
/// base32::append_decoded_to_vec(&mut dest, b"91JPRV3F5GG7EVVJDHJ22".into_iter())?;
/// assert_eq!(&dest, b"Hello, world!");
/// # Ok(())
/// # }
/// ```
/// You can reserve the needed capacity before decoding:
/// ```
/// # fn main() -> std::io::Result<()> {
/// use clockwork_base32 as base32;
/// let input = b"91JPRV3F5GG7EVVJDHJ22";
/// let mut dest = Vec::with_capacity(input.len());
/// base32::append_decoded_to_vec(&mut dest, input.into_iter())?;
/// assert_eq!(&dest, b"Hello, world!");
/// # Ok(())
/// # }
/// ```
pub fn append_decoded_to_vec<'a, I>(dest: &mut Vec<u8>, input: I) -> Result<()>
where
    I: Iterator<Item = &'a u8>,
{
    for b in DecodeIter::new(input) {
        dest.push(b?);
    }
    Ok(())
}

/// Encodes bytes and append the result to `dest`.
///
/// # Examples
/// Basic usage:
/// ```
/// use clockwork_base32 as base32;
/// let mut dest = String::new();
/// base32::append_encoded_to_string(&mut dest, b"Hello, world!".into_iter());
/// assert_eq!(&dest, "91JPRV3F5GG7EVVJDHJ22");
/// ```
/// If your input is a [`str`], you can convert it to bytes using [`str::as_bytes`].
/// ```
/// use clockwork_base32 as base32;
/// let mut dest = String::new();
/// let input = String::from("Hello, world!");
/// base32::append_encoded_to_string(&mut dest, input.as_bytes().into_iter());
/// assert_eq!(&dest, "91JPRV3F5GG7EVVJDHJ22");
/// ```
/// You can reserve the needed capacity before encoding:
/// ```
/// use clockwork_base32 as base32;
/// let input = b"Hello, world!";
/// let capacity = base32::capacity_hint_for_encode(input.len());
/// let mut dest = String::with_capacity(capacity);
/// base32::append_encoded_to_string(&mut dest, input.into_iter());
/// assert_eq!(&dest, "91JPRV3F5GG7EVVJDHJ22");
/// ```
pub fn append_encoded_to_string<'a, I>(dest: &mut String, input: I)
where
    I: Iterator<Item = &'a u8>,
{
    for b in FiveBitsIter::new(input) {
        dest.push(ENCODE_SYMBOLS[b as usize] as char);
    }
}

/// Encodes bytes and append the result to `dest`.
///
/// # Examples
/// Basic usage:
/// ```
/// use clockwork_base32 as base32;
/// let mut dest = Vec::new();
/// base32::append_encoded_to_vec(&mut dest, b"Hello, world!".into_iter());
/// assert_eq!(&dest, b"91JPRV3F5GG7EVVJDHJ22");
/// ```
/// If your input is a [`str`], you can convert it to bytes using [`str::as_bytes`].
/// ```
/// use clockwork_base32 as base32;
/// let mut dest = Vec::new();
/// let input = String::from("Hello, world!");
/// base32::append_encoded_to_vec(&mut dest, input.as_bytes().into_iter());
/// assert_eq!(&dest, b"91JPRV3F5GG7EVVJDHJ22");
/// ```
/// You can reserve the needed capacity before encoding:
/// ```
/// use clockwork_base32 as base32;
/// let input = b"Hello, world!";
/// let capacity = base32::capacity_hint_for_encode(input.len());
/// let mut dest = Vec::with_capacity(capacity);
/// base32::append_encoded_to_vec(&mut dest, input.into_iter());
/// assert_eq!(&dest, b"91JPRV3F5GG7EVVJDHJ22");
/// ```
pub fn append_encoded_to_vec<'a, I>(dest: &mut Vec<u8>, input: I)
where
    I: Iterator<Item = &'a u8>,
{
    for b in FiveBitsIter::new(input) {
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
            assert_eq!(capacity_hint_for_encode(c.plain.len()), c.encoded.len());
        }
    }

    #[test]
    fn test_encode_to_vec() {
        for c in CASES.iter() {
            assert_eq!(encode_to_vec(c.plain.as_bytes()), c.encoded.as_bytes());
            assert_eq!(capacity_hint_for_encode(c.plain.len()), c.encoded.len());
        }
    }

    #[test]
    fn test_decode_to_string() {
        for c in CASES.iter() {
            let ret = decode_to_string(c.encoded.as_bytes());
            assert!(ret.is_ok());
            assert_eq!(ret.ok().unwrap(), c.plain);
            assert_eq!(capacity_hint_for_decode(c.encoded.len()), c.plain.len());
        }
    }

    #[test]
    fn test_decode_to_vec() {
        for c in CASES.iter() {
            let ret = decode_to_vec(c.encoded.as_bytes());
            assert!(ret.is_ok());
            assert_eq!(ret.ok().unwrap(), c.plain.as_bytes());
            assert_eq!(capacity_hint_for_decode(c.encoded.len()), c.plain.len());
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
