use std::{env, ffi::OsString, fs::File, io::Read, time::Instant};

use io::reader::get_nth_arg;
use json::{parse, JsonValue};
use serde::de::Expected;
use std::process;

use crate::io::reader::{get_runner_params, get_start_end, read_expressions};
use crate::io::writer::write_results;
use crate::structs::{ExpressionStruct, ResultStructure};
use crate::trs::{filtered_rules, prove_expr, prove_rule, prove_expression_with_file_classes};

mod trs;

mod dataset;
mod io;
mod rules;
mod structs;

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // let rules = io::reader::read_rules(&get_nth_arg(1).unwrap()).unwrap();
        // let mut results: Vec<ResultStructure> = Vec::new();
        // for rule in rules{
        //     println!("{}", rule.index);
        //     results.push(trs::prove_rule(&rule, -2, (100, 10000, 5), true, true));
        // }
        // write_results("./results/results_rules_egg.csv", &results).unwrap()
        let exprs = io::reader::read_expressions(&get_nth_arg(1).unwrap()).unwrap();
        let mut results: Vec<ResultStructure> = Vec::new();
        for expr in exprs{
            results.push(trs::prove_expr(&expr, -2, (1000, 100000, 1), true, false))
        }
        write_results("./results/results_first_dataset.csv",&results).unwrap()
    } else {
        let (start, end) = get_start_end().unwrap();
        println!("Simplifying expression:\n {}\n", start);
        trs::prove_equiv(&start, &end, -2, (100, 10000, 5), true, true);
    }
}
