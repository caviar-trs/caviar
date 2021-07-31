use crate::trs::{rules, ConstantFold, Math};
use colored::*;
use csv::ReaderBuilder;
use egg::{Pattern, RecExpr, Runner, Searcher};
use json::object;
use json::JsonValue;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Given a vector (expression, goal), the functions writes
/// to dataset.json each expression along with the approximate
/// minimal subset of rules needed to prove it. (Parallelized)
#[allow(dead_code)]
pub fn generate_dataset_par(
    expressions: &Vec<(&str, &str)>,
    params: (usize, usize, f64),
    ruleset_id: i8,
    reorder_count: usize,
) {
    let mut dataset = File::create("results/dataset.json").unwrap();
    let data = Arc::new(Mutex::new(Vec::new()));
    expressions.par_iter().for_each(|&expression| {
        minimal_set_to_prove(expression, params, ruleset_id, reorder_count, &data)
    });
    dataset
        .write_all(json::stringify(Arc::try_unwrap(data).unwrap().into_inner().unwrap()).as_bytes())
        .unwrap();
}

/// For a given expression and goal, the function
/// adds to the data vector the expression along 
/// with the minimal subset of rules it needs to be
/// proved. Algorithm explained in the manuscript
#[allow(dead_code)]
pub fn minimal_set_to_prove(
    expression: (&str, &str),
    params: (usize, usize, f64),
    ruleset_id: i8,
    reorder_count: usize,
    data: &Arc<Mutex<Vec<JsonValue>>>,
) {
    let mut rng = thread_rng();
    let mut start: RecExpr<Math>;
    let mut end: Pattern<Math>;
    let mut runner;
    let mut id;
    let mut matches;
    let mut i: usize;
    let mut counter: usize;
    let mut proved_once: bool = false;
    // let mut minimal_ruleset_len: usize;
    let mut rule;
    let mut ruleset = rules(ruleset_id);
    let data_object;
    ruleset.shuffle(&mut rng);
    println!("Ruleset size == {}", ruleset.len());
    let mut ruleset_copy: Vec<egg::Rewrite<Math, ConstantFold>>;
    let mut ruleset_minimal: Vec<egg::Rewrite<Math, ConstantFold>>;
    let ruleset_copy_names: Vec<String>;
    counter = 0;
    ruleset_minimal = ruleset.clone();
    while counter < reorder_count {
        ruleset_copy = ruleset.clone();
        ruleset_copy.shuffle(&mut rng);
        i = 0;
        while i < ruleset_copy.len() {
            rule = ruleset_copy.remove(i);
            start = expression.0.parse().unwrap();
            end = expression.1.parse().unwrap();
            runner = Runner::default()
                .with_iter_limit(params.0)
                .with_node_limit(params.1)
                .with_time_limit(Duration::from_secs_f64(params.2))
                .with_expr(&start)
                .run(ruleset_copy.iter());
            id = runner.egraph.find(*runner.roots.last().unwrap());
            matches = end.search_eclass(&runner.egraph, id);
            if matches.is_none() {
                ruleset_copy.insert(i, rule);
                i += 1;
            } else {
                proved_once = true;
            }
        }
        if ruleset_copy.len() < ruleset_minimal.len() {
            ruleset_minimal = ruleset_copy.clone();
        }
        counter += 1;
    }
    if proved_once {
        ruleset_copy_names = ruleset_minimal
            .clone()
            .into_iter()
            .map(|rule| rule.name().to_string())
            .rev()
            .collect();
        data_object = object! {
            expression: object!{
                start: expression.0,
                end: expression.1,
            },
            rules: ruleset_copy_names
        };
        data.lock().unwrap().push(data_object);
        println!(
            "{0} rules are needed to prove: {1}",
            format!("{0}", ruleset_minimal.len()).red().bold(),
            format!("{0}", expression.0.to_string())
                .bright_green()
                .bold(),
        );
        // for r in ruleset_minimal{
        //     println!(
        //         "{}",format!("{}", r.name()).blue().bold()
        //     );
        // }
    }
}

/// Same as generate_dataset, but this time
/// the expression are passed without goals 
/// The goal is assumed to be either 0 or 1.
#[allow(dead_code)]
pub fn generate_dataset_0_1_par(
    expressions: &Vec<String>,
    ruleset_id: i8,
    params: (usize, usize, f64),
    use_iteration_check: bool,
    reorder_count: usize,
    batch_number: usize,
) {
    let results_file_name = format!("results/dataset-batch-{}.json", batch_number);
    let mut dataset = File::create(results_file_name).unwrap();
    let data = Arc::new(Mutex::new(Vec::new()));
    println!(
        "Generating batch #{0} with params iter_limit={1} nodes_limit={2} time_limit={3}",
        batch_number, params.0, params.1, params.2
    );
    expressions.par_iter().for_each(|expression| {
        minimal_set_to_prove_0_1(
            &expression,
            ruleset_id,
            params,
            use_iteration_check,
            reorder_count,
            &data,
            batch_number,
        )
    });
    dataset
        .write_all(json::stringify(Arc::try_unwrap(data).unwrap().into_inner().unwrap()).as_bytes())
        .unwrap();
}

