use colored::*;
use std::fs::File;
use rand::thread_rng;
use egg::{RecExpr, Pattern, Runner, Searcher};
use crate::trs::{Math, ConstantFold, rules};
use json::JsonValue;
use std::io::Write;
use std::time::Duration;
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use json::object;

#[allow(dead_code)]
pub fn generate_dataset(expressions: Vec<(&str, &str)>, params: (usize, usize, u64), ruleset_id: i8, reorder_count: usize) {
    let mut dataset = File::create("results/dataset.json").unwrap();
    let mut rng = thread_rng();
    let mut start: RecExpr<Math>;
    let mut end: Pattern<Math>;
    let mut runner;
    let mut id;
    let mut matches;
    let mut i: usize;
    let mut counter: usize;
    // let mut minimal_ruleset_len: usize;
    let mut rule;
    let mut ruleset = rules(ruleset_id);
    let mut data_object;
    let mut data: Vec<JsonValue> = Vec::new();
    ruleset.shuffle(&mut rng);
    println!("Ruleset size == {}", ruleset.len());
    let mut ruleset_copy: Vec<egg::Rewrite<Math, ConstantFold>>;
    let mut ruleset_minimal: Vec<egg::Rewrite<Math, ConstantFold>>;
    let mut ruleset_copy_names: Vec<String>;
    for expression in expressions.iter() {
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
                    .with_time_limit(Duration::new(params.2, 0))
                    .with_expr(&start)
                    .run(ruleset_copy.iter());
                id = runner.egraph.find(*runner.roots.last().unwrap());
                matches = end.search_eclass(&runner.egraph, id);
                if matches.is_none() {
                    ruleset_copy.insert(i, rule);
                    i += 1;
                }
            }
            if ruleset_copy.len() < ruleset_minimal.len() {
                ruleset_minimal = ruleset_copy.clone();
            }
            counter += 1;
        }
        ruleset_copy_names = ruleset_minimal.clone().into_iter().map(|rule| rule.name().to_string()).rev().collect();
        data_object = object! {
            expression: object!{
                start: expression.0,
                end: expression.1,
            },
            rules: ruleset_copy_names
        };
        data.push(data_object);
        println!(
            "{0} rules are needed to prove: {1}",
            format!("{0}", ruleset_minimal.len()).red().bold(),
            format!("{0}", expression.0.to_string()).bright_green().bold(),
        );
        // for r in ruleset_copy{
        //     println!(
        //         "{}",format!("{}", r.name()).blue().bold()
        //     );
        // }
    }
    dataset.write_all(json::stringify(data).as_bytes()).unwrap();
}


#[allow(dead_code)]
pub fn generate_dataset_par(expressions: &Vec<(&str, &str)>, params: (usize, usize, u64), ruleset_id: i8, reorder_count: usize) {
    let mut dataset = File::create("results/dataset.json").unwrap();
    let data = Arc::new(Mutex::new(Vec::new()));
    expressions
        .par_iter()
        .for_each(|&expression| minimal_set_to_prove(expression, params, ruleset_id, reorder_count, &data));
    dataset.write_all(json::stringify(Arc::try_unwrap(data).unwrap().into_inner().unwrap()).as_bytes()).unwrap();
}

#[allow(dead_code)]
pub fn minimal_set_to_prove(expression: (&str, &str), params: (usize, usize, u64), ruleset_id: i8, reorder_count: usize, data: &Arc<Mutex<Vec<JsonValue>>>) {
    let mut rng = thread_rng();
    let mut start: RecExpr<Math>;
    let mut end: Pattern<Math>;
    let mut runner;
    let mut id;
    let mut matches;
    let mut i: usize;
    let mut counter: usize;
    let mut provedOnce: bool = false;
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
                .with_time_limit(Duration::new(params.2, 0))
                .with_expr(&start)
                .run(ruleset_copy.iter());
            id = runner.egraph.find(*runner.roots.last().unwrap());
            matches = end.search_eclass(&runner.egraph, id);
            if matches.is_none() {
                ruleset_copy.insert(i, rule);
                i += 1;
            } else {
                provedOnce = true;
            }
        }
        if ruleset_copy.len() < ruleset_minimal.len() {
            ruleset_minimal = ruleset_copy.clone();
        }
        counter += 1;
    }
    if provedOnce{
        ruleset_copy_names = ruleset_minimal.clone().into_iter().map(|rule| rule.name().to_string()).rev().collect();
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
            format!("{0}", expression.0.to_string()).bright_green().bold(),
        );
        // for r in ruleset_minimal{
        //     println!(
        //         "{}",format!("{}", r.name()).blue().bold()
        //     );
        // }
    }  
}


