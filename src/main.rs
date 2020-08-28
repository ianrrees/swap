// Linux command line tool for swapping the names of two files.
// (thin wrapper around the renameat2 syscall)
//
// Ian Rees 2020

use clap;
use libc::{self, c_char, c_int, c_uint, AT_FDCWD, RENAME_EXCHANGE};
use std::io;

// 28 Aug 2020: renameat2() is not in the Rust libc crate, but
// is added by an open PR so this can probably be removed.
//
// From the man page:
// int renameat2(int olddirfd, const char *oldpath,
//               int newdirfd, const char *newpath, unsigned int flags);
extern "C" {
    pub fn renameat2(
        olddirfd: c_int,
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
        flags: c_uint,
    ) -> c_int;
}

fn main() {
    let args = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Swap a pair of files")
        .arg(
            clap::Arg::with_name("file_a")
                .help("First file to swap")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("file_b")
                .help("Second file to swap")
                .required(true),
        )
        .get_matches();

    std::process::exit(
        match unsafe {
            renameat2(
                AT_FDCWD,
                args.value_of("file_a").unwrap().as_ptr() as *const c_char,
                AT_FDCWD,
                args.value_of("file_b").unwrap().as_ptr() as *const c_char,
                RENAME_EXCHANGE as u32,
            )
        } {
            0 => 0,
            -1 => {
                eprintln!(
                    "{} failed: {}",
                    clap::crate_name!(),
                    io::Error::last_os_error()
                );
                1
            }
            x => {
                eprintln!("renameat2 returned {}!?", x);
                1
            }
        },
    );
}
