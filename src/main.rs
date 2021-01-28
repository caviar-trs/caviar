mod trs;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::panic;
use std::process;
mod rules;
use csv::Writer;
use crate::trs::ResultStructure;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut wtr = Writer::from_path("results_rules_egg.csv")?;
    for result in rdr.records() {
        let record = result?;
        // println!("{:?}", &record[1]);
        let index: i16 = record[0].parse::<i16>().unwrap();
        let start = &record[2];
        let end = &record[3];
        println!("{:?}", index);
        panic::set_hook(Box::new(|_info| {
            // do nothing
            println!("{:?}", _info);
        }));
        let result = panic::catch_unwind(||-> ResultStructure {
            println!("Simplifying expression:\n {}\n", start);
            let result_record = trs::prove_for_csv(index, start, end);
            result_record
        });

        match result {
            Ok(res) => wtr.serialize(res)?,
            Err(_) => println!("Error at expression: {}", start),
        }
    }
    wtr.flush();
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
    // let args: Vec<String> = env::args().collect();
    // let mut start = "( + x x )";
    // let mut end = "( * 2 x )";
    // if args.len() > 1 {
    //     start = &args[1][..];
    //     if args.len() > 2 {
    //         end = &args[2][..];
    //     }
    // }
    // println!("Simplifying expression:\n {}\n", start);
    // trs::prove_time(start, end);




    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
