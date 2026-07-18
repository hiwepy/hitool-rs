//! file_util parity tests
//! 对齐: hutool-core FileUtilTest

use hitool_core::FileUtil;
use std::path::Path;

// ── 路径操作 ──

#[test]
fn name_basic() {
    assert_eq!(FileUtil::name(Path::new("/path/to/file.txt")), "file.txt");
    assert_eq!(FileUtil::name(Path::new("file.txt")), "file.txt");
}

#[test]
fn suffix_basic() {
    assert_eq!(FileUtil::suffix(Path::new("/path/to/file.txt")), "txt");
    assert_eq!(FileUtil::suffix(Path::new("file.rs")), "rs");
}

#[test]
fn main_name_basic() {
    assert_eq!(FileUtil::main_name(Path::new("/path/to/file.txt")), "file");
    assert_eq!(FileUtil::main_name(Path::new("archive.tar.gz")), "archive.tar");
}

#[test]
fn name_from_str_basic() {
    assert_eq!(FileUtil::name_from_str("/path/to/file.txt"), "file.txt");
}

#[test]
fn suffix_from_str_basic() {
    assert_eq!(FileUtil::suffix_from_str("/path/to/file.txt"), "txt");
}

// ── 路径构建 ──

#[test]
fn join_paths_basic() {
    let path = FileUtil::join_paths(&["/home", "user", "file.txt"]);
    assert_eq!(path, Path::new("/home/user/file.txt"));
}

#[test]
fn file_basic() {
    let path = FileUtil::file(&["/tmp", "test.txt"]);
    assert_eq!(path, Path::new("/tmp/test.txt"));
}

// ── 文件判断 ──

#[test]
fn exists_tmp_dir() {
    assert!(FileUtil::exists("/tmp"));
}

#[test]
fn exists_nonexistent() {
    assert!(!FileUtil::exists("/nonexistent/path/that/does/not/exist"));
}

#[test]
fn is_directory_tmp() {
    assert!(FileUtil::is_directory("/tmp"));
}

#[test]
fn is_file_tmp() {
    assert!(!FileUtil::is_file("/tmp"));
}

// ── 文件读写 ──

#[test]
fn read_write_utf8_string() {
    let path = "/tmp/test_file_util_utf8.txt";
    let content = "Hello, World!\n你好世界";
    FileUtil::write_utf8_string(path, content).unwrap();
    let read = FileUtil::read_utf8_string(path).unwrap();
    assert_eq!(read, content);
    FileUtil::delete(path).unwrap();
}

#[test]
fn read_write_bytes() {
    let path = "/tmp/test_file_util_bytes.bin";
    let content = b"Hello, Binary!";
    FileUtil::write_bytes(path, content).unwrap();
    let read = FileUtil::read_bytes(path).unwrap();
    assert_eq!(read, content);
    FileUtil::delete(path).unwrap();
}

// ── 文件操作 ──

#[test]
fn copy_basic() {
    let from = "/tmp/test_file_util_copy_src.txt";
    let to = "/tmp/test_file_util_copy_dst.txt";
    FileUtil::write_utf8_string(from, "copy test").unwrap();
    FileUtil::copy(from, to).unwrap();
    let read = FileUtil::read_utf8_string(to).unwrap();
    assert_eq!(read, "copy test");
    FileUtil::delete(from).unwrap();
    FileUtil::delete(to).unwrap();
}

#[test]
fn mkdir_and_delete() {
    let path = "/tmp/test_file_util_dir";
    FileUtil::mkdir(path).unwrap();
    assert!(FileUtil::is_directory(path));
    FileUtil::delete(path).unwrap();
    assert!(!FileUtil::exists(path));
}

#[test]
fn rename_basic() {
    let from = "/tmp/test_file_util_rename_from.txt";
    let to = "/tmp/test_file_util_rename_to.txt";
    FileUtil::write_utf8_string(from, "rename test").unwrap();
    FileUtil::rename(from, to).unwrap();
    assert!(!FileUtil::exists(from));
    assert!(FileUtil::exists(to));
    FileUtil::delete(to).unwrap();
}

// ── 文件列表 ──

#[test]
fn list_file_names_basic() {
    let dir = "/tmp/test_file_util_list";
    FileUtil::mkdir(dir).unwrap();
    FileUtil::write_utf8_string(&format!("{}/a.txt", dir), "a").unwrap();
    FileUtil::write_utf8_string(&format!("{}/b.txt", dir), "b").unwrap();
    let names = FileUtil::list_file_names(dir).unwrap();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"a.txt".to_string()));
    assert!(names.contains(&"b.txt".to_string()));
    FileUtil::delete(dir).unwrap();
}

// ── 文件大小 ──

#[test]
fn size_basic() {
    let path = "/tmp/test_file_util_size.txt";
    FileUtil::write_utf8_string(path, "Hello").unwrap();
    let size = FileUtil::size(Path::new(path));
    assert_eq!(size, 5);
    FileUtil::delete(path).unwrap();
}

// ── 临时目录 ──

#[test]
fn tmp_dir_basic() {
    assert_eq!(FileUtil::tmp_dir(), "/tmp");
}