#[allow(dead_code)]
pub fn generate_dataset_0_1_par(expressions: &Vec<String>, ruleset_id: i8, params: (usize, usize, u64),use_iteration_check: bool, reorder_count: usize) {
    let mut dataset = File::create("results/dataset.json").unwrap();
    let data = Arc::new(Mutex::new(Vec::new()));
    expressions
        .par_iter()
        .for_each(|expression| minimal_set_to_prove_0_1(&expression,ruleset_id, params,  use_iteration_check, reorder_count,  &data));
    dataset.write_all(json::stringify(Arc::try_unwrap(data).unwrap().into_inner().unwrap()).as_bytes()).unwrap();
}


#[allow(dead_code)]
pub fn minimal_set_to_prove_0_1(expression: &str, ruleset_id:i8, params: (usize, usize, u64),use_iteration_check: bool,  reorder_count: usize, data: &Arc<Mutex<Vec<JsonValue>>>) {
    let mut rng = thread_rng();
    let mut start: RecExpr<Math>;
    // let mut end: Pattern<Math>;
    let end_1: Pattern<Math> = "1".parse().unwrap();
    let end_0: Pattern<Math> = "0".parse().unwrap();
    let goals = [end_0.clone(), end_1.clone()];
    let mut proved_goal= "0/1".to_string();
    let mut provedOnce: bool = false;
    let mut runner;
    let mut id;
    let mut matches;
    let mut i: usize;
    let mut counter: usize;
    // let mut minimal_ruleset_len: usize;
    let mut rule;
    let mut ruleset = rules(ruleset_id);
    let data_object;
    ruleset.shuffle(&mut rng);
    //println!("Ruleset size == {}", ruleset.len());
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
            // end = expression.1.parse().unwrap();
            runner = Runner::default()
                .with_iter_limit(params.0)
                .with_node_limit(params.1)
                .with_time_limit(Duration::new(params.2, 0))
                .with_expr(&start);

            if use_iteration_check{
                runner = runner.run_check_iteration(ruleset_copy.iter(), &goals);
            }else{
                runner = runner.run(ruleset_copy.iter());
            }
            id = runner.egraph.find(*runner.roots.last().unwrap());
            // matches = end.search_eclass(&runner.egraph, id);
            matches = goals.iter().all(|goal|{
                let mat = goal.search_eclass(&runner.egraph, id);
                if !mat.is_none(){
                    proved_goal = goal.to_string();
                }
                mat.is_none()
            });
            if matches {
                ruleset_copy.insert(i, rule);
                i += 1;
            } else {
                provedOnce = true;
            }
        }
        if ruleset_copy.len() < ruleset_minimal.len() {
            ruleset_minimal = ruleset_copy.clone();
        }
        counter += 1;
    } 
    if provedOnce {
        ruleset_copy_names = ruleset_minimal.clone().into_iter().map(|rule| rule.name().to_string()).rev().collect();
        data_object = object! {
                expression: object!{
                    start: expression,
                    end: proved_goal,
                },
                rules: ruleset_copy_names
            };
        data.lock().unwrap().push(data_object);
        println!(
            "{0} rules are needed to prove: {1}",
            format!("{0}", ruleset_minimal.len()).red().bold(),
            format!("{0}", expression.to_string()).bright_green().bold(),
        );
        // for r in ruleset_copy{
        //     println!(
        //         "{}",format!("{}", r.name()).blue().bold()
        //     );
        // }
    }
}