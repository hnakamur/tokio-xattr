#![deny(missing_docs, missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/tokio-xattr/0.2.0")]
#![feature(async_await)]

//! A pure-Rust library to manage extended attributes asynchronously.
//!
//! It provides support for manipulating extended attributes (xattrs)
//! on modern Unix filesystems.
//! See the attr(5) manpage for more details.
//!
//! This module uses tokio_threadpool to manage extended attributes
//! asynchronously.

mod xattr;

pub use blocking_xattr::XAttrs;
pub use xattr::{get, list, remove, set};

use std::io;
use std::io::ErrorKind::Other;
use std::task::Poll;
use std::task::Poll::*;

fn blocking_io<F, T>(f: F) -> Poll<io::Result<T>>
where
    F: FnOnce() -> io::Result<T>,
{
    match tokio_threadpool::blocking(f) {
        Ready(Ok(v)) => Ready(v),
        Ready(Err(_)) => Ready(Err(blocking_err())),
        Pending => Pending,
    }
}

async fn asyncify<F, T>(f: F) -> io::Result<T>
where
    F: FnOnce() -> io::Result<T>,
{
    use futures_util::future::poll_fn;

    let mut f = Some(f);
    poll_fn(move |_| blocking_io(|| f.take().unwrap()())).await
}

fn blocking_err() -> io::Error {
    io::Error::new(
        Other,
        "`blocking` annotated I/O must be called \
         from the context of the Tokio runtime.",
    )
}
