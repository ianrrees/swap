// Linux command line tool for swapping the names of two files.
// (thin wrapper around the renameat2 syscall)
//
// Ian Rees 2020

use clap;
use libc::{self, AT_FDCWD, c_int, c_char, c_uint, RENAME_EXCHANGE};
use std::io;
use std::path::Path;
use std::os::unix::ffi::OsStrExt; // For Path.as_os_str()

// 28 Aug 2020: renameat2() is not in the Rust libc crate, but is added by an open PR.
// From the man page:
// int renameat2(int olddirfd, const char *oldpath,
//               int newdirfd, const char *newpath, unsigned int flags);
extern "C" {
    pub fn renameat2(olddirfd: c_int, oldpath: *const c_char,
        newdirfd: c_int, newpath: *const c_char, flags: c_uint) -> c_int;
}

fn swap(a: &Path, b: &Path) -> io::Result<()> {
    match unsafe { renameat2(AT_FDCWD, a.as_os_str().as_bytes().as_ptr() as *const c_char,
                             AT_FDCWD, b.as_os_str().as_bytes().as_ptr() as *const c_char,
                             RENAME_EXCHANGE as u32)} {
        0 => Ok(()),
        -1 => Err(io::Error::last_os_error()),
        x => Err(io::Error::new(io::ErrorKind::Other, format!("renameat2 returned {}!?", x))),
    }
}

fn main() {
    let args = clap::App::new(clap::crate_name!())
                         .version(clap::crate_version!())
                         .author(clap::crate_authors!())
                         .about("Swap a pair of files")
                         .arg(clap::Arg::with_name("file_a")
                             .help("First file to swap")
                             .required(true))
                         .arg(clap::Arg::with_name("file_b")
                             .help("Second file to swap")
                             .required(true))
                         .get_matches();

    let file_a = args.value_of("file_a").unwrap();
    let file_b = args.value_of("file_b").unwrap();

    match swap(Path::new(file_a), Path::new(file_b)) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed: {}", clap::crate_name!(), e);
            std::process::exit(1);
        }
    }
}
