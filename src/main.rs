mod trs;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::panic;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let start = record.as_slice();
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        let result = panic::catch_unwind(|| {
            println!("Simplifying expression:\n {}\n", start);
            trs::prove_time(start, "1");
        });

        match result {
            Ok(res) => res,
            Err(_) => println!("Error at expression: {}!", start),
        }
    }
    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut start = "(<= ( + ( / ( - v0 v1 ) 8 ) 96 ) ( max ( / ( + ( - v0 v1 ) 769 ) 8 ) 0 ))";
    let mut end = "1";
    if args.len() > 1 {
        start = &args[1][..];
        if args.len() > 2 {
            end = &args[2][..];
        }
    }
    println!("Simplifying expression:\n {}\n", start);
    trs::prove_time(start, end);

    // if let Err(err) = run() {
    //     println!("{}", err);
    //     process::exit(1);
    // }
}
