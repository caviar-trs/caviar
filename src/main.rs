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
    // let expressions = vec![
    //     ("( <= ( - v0 11 ) ( + ( * ( / ( - v0 v1 ) 12 ) 12 ) v1 ) )","1"),
    //     ("( <= ( + ( / ( - v0 v1 ) 8 ) 32 ) ( max ( / ( + ( - v0 v1 ) 257 ) 8 ) 0 ) )","1"),
    //     ("( <= (/ a 2) (a))", "1"),
    //     ("( <= ( min ( + ( * ( + v0 v1 ) 161 ) ( + ( min v2 v3 ) v4 ) ) v5 ) ( + ( * ( + v0 v1 ) 161 ) ( + v2 v4 ) ) )","1"),
    //     ("( == (+ a b) (+ b a) )","1"),
    //     ("( == (min a b) (a))","1"),
    // ];
    // generate_dataset(expressions,(30, 10000, 5), 2, 2);
    // generate_dataset_par(&expressions,(30, 10000, 5), 2, 10);
    // println!("Printing rules ...");
    // let arr = filteredRules(&get_first_arg().unwrap(), 1).unwrap();
    // for rule in arr{
    //     println!("{}", rule.name());
    // }
    // println!("End.");

    if _args.len() > 1 {
        let operation = get_nth_arg(1).unwrap();
        
        match operation.to_str().unwrap() {
            "dataset" => {
                let expressions_file = get_nth_arg(2).unwrap();
                let params = get_runner_params(3).unwrap();
                dataset::generation_execution(&expressions_file, params, 5, 500);
            }
            "prove_exprs" => {
                let expressions_file = get_nth_arg(2).unwrap();
                let params = get_runner_params(3).unwrap();
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions(&expression_vect, -1, params, true, true);
                write_results("tmp/generated_expressions_results.csv", &results).unwrap();
            }
            "test_classes" => {
                let expressions_file = get_nth_arg(2).unwrap();
                let params = get_runner_params(3).unwrap();
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let classes_file = get_nth_arg(6).unwrap();
                test_classes(classes_file, &expression_vect, params, true, false);
            }
            "prove_one_expr" => {
                let expressions_file = get_nth_arg(2).unwrap();
                let params = get_runner_params(3).unwrap();
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
            "experiment_clustering" => {
                let dataset = get_nth_arg(2).unwrap();
                let mut file = File::open(dataset).unwrap();
                let mut s = String::new();
                file.read_to_string(&mut s).unwrap();
                let expressions = parse(&s).unwrap();
                dataset::experiment_minimal_subset(&expressions, (30, 10000, 5));
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
