use std::{env, ffi::OsString, fs::File, io::Read, time::Instant};

use crate::io::reader::{get_runner_params, get_start_end, read_expressions};
use crate::structs::{ExpressionStruct, ResultStructure};
use crate::{io::writer::write_results, trs::prove_fast_fail};
use dataset::generate_dataset_par;
use io::reader::get_nth_arg;
use json::parse;
use std::time::Duration;
use trs::{prove, prove_equiv, prove_expression_with_file_classes, prove_multiple_passes};
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
            expression.index,
            &expression.expression,
            ruleset_class,
            params,
            use_iteration_check,
            report,
        ));
    }
    results
}

#[allow(dead_code)]
fn prove_expressions_multiple_passes(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    threshold: f64,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<ResultStructure> {
    let mut results = Vec::new();
    for expression in exprs_vect.iter() {
        results.push(prove_multiple_passes(
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

#[allow(dead_code)]
fn prove_expressions_fast(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    threshold: f64,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<ResultStructure> {
    let mut results = Vec::new();
    for expression in exprs_vect.iter() {
        results.push(prove_fast_fail(
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

fn test_classes(
    path: OsString,
    exprs_vect: &Vec<ExpressionStruct>,
    params: (usize, usize, u64),
    count: usize,
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
    let mut average;
    let mut prove_result: (ResultStructure, i64, Duration);
    let mut i;
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
                false,
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
    // let expressions = vec![
    //     ("( < ( min y ( + x 2 ) ) x )","1"),
    // ];
    // dataset::generate_dataset(expressions,(3000, 100000, 5), -2, 15);
    // generate_dataset_par(&expressions,(30, 10000, 5), 2, 10);
    // println!("Printing rules ...");
    // let arr = filteredRules(&get_first_arg().unwrap(), 1).unwrap();
    // for rule in arr{
    //     println!("{}", rule.name());
    // }
    // println!("End.");

    if _args.len() > 4 {
        let operation = get_nth_arg(1).unwrap();
        let expressions_file = get_nth_arg(2).unwrap();
        let params = get_runner_params(3).unwrap();
        match operation.to_str().unwrap() {
            "comparaison" => {
                /*let expression_vect = read_expressions(&expressions_file).unwrap();
                let classes_file = get_nth_arg(6).unwrap();
                let mut average_k = 0.0;
                let mut average = 0.0;
                for i in 0..5000{
                    let results_k = test_classes(classes_file.clone(), &expression_vect, params, true, false);
                    let results = prove_expressions(&expression_vect, -1, params, true, false);
                    average_k += results_k[0].total_time;
                    average += results[0].total_time;
                }
                println!("Average time with classes {}", average_k/5000.0);
                println!("Average time without classes {}", average/5000.0);*/
            }
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
            "prove_exprs" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions(&expression_vect, -1, params, true, true);
                write_results("tmp/results_prove.csv", &results).unwrap();
            }
            "prove_exprs_passes" => {
                let threshold = get_nth_arg(6)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions_multiple_passes(
                    &expression_vect,
                    -1,
                    threshold,
                    params,
                    true,
                    true,
                );
                write_results(
                    &format!("tmp/results_multiple_passes_{}.csv", threshold),
                    &results,
                )
                .unwrap();
            }
            "prove_exprs_fast" => {
                let threshold = get_nth_arg(6)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results =
                    prove_expressions_fast(&expression_vect, -1, threshold, params, true, true);
                write_results(&format!("tmp/results_fast_{}.csv", threshold), &results).unwrap();
            }
            "test_classes" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let classes_file = get_nth_arg(6).unwrap();
                let iterations_count = get_nth_arg(7)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                test_classes(
                    classes_file,
                    &expression_vect,
                    params,
                    iterations_count,
                    true,
                    true,
                );
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
        // println!(
        //     "{:?}",
        //     prove_multiple_passes(-1, &start, -1, 0.5, params, true, true)
        // );
        // println!("{:?}", prove_equiv(&start, &end, -1, params, true, true));
        // println!("{:?}", prove(-1, &start, -1, params, true, true));
        println!(
            "{:?}",
            prove_fast_fail(-1, &start, -1, 1.0, params, true, true)
        );

        println!(
            "{:?}",
            prove_multiple_passes(-1, &start, -1, 1.0, params, true, true)
        );

        // let expressions = vec![(&start[..], &end[..])];
        // generate_dataset_par(&expressions, params, -1, 10);
    }
}
