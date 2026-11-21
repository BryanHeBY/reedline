//! Holds each high input byte in one editable scalar while byte editing is active.

use std::borrow::Cow;

const FIRST: u32 = 0xe080;
const LAST: u32 = 0xe0ff;

pub(crate) fn encode(text: &str) -> String {
    text.bytes().map(encode_byte).collect()
}

pub(crate) fn encode_byte(byte: u8) -> char {
    if byte.is_ascii() {
        char::from(byte)
    } else {
        char::from_u32(0xe000 + u32::from(byte)).expect("private-use scalar")
    }
}

pub(crate) fn decode_byte(c: char) -> Option<u8> {
    let value = u32::from(c);
    if (FIRST..=LAST).contains(&value) {
        Some((value - 0xe000) as u8)
    } else {
        None
    }
}

pub(crate) fn decode(text: &str) -> Cow<'_, str> {
    if !text.chars().any(|c| decode_byte(c).is_some()) {
        return Cow::Borrowed(text);
    }
    let mut bytes = Vec::with_capacity(text.len());
    let mut encoded = [0; 4];
    for c in text.chars() {
        if let Some(byte) = decode_byte(c) {
            bytes.push(byte);
        } else {
            bytes.extend(c.encode_utf8(&mut encoded).as_bytes());
        }
    }
    String::from_utf8_lossy(&bytes).into_owned().into()
}

pub(crate) fn offset(text: &str, byte_position: usize) -> usize {
    text.char_indices()
        .nth(byte_position)
        .map_or(text.len(), |(offset, _)| offset)
}
