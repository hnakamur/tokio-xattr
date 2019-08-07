//! Functions for working with [`xattr`] crate.
//!
//! [`xattr`]: https://crates.io/crates/xattr

use crate::{asyncify, XAttrs};
use blocking_xattr;

use std::ffi::OsStr;
use std::io;
use std::path::Path;

/// Get an extended attribute for the specified file.
///
/// # Examples
///
/// ```no_run
/// #![feature(async_await)]
///
/// # async fn dox() -> std::io::Result<()> {
/// let val = tokio_xattr::get(".", "user.myattr1").await?;
/// # Ok(())
/// # }
/// ```
pub async fn get<P, N>(path: P, name: N) -> io::Result<Option<Vec<u8>>>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    asyncify(|| Ok(blocking_xattr::get(&path, &name)?)).await
}

/// List extended attributes attached to the specified file.
///
/// # Examples
///
/// ```no_run
/// #![feature(async_await)]
///
/// # async fn dox() -> std::io::Result<()> {
/// let xattrs = tokio_xattr::list(".").await?;
/// for attr in xattrs {
///     println!("attr_name={:?}", attr);
/// }
/// # Ok(())
/// # }
/// ```
pub async fn list<P>(path: P) -> io::Result<XAttrs>
where
    P: AsRef<Path> + Send,
{
    asyncify(|| Ok(blocking_xattr::list(&path)?)).await
}

/// Remove an extended attribute from the specified file.
///
/// # Examples
///
/// ```no_run
/// #![feature(async_await)]
///
/// # async fn dox() -> std::io::Result<()> {
/// tokio_xattr::remove(".", "user.myattr1").await?;
/// # Ok(())
/// # }
/// ```
pub async fn remove<P, N>(path: P, name: N) -> io::Result<()>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    asyncify(|| Ok(blocking_xattr::remove(&path, &name)?)).await
}

/// Set an extended attribute on the specified file.
///
/// # Examples
///
/// ```no_run
/// #![feature(async_await)]
///
/// # async fn dox() -> std::io::Result<()> {
/// tokio_xattr::set(".", "user.myattr1", &[0x12, 0x34, 0x56]).await?;
/// # Ok(())
/// # }
/// ```
pub async fn set<P, N>(path: P, name: N, value: &[u8]) -> io::Result<()>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    asyncify(|| Ok(blocking_xattr::set(&path, &name, &value)?)).await
}
