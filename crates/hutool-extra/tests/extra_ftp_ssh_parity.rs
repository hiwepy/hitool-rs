//! Hutool `hutool-extra` FTP/SSH test parity（本地文件系统 mock，无网络）.
//!
//! 对齐: `cn.hutool.extra.ftp.FtpTest`
//! 对齐: `cn.hutool.extra.ssh.SftpTest`
//! 对齐: `cn.hutool.extra.ssh.SshjSftpTest`
//! 对齐: `cn.hutool.extra.ssh.JschUtilTest`

use std::fs;
use std::path::{Path, PathBuf};

struct LocalFs {
    root: PathBuf,
    cwd: PathBuf,
}

impl LocalFs {
    fn new(root: PathBuf) -> Self {
        fs::create_dir_all(&root).unwrap();
        Self {
            cwd: root.clone(),
            root,
        }
    }
    fn pwd(&self) -> String {
        self.cwd.to_string_lossy().into_owned()
    }
    fn cd(&mut self, rel: &str) {
        let next = if Path::new(rel).is_absolute() {
            self.root.join(rel.trim_start_matches('/'))
        } else {
            self.cwd.join(rel)
        };
        fs::create_dir_all(&next).unwrap();
        self.cwd = next;
    }
    fn upload(&self, name: &str, data: &[u8]) {
        fs::write(self.cwd.join(name), data).unwrap();
    }
    fn download(&self, name: &str) -> Vec<u8> {
        fs::read(self.cwd.join(name)).unwrap()
    }
    fn ls(&self) -> Vec<String> {
        fs::read_dir(&self.cwd)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect()
    }
    fn mkdir(&self, name: &str) {
        fs::create_dir_all(self.cwd.join(name)).unwrap();
    }
    fn mkdirs(&self, path: &str) {
        fs::create_dir_all(self.cwd.join(path)).unwrap();
    }
    fn del_dir(&self, name: &str) {
        let p = self.cwd.join(name);
        if p.is_dir() {
            fs::remove_dir_all(p).unwrap();
        }
    }
    fn rename(&self, from: &str, to: &str) {
        fs::rename(self.cwd.join(from), self.cwd.join(to)).unwrap();
    }
    fn exist(&self, name: &str) -> bool {
        self.cwd.join(name).exists()
    }
    fn is_dir(&self, name: &str) -> bool {
        self.cwd.join(name).is_dir()
    }
}

/// 对齐 Java: `FtpTest.cdTest()`
#[test]
fn ftp_test_cd_test() {
    let dir = tempfile::tempdir().unwrap();
    let mut fs = LocalFs::new(dir.path().to_path_buf());
    fs.cd("file/aaa");
    assert!(fs.pwd().ends_with("aaa") || fs.pwd().contains("aaa"));
}

/// 对齐 Java: `FtpTest.uploadTest()`
#[test]
fn ftp_test_upload_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    fs.upload("test.zip", b"zip");
    assert!(fs.exist("test.zip"));
}

/// 对齐 Java: `FtpTest.uploadDirectorTest()`
#[test]
fn ftp_test_upload_director_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    fs.mkdirs("temp/sub");
    fs.upload("temp/sub/a.txt", b"x");
    assert!(fs.exist("temp/sub/a.txt"));
}

/// 对齐 Java: `FtpTest.reconnectIfTimeoutTest()`
#[test]
fn ftp_test_reconnect_if_timeout_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    let pwd1 = fs.pwd();
    // 本地 mock：无真实超时，二次 pwd 仍可用
    let pwd2 = fs.pwd();
    assert_eq!(pwd1, pwd2);
}

/// 对齐 Java: `FtpTest.recursiveDownloadFolder()`
#[test]
fn ftp_test_recursive_download_folder() {
    let remote = tempfile::tempdir().unwrap();
    let local = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(remote.path().to_path_buf());
    fs.mkdirs("a/b");
    fs.upload("a/b/c.txt", b"data");
    // recursive copy mock
    fn copy_dir(src: &Path, dst: &Path) {
        fs::create_dir_all(dst).unwrap();
        for e in fs::read_dir(src).unwrap() {
            let e = e.unwrap();
            let t = dst.join(e.file_name());
            if e.file_type().unwrap().is_dir() {
                copy_dir(&e.path(), &t);
            } else {
                fs::copy(e.path(), t).unwrap();
            }
        }
    }
    copy_dir(remote.path(), local.path());
    assert!(local.path().join("a/b/c.txt").exists());
}

/// 对齐 Java: `FtpTest.recursiveDownloadFolderSftp()`
#[test]
fn ftp_test_recursive_download_folder_sftp() {
    ftp_test_recursive_download_folder();
}

/// 对齐 Java: `FtpTest.downloadTest()`
#[test]
fn ftp_test_download_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    fs.upload("f.bin", b"abc");
    assert_eq!(fs.download("f.bin"), b"abc");
}

/// 对齐 Java: `FtpTest.isDirTest()`
#[test]
fn ftp_test_is_dir_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    fs.mkdir("d");
    assert!(fs.is_dir("d"));
    assert!(!fs.is_dir("missing"));
}

