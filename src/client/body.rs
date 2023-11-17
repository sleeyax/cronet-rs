// This implementation is inspired by: https://github.com/seanmonstar/reqwest/blob/6792f697fcdb27c47dcbf7bd05f23368d1d4ac80/src/blocking/body.rs
// License: https://github.com/seanmonstar/reqwest/blob/master/LICENSE-MIT

use bytes::{Bytes, BytesMut};
use std::fmt;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Body {
    kind: Kind,
}

impl Body {
    pub fn new<R: Read + Send + 'static>(reader: R) -> Body {
        Body {
            kind: Kind::Reader(Box::from(reader), None),
        }
    }

    pub fn sized<R: Read + Send + 'static>(reader: R, len: u64) -> Body {
        Body {
            kind: Kind::Reader(Box::from(reader), Some(len)),
        }
    }

    /// Returns the body as a byte slice if the body is already buffered in
    /// memory. For streamed requests this method returns `None`.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self.kind {
            Kind::Reader(_, _) => None,
            Kind::Bytes(ref bytes) => Some(bytes.as_ref()),
            Kind::BytesMut(ref bytes) => Some(bytes.as_ref()),
        }
    }

    /// Returns the body as a mutable byte slice.
    /// Useful to build the body of a HTTP response.
    /// For HTTP requests this method returns `None`.
    pub fn as_bytes_mut(&mut self) -> Option<&mut BytesMut> {
        match self.kind {
            Kind::Reader(_, _) => None,
            Kind::Bytes(_) => None,
            Kind::BytesMut(ref mut bytes) => Some(bytes),
        }
    }

    pub(crate) fn len(&self) -> Option<u64> {
        match self.kind {
            Kind::Reader(_, len) => len,
            Kind::Bytes(ref bytes) => Some(bytes.len() as u64),
            Kind::BytesMut(ref bytes) => Some(bytes.len() as u64),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn try_clone(&self) -> Option<Body> {
        self.kind.try_clone().map(|kind| Body { kind })
    }
}

enum Kind {
    Reader(Box<dyn Read + Send>, Option<u64>),
    Bytes(Bytes),
    BytesMut(BytesMut),
}

impl Kind {
    fn try_clone(&self) -> Option<Kind> {
        match self {
            Kind::Reader(..) => None,
            Kind::Bytes(v) => Some(Kind::Bytes(v.clone())),
            Kind::BytesMut(v) => Some(Kind::BytesMut(v.clone())),
        }
    }
}

impl Default for Body {
    fn default() -> Body {
        Body {
            kind: Kind::BytesMut(BytesMut::new()),
        }
    }
}

impl From<Vec<u8>> for Body {
    #[inline]
    fn from(v: Vec<u8>) -> Body {
        Body {
            kind: Kind::Bytes(v.into()),
        }
    }
}

impl From<String> for Body {
    #[inline]
    fn from(s: String) -> Body {
        s.into_bytes().into()
    }
}

impl From<&'static [u8]> for Body {
    #[inline]
    fn from(s: &'static [u8]) -> Body {
        Body {
            kind: Kind::Bytes(Bytes::from_static(s)),
        }
    }
}

impl From<&'static str> for Body {
    #[inline]
    fn from(s: &'static str) -> Body {
        s.as_bytes().into()
    }
}

impl From<File> for Body {
    #[inline]
    fn from(f: File) -> Body {
        let len = f.metadata().map(|m| m.len()).ok();
        Body {
            kind: Kind::Reader(Box::new(f), len),
        }
    }
}
impl From<Bytes> for Body {
    #[inline]
    fn from(b: Bytes) -> Body {
        Body {
            kind: Kind::Bytes(b),
        }
    }
}

impl fmt::Debug for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Kind::Reader(_, ref v) => f
                .debug_struct("Reader")
                .field("length", &DebugLength(v))
                .finish(),
            Kind::Bytes(ref v) => fmt::Debug::fmt(v, f),
            Kind::BytesMut(ref v) => fmt::Debug::fmt(v, f),
        }
    }
}

struct DebugLength<'a>(&'a Option<u64>);

impl<'a> fmt::Debug for DebugLength<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Some(ref len) => fmt::Debug::fmt(len, f),
            None => f.write_str("Unknown"),
        }
    }
}
