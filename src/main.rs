mod trs;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::panic;
use std::process;

use std::io::prelude::*;

mod rules;

use csv::Writer;
use crate::trs::ResultStructure;
use ordered_float::NotNan;
use num_traits::ToPrimitive;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut wtr = Writer::from_path("results/results_rules_egg.csv")?;
    for result in rdr.records() {
        let record = result?;
        // println!("{:?}", &record[1]);
        let index: i16 = record[0].parse::<i16>().unwrap();
        let start = &record[2];
        let end = &record[3];
        let condition = &record[4];
        println!("{:?}", index);
        panic::set_hook(Box::new(|_info| {
            // do nothing
            println!("{:?}", _info);
        }));
        let result = panic::catch_unwind(|| -> ResultStructure {
            println!("Simplifying expression:\n {}\n", start);
            let result_record = trs::prove_for_csv(index, start, end, condition);
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


fn run_expressions() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let params = (get_runner_iter_limit().unwrap(), get_runner_node_limit().unwrap(), get_runner_time_limit().unwrap());
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut wtr = Writer::from_path("results/results_expressions_egg.csv")?;
    for result in rdr.records() {
        let record = result?;
        // println!("{:?}", &record[1]);
        let index: i16 = record[0].parse::<i16>().unwrap();
        let start = &record[1];
        // let end = &record[3];
        // let condition = &record[4];
        // println!("{:?}", index);
        panic::set_hook(Box::new(|_info| {
            // do nothing
            println!("{:?}", _info);
        }));
        let result = panic::catch_unwind(|| -> ResultStructure {
            println!("Simplifying expression:\n {}\n", start);
            let result_record = trs::prove_exprs_for_csv(index, start, params);
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

fn get_runner_iter_limit() -> Result<usize, Box<dyn Error>>{
    match env::args_os().nth(2) {
        None => Ok(10),
        Some(i) => Ok(i.into_string().unwrap().parse::<usize>().unwrap()),
    }
}

fn get_runner_node_limit() -> Result<usize, Box<dyn Error>>{
    match env::args_os().nth(3) {
        None => Ok(10000),
        Some(i) => Ok(i.into_string().unwrap().parse::<usize>().unwrap()),
    }
}

fn get_runner_time_limit() -> Result<u64, Box<dyn Error>>{
    match env::args_os().nth(4) {
        None => Ok(5),
        Some(i) => Ok(i.into_string().unwrap().parse::<u64>().unwrap()),
    }
}

fn get_start_end() -> Result<(String, String), Box<dyn Error>>{
    let mut file = File::open("./tmp/exprs.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let v: Vec<&str> = s.split("\n").collect();
    return  Ok((v[0].to_string(), v[1].to_string()));
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if let Err(err) = run_expressions() {
            println!("{}", err);
            process::exit(1);
        }
    } else {
        let (start, end) = get_start_end().unwrap();
        println!("Simplifying expression:\n {}\n", start);
        // trs::prove_report(&start, &end, -2);
        // trs::prove_report(&start, &end, 1);
        // trs::prove_report(&start, &end, 2);
        //trs::prove_report_all_classes(&start, &end, 1);
        trs::prove_report(&start, &end, 2);
    }
}
