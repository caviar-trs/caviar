use std::{env, ffi::OsString, fs::File, io::Read, time::Instant};

use io::reader::get_nth_arg;
use json::parse;

use crate::io::reader::{get_runner_params, get_start_end, read_expressions};
use crate::io::writer::write_results;
use crate::structs::{ExpressionStruct, ResultStructure};
use trs::{prove, prove_expression_with_file_classes};
mod trs;

mod dataset;
mod io;
mod rules;
mod structs;

#[allow(dead_code)]
fn prove_expressions(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<ResultStructure> {
    let mut results = Vec::new();
    for expression in exprs_vect.iter() {
        results.push(prove(
            &expression.expression,
            ruleset_class,
            params,
            use_iteration_check,
            report,
        ));
    }
    results
}

fn test_classes(
    path: OsString,
    exprs_vect: &Vec<ExpressionStruct>,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> () {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let classes = parse(&s).unwrap();
    let mut results_structs = Vec::new();
    let mut results_proving_class = Vec::new();
    let mut results_exec_time = Vec::new();
    let start_t = Instant::now();

    for expression in exprs_vect.iter() {
        println!("Starting Expression: {}", expression.index);
        let (strct, class, exec_time) = prove_expression_with_file_classes(
            &classes,
            params,
            expression.index,
            &expression.expression.clone(),
            use_iteration_check,
            report,
        )
        .unwrap();
        results_structs.push(strct);
        results_proving_class.push(class);
        results_exec_time.push(exec_time);
    }
    let duration = start_t.elapsed().as_secs();
    let exec_time: f64 = results_exec_time.iter().map(|i| i.as_secs() as f64).sum();
    println!("Execution time : |{}| |{}|", duration, exec_time);
    write_results(
        &format!(
            "results/k_{}_class_analysis_results_params_{}_{}_{}_exec_{}.csv",
            classes[0].len(),
            params.0,
            params.1,
            params.2,
            duration
        ),
        &results_structs,
    )
    .unwrap();
}

fn main() {
    let _args: Vec<String> = env::args().collect();
    if _args.len() > 4 {
        let operation = get_nth_arg(1).unwrap();
        let expressions_file = get_nth_arg(2).unwrap();
        let params = get_runner_params(3).unwrap();
        match operation.to_str().unwrap() {
            "dataset" => {
                // cargo run --release dataset ./results/expressions_egg.csv 1000000 10000000 5 5 3 0 4
                let reorder_count = get_nth_arg(6).unwrap().into_string().unwrap().parse::<usize>().unwrap();
                let batch_size = get_nth_arg(7).unwrap().into_string().unwrap().parse::<usize>().unwrap();
                let continue_from_expr = get_nth_arg(8).unwrap().into_string().unwrap().parse::<usize>().unwrap();
                let cores = get_nth_arg(9).unwrap().into_string().unwrap().parse::<usize>().unwrap();
                rayon::ThreadPoolBuilder::new().num_threads(cores).build_global().unwrap();
                dataset::generation_execution(&expressions_file, params, reorder_count, batch_size, continue_from_expr);
            }
            "prove_exprs" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions(&expression_vect, -1, params, true, true);
                write_results("tmp/generated_expressions_results.csv", &results).unwrap();
            }
            "test_classes" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let classes_file = get_nth_arg(6).unwrap();
                test_classes(classes_file, &expression_vect, params, true, false);
            }
            "prove_one_expr" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let classes_file = get_nth_arg(6).unwrap();
                let mut file = File::open(classes_file).unwrap();
                let mut s = String::new();
                file.read_to_string(&mut s).unwrap();
                let classes = parse(&s).unwrap();
                let start_t = Instant::now();

                let (strct, class, exec_time) = prove_expression_with_file_classes(
                    &classes,
                    params,
                    expression_vect[0].index,
                    &expression_vect[0].expression.clone(),
                    true,
                    true,
                )
                .unwrap();
                println!("{}", start_t.elapsed().as_secs_f64());
            }
            _ => {}
        }
    } else {
        let params = get_runner_params(1).unwrap();
        let (start, end) = get_start_end().unwrap();
        println!("Simplifying expression:\n {}\n to {}", start, end);
        println!("{:?}", prove(&start, -1, params, true, true));
    }
}
