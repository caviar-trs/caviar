use std::env;

use crate::io::reader::{get_start_end, read_expressions, get_first_arg, get_runner_params};
use crate::structs::{ExpressionStruct, ResultStructure};
use crate::trs::prove_expr;
use crate::io::writer::write_results;
use crate::dataset::{generate_dataset_par, minimal_set_to_prove_0_1, generate_dataset_0_1_par};

mod trs;

mod rules;
mod io;
mod structs;
mod dataset;

fn prove_expressions(exprs_vect: &Vec<ExpressionStruct>,ruleset_class: i8, params: (usize, usize, u64), use_iteration_check: bool,report: bool) -> Vec<ResultStructure> {
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
    //     ("( <= (/ a 2) (a))", "1"),
    //     ("( <= ( min ( + ( * ( + v0 v1 ) 161 ) ( + ( min v2 v3 ) v4 ) ) v5 ) ( + ( * ( + v0 v1 ) 161 ) ( + v2 v4 ) ) )","1"),
    //     ("( == (+ a b) (+ b a) )","1"),
    //     ("( == (min a b) (a))","1"),
    // ];
    // generate_dataset(expressions,(30, 10000, 5), 2, 2);
    //generate_dataset_par(&expressions,(30, 10000, 5), 2, 10);



    if args.len() > 4 {
        let file_path = get_first_arg().unwrap();
        let params = get_runner_params(2).unwrap();
        let expression_vect = read_expressions(&file_path).unwrap();
        let mut expression_str_vct = Vec::new();

        for expressionStruct in expression_vect.iter() {
            expression_str_vct.push( expressionStruct.expression.clone());
        }
        // generate_dataset_0_1_par(&expression_str_vct, -1,params,true, 10);
        prove_expressions(&expression_vect,-1,params,true,true);
    } else {
        let params = get_runner_params(1).unwrap();
        let (start, end) = get_start_end().unwrap();
        println!("Simplifying expression:\n {}\n to {}", start,end);
        trs::prove(&start, -2, params, false, true);
        // trs::prove_expr(&start, &end, 2, true);
    }
}