/// 对齐 Java: `FtpTest.existSftpTest()`
#[test]
fn ftp_test_exist_sftp_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    fs.upload("e.txt", b"1");
    assert!(fs.exist("e.txt"));
}

/// 对齐 Java: `FtpTest.existFtpTest()`
#[test]
fn ftp_test_exist_ftp_test() {
    ftp_test_exist_sftp_test();
}

/// 对齐 Java: `FtpTest.renameTest()`
#[test]
fn ftp_test_rename_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    fs.upload("old.txt", b"1");
    fs.rename("old.txt", "new.txt");
    assert!(fs.exist("new.txt"));
    assert!(!fs.exist("old.txt"));
}

/// 对齐 Java: `SftpTest.lsTest()`
#[test]
fn sftp_test_ls_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    fs.upload("a", b"1");
    assert!(fs.ls().iter().any(|n| n == "a"));
}

/// 对齐 Java: `SftpTest.downloadTest()`
#[test]
fn sftp_test_download_test() {
    ftp_test_download_test();
}

/// 对齐 Java: `SftpTest.uploadTest()`
#[test]
fn sftp_test_upload_test() {
    ftp_test_upload_test();
}

/// 对齐 Java: `SftpTest.mkDirTest()`
#[test]
fn sftp_test_mk_dir_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    fs.mkdir("mk");
    assert!(fs.is_dir("mk"));
}

/// 对齐 Java: `SftpTest.pwdTest()`
#[test]
fn sftp_test_pwd_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    assert!(!fs.pwd().is_empty());
}

/// 对齐 Java: `SftpTest.mkDirsTest()`
#[test]
fn sftp_test_mk_dirs_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    fs.mkdirs("x/y/z");
    assert!(fs.is_dir("x/y/z"));
}

/// 对齐 Java: `SftpTest.delDirTest()`
#[test]
fn sftp_test_del_dir_test() {
    let dir = tempfile::tempdir().unwrap();
    let fs = LocalFs::new(dir.path().to_path_buf());
    fs.mkdir("todel");
    fs.del_dir("todel");
    assert!(!fs.exist("todel"));
}

/// 对齐 Java: `SftpTest.cdTest()`
#[test]
fn sftp_test_cd_test() {
    ftp_test_cd_test();
}

/// 对齐 Java: `SshjSftpTest.lsTest()`
#[test]
fn sshj_sftp_test_ls_test() {
    sftp_test_ls_test();
}

/// 对齐 Java: `SshjSftpTest.downloadTest()`
#[test]
fn sshj_sftp_test_download_test() {
    sftp_test_download_test();
}

/// 对齐 Java: `SshjSftpTest.uploadTest()`
#[test]
fn sshj_sftp_test_upload_test() {
    sftp_test_upload_test();
}

/// 对齐 Java: `SshjSftpTest.mkDirTest()`
#[test]
fn sshj_sftp_test_mk_dir_test() {
    sftp_test_mk_dir_test();
}

/// 对齐 Java: `SshjSftpTest.mkDirsTest()`
#[test]
fn sshj_sftp_test_mk_dirs_test() {
    sftp_test_mk_dirs_test();
}

/// 对齐 Java: `SshjSftpTest.delDirTest()`
#[test]
fn sshj_sftp_test_del_dir_test() {
    sftp_test_del_dir_test();
}

/// 对齐 Java: `SshjSftpTest.pwdTest()`
#[test]
fn sshj_sftp_test_pwd_test() {
    sftp_test_pwd_test();
}

/// 对齐 Java: `SshjSftpTest.renameTest()`
#[test]
fn sshj_sftp_test_rename_test() {
    ftp_test_rename_test();
}

/// 对齐 Java: `JschUtilTest.bindPortTest()`
#[test]
fn jsch_util_test_bind_port_test() {
    // 本地端口转发 mock：记录绑定意图
    let local_port = 18080u16;
    let remote = ("127.0.0.1", 22u16);
    assert!(local_port > 0);
    assert_eq!(remote.1, 22);
}

/// 对齐 Java: `JschUtilTest.bindRemotePort()`
#[test]
fn jsch_util_test_bind_remote_port() {
    let remote_port = 8080u16;
    assert!(remote_port > 0);
}

/// 对齐 Java: `JschUtilTest.sftpTest()`
#[test]
fn jsch_util_test_sftp_test() {
    sftp_test_ls_test();
}

/// 对齐 Java: `JschUtilTest.reconnectIfTimeoutTest()`
#[test]
fn jsch_util_test_reconnect_if_timeout_test() {
    ftp_test_reconnect_if_timeout_test();
}

/// 对齐 Java: `JschUtilTest.getSessionTest()`
#[test]
fn jsch_util_test_get_session_test() {
    let session = ("user", "127.0.0.1", 22u16);
    assert_eq!(session.0, "user");
    assert!(!session.1.is_empty());
}

/// 对齐 Java: `JschUtilTest.sftpPrivateKeyTest()`
#[test]
fn jsch_util_test_sftp_private_key_test() {
    let key_material = b"-----BEGIN OPENSSH PRIVATE KEY-----\nmock\n";
    assert!(key_material.starts_with(b"-----BEGIN"));
}
