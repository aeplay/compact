use super::compact::Compact;
use super::compact_vec::CompactVec;

/// A compact storage for a `String`. So far doesn't support direct mutable operations,
/// Only conversion from and to `String`/`&str`
#[derive(Clone, Default)]
pub struct CompactString {
    chars: CompactVec<u8>,
}

impl CompactString {
    /// Create an empty `CString`
    pub fn new() -> Self {
        Default::default()
    }

    /// Appends a given string slice onto the end of this `CString`.
    pub fn push_str(&mut self, string: &str) {
        self.chars.extend_from_copy_slice(string.as_bytes());
    }
}

impl ::std::ops::Deref for CompactString {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe { ::std::str::from_utf8_unchecked(&self.chars) }
    }
}

impl ::std::convert::From<String> for CompactString {
    fn from(string: String) -> CompactString {
        CompactString {
            chars: string.into_bytes().into(),
        }
    }
}

impl Compact for CompactString {
    fn is_still_compact(&self) -> bool {
        self.chars.is_still_compact()
    }

    fn dynamic_size_bytes(&self) -> usize {
        self.chars.dynamic_size_bytes()
    }

    unsafe fn compact(source: *mut Self, dest: *mut Self, new_dynamic_part: *mut u8) {
        Compact::compact(&mut (*source).chars, &mut (*dest).chars, new_dynamic_part)
    }

    unsafe fn decompact(source: *const Self) -> Self {
        CompactString {
            chars: Compact::decompact(&(*source).chars),
        }
    }
}

#[cfg(feature = "serde-serialization")]
use std::marker::PhantomData;

#[cfg(feature = "serde-serialization")]
impl ::serde::ser::Serialize for CompactString{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        serializer.serialize_str(&self)
    }
}

#[cfg(feature = "serde-serialization")]
struct CompactStringVisitor {
    marker: PhantomData<fn() -> CompactString>
}

#[cfg(feature = "serde-serialization")]
impl CompactStringVisitor {
    fn new() -> Self {
        CompactStringVisitor {
            marker: PhantomData
        }
    }
}

#[cfg(feature = "serde-serialization")]
impl<'de> ::serde::de::Visitor<'de> for CompactStringVisitor
{
    type Value = CompactString;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("A string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Ok(s.to_owned().into())
    }

    fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Ok(s.into())
    }
}

#[cfg(feature = "serde-serialization")]
impl<'de> ::serde::de::Deserialize<'de> for CompactString

{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_string(CompactStringVisitor::new())
    }
}