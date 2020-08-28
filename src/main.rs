// Linux command line tool for swapping two files.
// (thin wrapper around the renameat2 syscall)
//
// Ian Rees 2020

extern crate clap;

fn main() {
    let matches = clap::App::new(clap::crate_name!())
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

    let file_a_name = matches.value_of("file_a").unwrap();
    let file_b_name = matches.value_of("file_b").unwrap();

    println!("{:?} and {:?}", file_a_name, file_b_name);
}
