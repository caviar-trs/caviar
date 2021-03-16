use crate::structs::{ExpressionStruct};
use std::error::Error;
use std::fs::File;
use std::{env};
use std::ffi::OsString;
use std::io::Read;


pub fn read_expressions(file_path:OsString) -> Result<Vec<ExpressionStruct>, Box<dyn Error>> {
    let mut expressions_vect  = Vec::new();
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let index: i16 = record[0].parse::<i16>().unwrap();
        let expression = &record[1];
        expressions_vect.push(ExpressionStruct::new(index, expression.to_string()))
    }
    return Ok(expressions_vect)
}




pub fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

pub fn get_runner_iter_limit() -> Result<usize, Box<dyn Error>>{
    match env::args_os().nth(2) {
        None => Ok(30),
        Some(i) => Ok(i.into_string().unwrap().parse::<usize>().unwrap()),
    }
}

pub fn get_runner_node_limit() -> Result<usize, Box<dyn Error>>{
    match env::args_os().nth(3) {
        None => Ok(10000),
        Some(i) => Ok(i.into_string().unwrap().parse::<usize>().unwrap()),
    }
}

pub fn get_runner_time_limit() -> Result<u64, Box<dyn Error>>{
    match env::args_os().nth(4) {
        None => Ok(5),
        Some(i) => Ok(i.into_string().unwrap().parse::<u64>().unwrap()),
    }
}

pub fn get_start_end() -> Result<(String, String), Box<dyn Error>>{
    let mut file = File::open("./tmp/exprs.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let v: Vec<&str> = s.split("\n").collect();
    return  Ok((v[0].to_string(), v[1].to_string()));
}
