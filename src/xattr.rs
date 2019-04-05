//! Functions for working with [`xattr`] crate.
//!
//! [`xattr`]: https://crates.io/crates/xattr

extern crate blocking_xattr;
extern crate tokio;

use super::blocking_io;
use futures::{Future, Poll};
use std::ffi::OsStr;
use std::io;
use std::path::Path;

/// Get an extended attribute for the specified file.
///
/// # Examples
///
/// ```
/// # extern crate futures;
/// # extern crate tokio;
/// # extern crate tokio_xattr;
/// use futures::Future;
///
/// fn main() {
///     let fut = tokio_xattr::get(".", "user.myattr1").map(|val| {
///         println!("val={:?}", val);
///     }).map_err(|err| { eprintln!("Error: {:?}", err); () });
///     tokio::run(fut);
/// }
/// ```
pub fn get<P, N>(path: P, name: N) -> GetFuture<P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    GetFuture::new(path, name)
}

/// Future returned by `get`.
#[derive(Debug)]
pub struct GetFuture<P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    path: P,
    name: N,
}

impl<P, N> GetFuture<P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    fn new(path: P, name: N) -> GetFuture<P, N> {
        GetFuture {
            path: path,
            name: name,
        }
    }
}

impl<P, N> Future for GetFuture<P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    type Item = Option<Vec<u8>>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, io::Error> {
        blocking_io(|| Ok(blocking_xattr::get(&self.path, &self.name)?))
    }
}

/// List extended attributes attached to the specified file.
///
/// # Examples
///
/// ```
/// # extern crate futures;
/// # extern crate tokio;
/// # extern crate tokio_xattr;
/// use futures::Future;
///
/// fn main() {
///     let fut = tokio_xattr::list(".")
///         .map(|xattrs| {
///             for attr in xattrs {
///                 println!("attr_name={:?}", attr);
///             }
///         })
///         .map_err(|err| {
///             eprintln!("Error: {:?}", err);
///             ()
///         });
///     tokio::run(fut);
/// }
/// ```
pub fn list<P>(path: P) -> ListFuture<P>
where
    P: AsRef<Path> + Send,
{
    ListFuture::new(path)
}

/// Future returned by `list`.
#[derive(Debug)]
pub struct ListFuture<P>
where
    P: AsRef<Path> + Send,
{
    path: P,
}

impl<P> ListFuture<P>
where
    P: AsRef<Path> + Send,
{
    fn new(path: P) -> ListFuture<P> {
        ListFuture { path: path }
    }
}

impl<P> Future for ListFuture<P>
where
    P: AsRef<Path> + Send,
{
    type Item = blocking_xattr::XAttrs;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, io::Error> {
        blocking_io(|| Ok(blocking_xattr::list(&self.path)?))
    }
}

/// Remove an extended attribute from the specified file.
///
/// # Examples
///
/// ```
/// # extern crate futures;
/// # extern crate tokio;
/// # extern crate tokio_xattr;
/// use futures::Future;
///
/// fn main() {
///     let fut = tokio_xattr::remove(".", "user.myattr1").map_err(|err| {
///         eprintln!("Error: {:?}", err);
///         ()
///     });
///     tokio::run(fut);
/// }
/// ```
pub fn remove<P, N>(path: P, name: N) -> RemoveFuture<P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    RemoveFuture::new(path, name)
}

/// Future returned by `remove`.
#[derive(Debug)]
pub struct RemoveFuture<P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    path: P,
    name: N,
}

impl<P, N> RemoveFuture<P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    fn new(path: P, name: N) -> RemoveFuture<P, N> {
        RemoveFuture {
            path: path,
            name: name,
        }
    }
}

impl<P, N> Future for RemoveFuture<P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, io::Error> {
        blocking_io(|| Ok(blocking_xattr::remove(&self.path, &self.name)?))
    }
}

/// Set an extended attribute on the specified file.
///
/// # Examples
///
/// ```
/// # extern crate futures;
/// # extern crate tokio;
/// # extern crate tokio_xattr;
/// use futures::Future;
///
/// fn main() {
///     let fut = tokio_xattr::set(".", "user.myattr1", &[0x12, 0x34, 0x56]).map_err(|err| {
///         eprintln!("Error: {:?}", err);
///         ()
///     });
///     tokio::run(fut);
/// }
/// ```
pub fn set<P, N>(path: P, name: N, value: &[u8]) -> SetFuture<P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    SetFuture::new(path, name, value)
}

/// Future returned by `set`.
#[derive(Debug)]
pub struct SetFuture<'a, P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    path: P,
    name: N,
    value: &'a [u8],
}

impl<'a, P, N> SetFuture<'a, P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    fn new(path: P, name: N, value: &'a [u8]) -> SetFuture<P, N> {
        SetFuture {
            path: path,
            name: name,
            value: value,
        }
    }
}

impl<'a, P, N> Future for SetFuture<'a, P, N>
where
    P: AsRef<Path> + Send,
    N: AsRef<OsStr> + Send,
{
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, io::Error> {
        blocking_io(|| Ok(blocking_xattr::set(&self.path, &self.name, &self.value)?))
    }
}

#[cfg(test)]
mod tests {
    use futures::Future;
    use std::ffi::OsString;

    #[test]
    fn xattr_set_get_remove() {
        let path = ".";
        let name = "user.myattr1";
        let value = &[0x12, 0x34];
        let fut = super::set(path, name, value)
            .and_then(move |_| super::get(path, name))
            .map(|val| {
                assert_eq!(val, Some(vec![0x12, 0x34]));
            })
            .and_then(move |_| super::list(path))
            .map(move |mut xattrs| {
                assert_eq!(xattrs.next(), Some(OsString::from(name)));
                assert_eq!(xattrs.next(), None);
            })
            .and_then(move |_| super::remove(path, name))
            .and_then(move |_| super::get(path, name))
            .map(|val| {
                assert_eq!(val, None);
            })
            .map_err(|err| {
                eprintln!("Error: {:?}", err);
                ()
            });
        tokio::run(fut);
    }
}
