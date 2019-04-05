#![deny(missing_docs, missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/tokio-xattr/0.1.0")]

//! A pure-Rust library to manage extended attributes asynchronously.
//!
//! It provides support for manipulating extended attributes (xattrs)
//! on modern Unix filesystems.
//! See the attr(5) manpage for more details.
//!
//! This module uses tokio_threadpool to manage extended attributes
//! asynchronously.

extern crate futures;
extern crate tokio_io;
extern crate tokio_threadpool;

mod xattr;

pub use blocking_xattr::XAttrs;
pub use xattr::{get, list, remove, set};

use futures::Async::*;
use futures::Poll;

use std::io;
use std::io::ErrorKind::Other;

fn blocking_io<F, T>(f: F) -> Poll<T, io::Error>
where
    F: FnOnce() -> io::Result<T>,
{
    match tokio_threadpool::blocking(f) {
        Ok(Ready(Ok(v))) => Ok(v.into()),
        Ok(Ready(Err(err))) => Err(err),
        Ok(NotReady) => Ok(NotReady),
        Err(_) => Err(blocking_err()),
    }
}

fn blocking_err() -> io::Error {
    io::Error::new(
        Other,
        "`blocking` annotated I/O must be called \
         from the context of the Tokio runtime.",
    )
}
