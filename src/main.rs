mod trs;

use std::env;


mod rules;
mod io;
mod structs;

use crate::io::reader::{read_expressions,get_start_end};
use crate::structs::{ResultStructure, ExpressionStruct};

// fn run() -> Result<(), Box<dyn Error>> {
//     let file_path = get_first_arg()?;
//     let file = File::open(file_path)?;
//     let mut rdr = csv::Reader::from_reader(file);
//     let mut wtr = Writer::from_path("results/results_rules_egg.csv")?;
//     for result in rdr.records() {
//         let record = result?;
//         // println!("{:?}", &record[1]);
//         let index: i16 = record[0].parse::<i16>().unwrap();
//         let start = &record[2];
//         let end = &record[3];
//         let condition = &record[4];
//         println!("{:?}", index);
//         panic::set_hook(Box::new(|_info| {
//             // do nothing
//             println!("{:?}", _info);
//         }));
//         let result = panic::catch_unwind(|| -> ResultStructure {
//             println!("Simplifying expression:\n {}\n", start);
//             let result_record = trs::prove_for_csv(index, start, end, condition);
//             result_record
//         });
//
//         match result {
//             Ok(res) => wtr.serialize(res)?,
//             Err(_) => println!("Error at expression: {}", start),
//         }
//     }
//     wtr.flush();
//     Ok(())
// }


// fn run_expressions() -> Result<(), Box<dyn Error>> {
//     let file_path = get_first_arg()?;
//     let params = (get_runner_iter_limit().unwrap(), get_runner_node_limit().unwrap(), get_runner_time_limit().unwrap());
//     let file = File::open(file_path)?;
//     let mut rdr = csv::Reader::from_reader(file);
//     let mut wtr = Writer::from_path("results/results_expressions_egg.csv")?;
//     let start_t = Instant::now();
//     for result in rdr.records() {
//         let record = result?;
//         let index: i16 = record[0].parse::<i16>().unwrap();
//         let start = &record[1];
//         panic::set_hook(Box::new(|_info| {
//             // do nothing
//             println!("{:?}", _info);
//         }));
//         let result = panic::catch_unwind(|| -> ResultStructure {
//             println!("Simplifying expression:\n {}\n", start);
//             let result_record = trs::prove_exprs_for_csv_check(index, start, params, true);
//             result_record
//         });
//
//         match result {
//             Ok(res) => wtr.serialize(res)?,
//             Err(_) => println!("Error at expression: {}", start),
//         }
//     }
//     println!("Time elapsed in simplifying expressions is: {:?}", start_t.elapsed());
//     wtr.flush();
//     Ok(())
// }

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.



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
        trs::prove_equiv(&start,&end,  -1,(10,10000,5), true,true);
        // trs::prove_expr(&start, &end, 2, true);
    }
}
