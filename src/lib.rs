#![deny(missing_docs, missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/tokio-xattr/0.2.0")]

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

async fn asyncify<F, T>(f: F) -> io::Result<T>
where
    F: FnOnce() -> io::Result<T> + Send + 'static,
    T: Send + 'static,
{
    tokio_executor::blocking::run(f).await
}
