use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::{env, usize};

use crate::structs::ExpressionStruct;
use crate::structs::Rule;

#[allow(dead_code)]
pub fn read_expressions(file_path: &OsString) -> Result<Vec<ExpressionStruct>, Box<dyn Error>> {
    let mut expressions_vect = Vec::new();
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let index: i16 = record[0].parse::<i16>().unwrap();
        let expression = &record[1];
        let halide_result = if record[2].parse::<i16>().unwrap() == 1 {
            true
        } else {
            false
        };
        let halide_time = record[3].parse::<f64>().unwrap();
        expressions_vect.push(ExpressionStruct::new(
            index,
            expression.to_string(),
            halide_result,
            halide_time,
        ))
    }
    return Ok(expressions_vect);
}

#[allow(dead_code)]
pub fn read_expressions_paper(
    file_path: &OsString,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut expressions_vect = Vec::new();
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let infix = record[0].to_string();
        let prefix = record[1].to_string();
        expressions_vect.push((infix, prefix))
    }
    return Ok(expressions_vect);
}

#[allow(dead_code)]
pub fn read_rules(file_path: &OsString) -> Result<Vec<Rule>, Box<dyn Error>> {
    let mut rules_vect: Vec<Rule> = Vec::new();
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let index: i16 = record[0].parse::<i16>().unwrap();
        let lhs = (&record[2]).to_string();
        let rhs = (&record[3]).to_string();
        let condition = (&record[4]).to_string();
        rules_vect.push(Rule::new(index, lhs, rhs, Some(condition)))
    }
    return Ok(rules_vect);
}

pub fn get_nth_arg(n: usize) -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(n) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

pub fn get_runner_params(start: usize) -> Result<(usize, usize, f64), Box<dyn Error>> {
    let iter = match env::args_os().nth(start) {
        None => 30,
        Some(i) => i.into_string().unwrap().parse::<usize>().unwrap(),
    };

    let nodes = match env::args_os().nth(start + 1) {
        None => 10000,
        Some(i) => i.into_string().unwrap().parse::<usize>().unwrap(),
    };
    let time = match env::args_os().nth(start + 2) {
        None => 3.0,
        Some(i) => i.into_string().unwrap().parse::<f64>().unwrap(),
    };

    return Ok((iter, nodes, time));
}

#[allow(dead_code)]
pub fn get_runner_iter_limit() -> Result<usize, Box<dyn Error>> {
    match env::args_os().nth(2) {
        None => Ok(30),
        Some(i) => Ok(i.into_string().unwrap().parse::<usize>().unwrap()),
    }
}

#[allow(dead_code)]
pub fn get_runner_node_limit() -> Result<usize, Box<dyn Error>> {
    match env::args_os().nth(3) {
        None => Ok(10000),
        Some(i) => Ok(i.into_string().unwrap().parse::<usize>().unwrap()),
    }
}

#[allow(dead_code)]
pub fn get_runner_time_limit() -> Result<u64, Box<dyn Error>> {
    match env::args_os().nth(4) {
        None => Ok(5),
        Some(i) => Ok(i.into_string().unwrap().parse::<u64>().unwrap()),
    }
}

pub fn get_start_end() -> Result<(String, String), Box<dyn Error>> {
    let mut file = File::open("./tmp/exprs.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let v: Vec<&str> = s.split("\n").collect();
    return Ok((v[0].to_string(), v[1].to_string()));
}
