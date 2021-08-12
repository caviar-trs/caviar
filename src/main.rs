use std::{env, ffi::OsString, fs::File, io::Read, time::Instant};

use io::reader::{get_nth_arg, get_runner_params, get_start_end, read_expressions};
use io::writer::write_results;
use json::parse;
use std::time::Duration;
use structs::{ExpressionStruct, ResultStructure};
use trs::{prove, prove_expression_with_file_classes, prove_npp, prove_pulses, prove_pulses_npp, prove_equiv};

use crate::io::reader::read_expressions_paper;
use crate::io::writer::write_results_paper;
use crate::structs::PaperResult;
use crate::trs::simplify;
mod trs;

mod dataset;
mod io;
mod rules;
mod structs;

/// Runs Simple Caviar to prove the expressions passed as vector using the different params passed.
#[allow(dead_code)]
fn prove_expressions(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    params: (usize, usize, f64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<ResultStructure> {
    //Initialize the results vector.
    let mut results = Vec::new();

    //For each expression try to prove it then push the results into the results vector.
    for expression in exprs_vect.iter() {
        println!("Starting Expression: {}", expression.index);
        let mut res = prove(
            expression.index,
            &expression.expression,
            ruleset_class,
            params,
            use_iteration_check,
            report,
        );
        res.add_halide(expression.halide_result.clone(), expression.halide_time);
        results.push(res);
    }
    results
}

/// Runs Caviar with Pulses on the expressions passed as vector using the different params passed.
#[allow(dead_code)]
fn prove_expressions_pulses(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    threshold: f64,
    params: (usize, usize, f64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<ResultStructure> {
    //Initialize the results vector.
    let mut results = Vec::new();
    //For each expression try to prove it using Caviar with Pulses then push the results into the results vector.
    for expression in exprs_vect.iter() {
        println!("Starting Expression: {}", expression.index);
        let mut res = prove_pulses(
            expression.index,
            &expression.expression,
            ruleset_class,
            threshold,
            params,
            use_iteration_check,
            report,
        );
        res.add_halide(expression.halide_result.clone(), expression.halide_time);
        results.push(res);
    }
    results
}

/// Runs Caviar with NPP on the expressions passed as vector using the different params passed.
#[allow(dead_code)]
fn prove_expressions_npp(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    params: (usize, usize, f64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<ResultStructure> {
    //Initialize the results vector.
    let mut results = Vec::new();

    //For each expression try to prove it using Caviar with NPP then push the results into the results vector.
    for expression in exprs_vect.iter() {
        println!("Starting Expression: {}", expression.index);
        let mut res = prove_npp(
            expression.index,
            &expression.expression,
            ruleset_class,
            params,
            use_iteration_check,
            report,
        );
        res.add_halide(expression.halide_result.clone(), expression.halide_time);
        results.push(res);
    }
    results
}

/// Runs  Caviar with Pulses and NPP on the expressions passed as vector using the different params passed.
#[allow(dead_code)]
fn prove_expressions_pulses_npp_paper(
    exprs_vect: &Vec<(String, String)>,
    ruleset_class: i8,
    threshold: f64,
    params: (usize, usize, f64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<PaperResult> {
    //Initialize the results vector.
    let mut results = Vec::new();
    // For each expression try to prove it using Caviar with Pulses and NPP then push the results into the results vector.
    for expression in exprs_vect.iter() {
        println!("Starting Expression: {}", expression.0);
        let res = prove_pulses_npp(
            -1,
            &expression.1,
            ruleset_class,
            threshold,
            params,
            use_iteration_check,
            report,
        );
        // res.add_halide(expression.halide_result, expression.halide_time);
        results.push(PaperResult::new(
            expression.0.clone(),
            expression.1.clone(),
            if res.result { 1 } else { 0 },
        ));
    }
    results
}

///Runs Caviar with Pulses and NPP on the expressions passed as vector using the different params passed.
#[allow(dead_code)]
fn prove_expressions_pulses_npp(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    threshold: f64,
    params: (usize, usize, f64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<ResultStructure> {
    //Initialize the results vector.
    let mut results = Vec::new();
    // For each expression try to prove it using Caviar with Pulses and NPP then push the results into the results vector.
    for expression in exprs_vect.iter() {
        println!("Starting Expression: {}", expression.index);
        results.push(prove_pulses_npp(
            expression.index,
            &expression.expression,
            ruleset_class,
            threshold,
            params,
            use_iteration_check,
            report,
        ));
    }
    results
}

/// Runs Caviar using hierarchical clusters of rules to prove the expressions passed as vector using the different params passed.
fn prove_clusters(
    path: OsString,
    exprs_vect: &Vec<ExpressionStruct>,
    params: (usize, usize, f64),
    count: usize,
    use_iteration_check: bool,
    report: bool,
) -> () {
    //Read the clusters from the files generated using Python.
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let classes = parse(&s).unwrap();

    //Initialization
    let mut results_structs = Vec::new();
    let mut results_proving_class = Vec::new();
    let mut results_exec_time = Vec::new();
    let start_t = Instant::now();
    let mut average;
    let mut prove_result: (ResultStructure, i64, Duration);
    let mut i;

    //For each expression try to prove it using the clusters generated one after the other.
    for expression in exprs_vect.iter() {
        if report {
            println!("Starting Expression: {}", expression.index);
        }
        i = 0;
        average = 0.0;
        loop {
            prove_result = prove_expression_with_file_classes(
                &classes,
                params,
                expression.index,
                &expression.expression.clone(),
                use_iteration_check,
                report,
            )
            .unwrap();
            if report {
                println!("Iter: {} | time: {}", i, prove_result.0.total_time);
            }
            average += prove_result.0.total_time;
            i += 1;
            if i == count || !prove_result.0.result {
                break;
            }
        }
        prove_result.0.total_time = average / (i as f64);
        results_structs.push(prove_result.0);
        results_proving_class.push(prove_result.1);
        results_exec_time.push(prove_result.2);
        if report {
            println!("Average time: {}", average / (i as f64));
        }
    }
    let duration = start_t.elapsed().as_secs();
    let exec_time: f64 = results_exec_time.iter().map(|i| i.as_secs() as f64).sum();
    if report {
        println!("Execution time : |{}| |{}|", duration, exec_time);
    }

    //Write the results into the results csv file.
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

/// Runs Simple Caviar to simplify the expressions passed as vector using the different params passed.
#[allow(dead_code)]
fn simplify_expressions(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    params: (usize, usize, f64),
    report: bool,
) -> Vec<ResultStructure> {
    //Initialize the results vector.
    let mut results = Vec::new();

    //For each expression try to prove it then push the results into the results vector.
    for expression in exprs_vect.iter() {
        println!("Starting Expression: {}", expression.index);
        let mut res = simplify(
            expression.index,
            &expression.expression,
            ruleset_class,
            params,
            report,
        );
        res.add_halide(expression.halide_result.clone(), expression.halide_time);
        results.push(res);
    }
    results
}

fn main() {
    let _args: Vec<String> = env::args().collect();
    if _args.len() > 4 {
        let operation = get_nth_arg(1).unwrap();
        let expressions_file = get_nth_arg(2).unwrap();
        let params = get_runner_params(3).unwrap();
        match operation.to_str().unwrap() {
            // Generates a dataset for minimum rulesets needed for each expression from the expressions file passed as argument
            "dataset" => {
                // cargo run --release dataset ./results/expressions_egg.csv 1000000 10000000 5 5 3 0 4
                let reorder_count = get_nth_arg(6)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let batch_size = get_nth_arg(7)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let continue_from_expr = get_nth_arg(8)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let cores = get_nth_arg(9)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                rayon::ThreadPoolBuilder::new()
                    .num_threads(cores)
                    .build_global()
                    .unwrap();
                dataset::generation_execution(
                    &expressions_file,
                    params,
                    reorder_count,
                    batch_size,
                    continue_from_expr,
                );
            }
            // Prove expressions using Caviar with/without ILC
            "prove" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions(&expression_vect, -1, params, true, false);
                write_results("tmp/results_prove.csv", &results).unwrap();
            }
            // Prove expressions using Caviar with pulses and with/without ILC.
            "pulses" => {
                let threshold = get_nth_arg(6)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results =
                    prove_expressions_pulses(&expression_vect, -1, threshold, params, true, false);
                write_results(&format!("tmp/results_beh_{}.csv", threshold), &results).unwrap();
            }
            // Prove expressions using Caviar with NPP and with/without ILC.
            "npp" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions_npp(&expression_vect, -1, params, true, false);
                write_results(&format!("tmp/results_fast.csv"), &results).unwrap();
            }
            // Prove expressions using Caviar with Pulses and NPP and with pulses and with/without ILC.
            "pulses_npp" => {
                let threshold = get_nth_arg(6)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions_pulses_npp(
                    &expression_vect,
                    -1,
                    threshold,
                    params,
                    true,
                    false,
                );
                write_results(&format!("tmp/results_beh_npp_{}.csv", threshold), &results).unwrap();
            }
            // Prove expressions using Caviar with clusters of rules and with pulses and with/without ILC.
            "clusters" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let classes_file = get_nth_arg(6).unwrap();
                let iterations_count = get_nth_arg(7)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                prove_clusters(
                    classes_file,
                    &expression_vect,
                    params,
                    iterations_count,
                    true,
                    true,
                );
            }
            "simplify" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = simplify_expressions(&expression_vect, -1, params, true);
                write_results("tmp/results_simplify.csv", &results).unwrap();
            }
            _ => {}
        }
    } else {
        //Quick executions with default parameters
        let params = get_runner_params(1).unwrap();
        let (start, end) = get_start_end().unwrap();
        println!("Simplifying expression:\n {}\n to {}", start, end);
        //Example of NPP execution with default parameters
        prove_equiv(&start, &end, -1, params, true, true);
    }
}
