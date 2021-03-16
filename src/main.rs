mod trs;

use std::env;


mod rules;
mod io;
mod structs;
mod dataset;

use crate::io::reader::{read_expressions,get_start_end};
use crate::structs::Rule;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let expressions = vec![
    //     ("( <= ( - v0 11 ) ( + ( * ( / ( - v0 v1 ) 12 ) 12 ) v1 ) )","1"),
    //     ("( <= ( + ( / ( - v0 v1 ) 8 ) 32 ) ( max ( / ( + ( - v0 v1 ) 257 ) 8 ) 0 ) )","1"),
    //     ("( <= ( min ( + ( * ( + v0 v1 ) 161 ) ( + ( min v2 v3 ) v4 ) ) v5 ) ( + ( * ( + v0 v1 ) 161 ) ( + v2 v4 ) ) )","1"),
    //     ("( == (+ a b) (+ b a) )","1"),
    //     ("( == (min a a) (a))","1"),
    // ];
    //trs::generate_dataset(expressions,(30, 10000, 5), 2, 2);
    // trs::generate_dataset_par(&expressions,(30, 10000, 5), 2, 10);
    
    if args.len() > 1 {
        let expression_vect = read_expressions();
        println!("{:?}",expression_vect);
    } else {
        let (start, end) = get_start_end().unwrap();
        println!("Simplifying expression:\n {}\n", start);
        trs::prove_rule(Rule::new(1,"(== a a)".to_string(), "0".to_string(), None),  -1,(10,10000,5), true,true);
        // trs::prove_expr(&start, &end, 2, true);
    }
}
