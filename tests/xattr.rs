#![warn(rust_2018_idioms)]

use std::ffi::OsString;
use tempfile::tempdir;
use tokio_test::assert_ok;

#[tokio::test]
async fn xattr_set_get_remove() {
    let temp_dir = tempdir().unwrap();
    let path = temp_dir.path();
    let name = "user.myattr1";
    let value = &[0x12, 0x34];

    assert_ok!(tokio_xattr::set(path.to_owned(), name, value).await);
    let val = tokio_xattr::get(path.to_owned(), name).await.unwrap();
    assert_eq!(val, Some(vec![0x12, 0x34]));

    let mut xattrs = tokio_xattr::list(path.to_owned()).await.unwrap();
    assert_eq!(xattrs.next(), Some(OsString::from(name)));
    assert_eq!(xattrs.next(), None);

    assert_ok!(tokio_xattr::remove(path.to_owned(), name).await);
    let val2 = tokio_xattr::get(path.to_owned(), name).await.unwrap();
    assert_eq!(val2, None);
}