/// Same as minimal_set_to_prove, but this time
/// the expression are passed without goals 
/// The goal is assumed to be either 0 or 1.
#[allow(dead_code)]
pub fn minimal_set_to_prove_0_1(
    expression: &str,
    ruleset_id: i8,
    params: (usize, usize, f64),
    use_iteration_check: bool,
    reorder_count: usize,
    data: &Arc<Mutex<Vec<JsonValue>>>,
    batch_number: usize,
) {
    let mut rng = thread_rng();
    let mut start: RecExpr<Math>;
    let end_1: Pattern<Math> = "1".parse().unwrap();
    let end_0: Pattern<Math> = "0".parse().unwrap();
    let goals = [end_0.clone(), end_1.clone()];
    let mut proved_goal = "0/1".to_string();
    let result = crate::trs::prove(-1, expression, -2, params, true, false);
    if result.result {
        let mut runner;
        let mut id;
        let mut matches;
        let mut i: usize;
        let mut counter: usize;
        let mut rule;
        let mut ruleset = rules(ruleset_id);
        let data_object;
        ruleset.shuffle(&mut rng);
        let mut ruleset_copy: Vec<egg::Rewrite<Math, ConstantFold>>;
        let mut ruleset_minimal: Vec<egg::Rewrite<Math, ConstantFold>>;
        let ruleset_copy_names: Vec<String>;
        counter = 0;
        ruleset_minimal = ruleset.clone();
        while counter < reorder_count {
            ruleset_copy = ruleset.clone();
            ruleset_copy.shuffle(&mut rng);
            i = 0;
            while i < ruleset_copy.len() {
                rule = ruleset_copy.remove(i);
                start = expression.parse().unwrap();
                runner = Runner::default()
                    .with_iter_limit(params.0)
                    .with_node_limit(params.1)
                    .with_time_limit(Duration::from_secs_f64(params.2))
                    .with_expr(&start);

                if use_iteration_check {
                    runner = runner.run_check_iteration(ruleset_copy.iter(), &goals);
                } else {
                    runner = runner.run(ruleset_copy.iter());
                }
                id = runner.egraph.find(*runner.roots.last().unwrap());
                matches = goals.iter().all(|goal| {
                    let mat = goal.search_eclass(&runner.egraph, id);
                    if !mat.is_none() {
                        proved_goal = goal.to_string();
                    }
                    mat.is_none()
                });
                if matches {
                    ruleset_copy.insert(i, rule);
                    i += 1;
                }
            }
            if ruleset_copy.len() < ruleset_minimal.len() {
                ruleset_minimal = ruleset_copy.clone();
            }
            counter += 1;
        }
        ruleset_copy_names = ruleset_minimal
            .clone()
            .into_iter()
            .map(|rule| rule.name().to_string())
            .rev()
            .collect();
        data_object = object! {
            expression: object!{
                start: expression,
                end: proved_goal,
            },
            rules: ruleset_copy_names
        };
        data.lock().unwrap().push(data_object);
        println!(
            "Batch #{0}: {1} rules are needed to prove: {2}",
            format!("{0}", batch_number).blue().bold(),
            format!("{0}", ruleset_minimal.len()).red().bold(),
            format!("{0}", expression.to_string()).bright_green().bold(),
        );
        // for r in ruleset_copy{
        //     println!(
        //         "{}",format!("{}", r.name()).blue().bold()
        //     );
        // }
    } else {
        println!(
            "Batch #{0}: Could not prove {1}",
            format!("{0}", batch_number).blue().bold(),
            format!("{0}", expression.to_string()).red().bold()
        );
    }
}

/// This function was made to generate the
/// dataset for big number of expressions.
/// It starts by reading expressions from a CSV file
/// that must have the format (id, expression) for each
/// row. It reads batch_size expressions before passing
/// them to generate_dataset_0_1_par, which generate the 
/// dataset for this batch of expressions and store it in a 
/// json file. Until the vector of expressions end.
/// The continue_from_expr parameter can be used in case of failure.
/// For example if we have 1 million expressions, our batch_size
/// is set to 1000, and the execution fails after generating the
/// dataset of 200 batched, we should relaunch the execution starting 
/// from the 200*1000 = 200,000th expression.
pub fn generation_execution(
    file_path: &OsString,
    params: (usize, usize, f64),
    reorder_count: usize,
    batch_size: usize,
    continue_from_expr: usize,
) {
    let mut expressions_vect = Vec::new();
    let file = File::open(file_path).unwrap();
    //let mut rdr = csv::Reader::from_reader(file);
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(file);
    let mut i = 0;
    for result in rdr.records() {
        i += 1;
        if i > continue_from_expr {
            let record = result.unwrap();
            let expression = &record[1];
            expressions_vect.push(expression.to_string());
            if i % batch_size == 0 {
                generate_dataset_0_1_par(
                    &expressions_vect,
                    -2,
                    params,
                    true,
                    reorder_count,
                    i / batch_size,
                );
                println!("{} expressions processed!", i);
                expressions_vect = Vec::new();
            }
        }
    }
    if expressions_vect.len() > 0 {
        generate_dataset_0_1_par(
            &expressions_vect,
            -2,
            params,
            true,
            reorder_count,
            i / batch_size + 1,
        );
        println!("{} expressions processed!", i);
    }
}