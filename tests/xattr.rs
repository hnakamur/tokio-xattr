#![deny(warnings, rust_2018_idioms)]
#![feature(async_await)]

mod pool;

use std::ffi::OsString;

#[test]
fn xattr_set_get_remove() {
    let path = ".";
    let name = "user.myattr1";
    let value = &[0x12, 0x34];
    pool::run(async move {
        tokio_xattr::set(path, name, value).await?;
        let val = tokio_xattr::get(path, name).await?;
        assert_eq!(val, Some(vec![0x12, 0x34]));
        let mut xattrs = tokio_xattr::list(path).await?;
        assert_eq!(xattrs.next(), Some(OsString::from(name)));
        assert_eq!(xattrs.next(), None);
        tokio_xattr::remove(path, name).await?;
        let val2 = tokio_xattr::get(path, name).await?;
        assert_eq!(val2, None);
        Ok(())
    });
}
