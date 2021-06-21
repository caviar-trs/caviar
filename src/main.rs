use std::{env, ffi::OsString, fs::File, io::Read, time::Instant};
use io::reader::{get_nth_arg, get_runner_params, get_start_end, read_expressions};
use io::writer::write_results;
use json::parse;
use std::time::Duration;
use structs::{ExpressionStruct, ResultStructure};
use trs::{prove, prove_equiv, simplify, prove_beh, prove_beh_npp, prove_expression_with_file_classes, prove_npp};
use colored::*;
mod trs;
use std::time::{SystemTime, UNIX_EPOCH};
mod dataset;
mod io;
mod rules;
mod structs;

fn get_epoch_s() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[allow(dead_code)]
fn prove_expressions(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    params: (usize, usize, f64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<ResultStructure> {
    let mut results = Vec::new();
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
        res.add_halide(expression.halide_result, expression.halide_time);
        results.push(res);
    }
    results
}

#[allow(dead_code)]
fn prove_expressions_beh(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    threshold: f64,
    params: (usize, usize, f64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<ResultStructure> {
    let mut results = Vec::new();
    for expression in exprs_vect.iter() {
        println!("Starting Expression: {}", expression.index);
        let mut res = prove_beh(
            expression.index,
            &expression.expression,
            ruleset_class,
            threshold,
            params,
            use_iteration_check,
            report,
        );
        res.add_halide(expression.halide_result, expression.halide_time);
        results.push(res);
    }
    results
}

#[allow(dead_code)]
fn prove_expressions_npp(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    params: (usize, usize, f64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<ResultStructure> {
    let mut results = Vec::new();
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
        res.add_halide(expression.halide_result, expression.halide_time);
        results.push(res);
    }
    results
}

#[allow(dead_code)]
fn prove_expressions_beh_npp(
    exprs_vect: &Vec<ExpressionStruct>,
    ruleset_class: i8,
    threshold: f64,
    params: (usize, usize, f64),
    use_iteration_check: bool,
    report: bool,
) -> Vec<ResultStructure> {
    let mut results = Vec::new();
    for expression in exprs_vect.iter() {
        println!("Starting Expression: {}", expression.index);
        let mut res = prove_beh_npp(
            expression.index,
            &expression.expression,
            ruleset_class,
            threshold,
            params,
            use_iteration_check,
            report,
        );
        res.add_halide(expression.halide_result, expression.halide_time);
        results.push(res);
    }
    results
}

fn test_classes(
    path: OsString,
    exprs_vect: &Vec<ExpressionStruct>,
    params: (usize, usize, f64),
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

fn summary(results: Vec<structs::ResultStructure>, npp: bool){
    let max_time: f64 = results.iter().map(|result| result.total_time).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min_time: f64 = results.iter().map(|result| result.total_time).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let average_time = results.iter().map(|result| result.total_time).sum::<f64>() as f64  / results.len() as f64;
    let total_time = results.iter().map(|result| result.total_time).sum::<f64>() as f64;
    let proved_expressions = results.iter().map(|result| result.result as usize).sum::<usize>() as usize;
    let mut npp_matched = 0;
    if npp{
        npp_matched = results.iter().filter(|result| result.stop_reason.contains("Impossible")).collect::<Vec<&ResultStructure>>().len();
    }
    println!("\n\n\n");
    println!(" =====================================");
    println!("| ");
    println!("| {}", format!("Execution Summary").green().bold());
    println!("| =================");
    println!("| {}{}","Total time:   ".truecolor(255,255,255) ,format!("{} s", total_time).cyan().bold());
    println!("| {}{}","Max time:     ".truecolor(255,255,255) ,format!("{} s", max_time).cyan().bold());
    println!("| {}{}","Min time:     ".truecolor(255,255,255) ,format!("{} s", min_time).cyan().bold());
    println!("| {}{}","Average time: ".truecolor(255,255,255) ,format!("{} s", average_time).cyan().bold());
    println!("| ");
    println!("| {}{}{}{}","Proved expressions ".truecolor(255,255,255), format!("{}", proved_expressions).green().bold()," out of ".truecolor(255,255,255) , format!("{}", results.len()).red().bold());
    if npp{
        println!("| {}{}{}{}{}",
        "Non provable expression identified ".truecolor(255,255,255), 
        format!("{}", npp_matched).green().bold()," out of ".truecolor(255,255,255), 
        format!("{}", results.len() - proved_expressions).red().bold(), 
        " Non proved expressions.".truecolor(255,255,255));
    }
    println!("| ");
    println!(" =====================================");

}

fn main() {
    let _args: Vec<String> = env::args().collect();
    // let expressions = vec![(
    //     "( == 0 ( - ( + 0 ( / ( + ( - 494 ( * v0 256 ) ) 21 ) 4 ) ) 1 ) )",
    //     "0"
    // )];
    // dataset::generate_dataset(expressions, (3000, 100000, 1), -2, 1);
    // generate_dataset_par(&expressions, (30, 10000, 5), 2, 10);
    // println!("Printing rules ...");
    // let arr = filteredRules(&get_first_arg().unwrap(), 1).unwrap();
    // for rule in arr {
    //     println!("{}", rule.name());
    // }
    // println!("End.");

    if _args.len() > 5 {
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
            "prove" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions(&expression_vect, -1, params, true, false);
                write_results("tmp/results_prove.csv", &results).unwrap();
            }
            "beh" => {
                let threshold = get_nth_arg(6)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results =
                    prove_expressions_beh(&expression_vect, -1, threshold, params, true, false);
                write_results(&format!("tmp/results_beh_{}.csv", threshold), &results).unwrap();
            }

            // "npp" => {
            //     let expression_vect = read_expressions(&expressions_file).unwrap();
            //     let results = prove_expressions_npp(&expression_vect, -1, params, true, false);
            //     write_results(&format!("tmp/results_fast.csv"), &results).unwrap();
            // }
            "beh_npp" => {
                let threshold = get_nth_arg(6)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results =
                    prove_expressions_beh_npp(&expression_vect, -1, threshold, params, true, false);
                write_results(&format!("tmp/results_beh_npp_{}.csv", threshold), &results).unwrap();
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

                let (_strct, _class, _exec_time) = prove_expression_with_file_classes(
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
            /* ===============DEMO=========================================== */
            "caviar" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions(&expression_vect, -1, params, false, true);
                write_results(&format!("tmp/demo/{}_results_caviar.csv",get_epoch_s()), &results).unwrap();
                summary(results, false);
            }
            "ilc" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions(&expression_vect, -1, params, true, true);
                write_results(&format!("tmp/demo/{}_results_ilc.csv",get_epoch_s()), &results).unwrap();
                summary(results, false);
            }
            "pulses" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let threshold = get_nth_arg(6)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                let results = prove_expressions_beh(&expression_vect, -1, threshold,params, false, true);
                write_results(&format!("tmp/demo/{}_results_pulses.csv",get_epoch_s()), &results).unwrap();
                summary(results, false);
            }
            "npp" => {
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions_npp(&expression_vect, -1, params, false, true);
                write_results(&format!("tmp/demo/{}_results_npp.csv",get_epoch_s()), &results).unwrap();
                summary(results, true);
            }
            "caviar+" => {
                let threshold = get_nth_arg(6)
                    .unwrap()
                    .into_string()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                let expression_vect = read_expressions(&expressions_file).unwrap();
                let results = prove_expressions_beh_npp(&expression_vect, -1, threshold, params, true, true);
                write_results(&format!("tmp/demo/{}_results_caviar+.csv",get_epoch_s()), &results).unwrap();
                summary(results, true);
            }
            _ => {}
        }
    } else {
        let operation = get_nth_arg(1).unwrap();
        let params = get_runner_params(2).unwrap();
        let (start, end) = get_start_end().unwrap();
        match operation.to_str().unwrap() {
            "prove" => {
                println!("\n\n\n");
                println!("{}", format!("Proving that an expression is True or False").cyan().bold());
                println!("===========================================");
                prove(1, &start, -1, params, false, true);
            }
            "prove_equivalence" => {
                println!("\n\n\n");
                println!("{}", format!("Proving equivalence between 2 expressions").cyan().bold());
                println!("=========================================");
                prove_equiv(&start, &end, -1, params, false, true);
            }
            "simplify" => {
                println!("\n\n\n");
                println!("{}", format!("Simplifying an expression to its shorter equivalent form").cyan().bold());
                println!("========================================================");
                simplify(&start, -1, params, true);
            }
            _ => {}
        }
    }
}
