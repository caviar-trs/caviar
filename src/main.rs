use std::env;

use crate::io::reader::{get_start_end, read_expressions, get_first_arg, get_runner_params};
use crate::structs::{ExpressionStruct, ResultStructure};
use crate::trs::prove_expr;
use crate::io::writer::write_results;

mod trs;

mod rules;
mod io;
mod structs;
mod dataset;

fn simplify_expressions(exprs_vect: &Vec<ExpressionStruct>,ruleset_class: i8, params: (usize, usize, u64), use_iteration_check: bool,report: bool) -> Vec<ResultStructure> {
    let mut results = Vec::new();
    for expression in exprs_vect.iter() {
        results.push(prove_expr(expression, ruleset_class, params, use_iteration_check,report));
    }
    results
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let expressions = vec![
    //     ("( <= ( - v0 11 ) ( + ( * ( / ( - v0 v1 ) 12 ) 12 ) v1 ) )","1"),
    //     ("( <= ( + ( / ( - v0 v1 ) 8 ) 32 ) ( max ( / ( + ( - v0 v1 ) 257 ) 8 ) 0 ) )","1"),
    //     ("( <= ( min ( + ( * ( + v0 v1 ) 161 ) ( + ( min v2 v3 ) v4 ) ) v5 ) ( + ( * ( + v0 v1 ) 161 ) ( + v2 v4 ) ) )","1"),
    //     ("( == (+ a b) (+ b a) )","1"),
    //     ("( == (min a a) (a))","1"),
    // ];
    // generate_dataset(expressions,(30, 10000, 5), 2, 2);
    // generate_dataset_par(&expressions,(30, 10000, 5), 2, 10);



    if args.len() > 4 {
        let file_path = get_first_arg().unwrap();
        let params = get_runner_params(2).unwrap();
        let expression_vect = read_expressions(&file_path).unwrap();
        write_results("results/results_expressions_egg.csv",&simplify_expressions(&expression_vect, -1, params, true, true) ).unwrap();
    } else {
        let params = get_runner_params(1).unwrap();
        println!("{:?}", params);
        let (start, end) = get_start_end().unwrap();
        println!("Simplifying expression:\n {}\n to {}", start,end);
        trs::prove_equiv(&start,&end, -1, params, true, true);
        // trs::prove_expr(&start, &end, 2, true);
    }
}
