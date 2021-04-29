use json::JsonValue;
// use ordered_float::NotNan;
use std::error::Error;
use std::time::Duration;
use std::{cmp::Ordering, time::Instant};

use colored::*;
use egg::*;

use crate::structs::{ResultStructure, Rule};

pub type EGraph = egg::EGraph<Math, ConstantFold>;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;

define_language! {
    pub enum Math {
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        "%" = Mod([Id; 2]),
        "max" = Max([Id; 2]),
        "min" = Min([Id; 2]),
        "<" = Lt([Id; 2]),
        ">" = Gt([Id; 2]),
        "!" = Not(Id),
        "<=" = Let([Id;2]),
        ">=" = Get([Id;2]),
        "==" = Eq([Id; 2]),
        "!=" = IEq([Id; 2]),
        "||" = Or([Id; 2]),
        "&&" = And([Id; 2]),
        Constant(i64),
        Symbol(Symbol),
    }
}

#[derive(Default, Clone)]
pub struct ConstantFold;

impl Analysis<Math> for ConstantFold {
    type Data = Option<i64>;

    fn merge(&self, a: &mut Self::Data, b: Self::Data) -> Option<Ordering> {
        match (a.as_mut(), &b) {
            (None, None) => Some(Ordering::Equal),
            (None, Some(_)) => {
                *a = b;
                Some(Ordering::Less)
            }
            (Some(_), None) => Some(Ordering::Greater),
            (Some(_), Some(_)) => Some(Ordering::Equal),
        }
        // if a.is_none() && b.is_some() {
        //     *a = b
        // }
        // cmp
    }

    fn make(egraph: &EGraph, enode: &Math) -> Self::Data {
        let x = |i: &Id| egraph[*i].data.as_ref();
        Some(match enode {
            Math::Constant(c) => (*c),
            Math::Add([a, b]) => (x(a)? + x(b)?),
            Math::Sub([a, b]) => (x(a)? - x(b)?),
            Math::Mul([a, b]) => (x(a)? * x(b)?),
            Math::Div([a, b]) if *x(b)? != 0 => (x(a)? / x(b)?),
            Math::Max([a, b]) => std::cmp::max(*x(a)?, *x(b)?),
            Math::Min([a, b]) => std::cmp::min(*x(a)?, *x(b)?),
            Math::Not(a) => {
                if *x(a)? == 0 {
                    1
                } else {
                    0
                }
            }
            Math::Lt([a, b]) => {
                if x(a)? < x(b)? {
                    1
                } else {
                    0
                }
            }
            Math::Gt([a, b]) => {
                if x(a)? > x(b)? {
                    1
                } else {
                    0
                }
            }
            Math::Let([a, b]) => {
                if x(a)? <= x(b)? {
                    1
                } else {
                    0
                }
            }
            Math::Get([a, b]) => {
                if x(a)? >= x(b)? {
                    1
                } else {
                    0
                }
            }
            Math::Mod([a, b]) => {
                if *x(b)? == 0 {
                    0
                } else {
                    x(a)? % x(b)?
                }
            }
            Math::Eq([a, b]) => {
                if x(a)? == x(b)? {
                    1
                } else {
                    0
                }
            }
            Math::IEq([a, b]) => {
                if x(a)? != x(b)? {
                    1
                } else {
                    0
                }
            }
            Math::And([a, b]) => {
                if *x(a)? == 0 || *x(b)? == 0 {
                    0
                } else {
                    1
                }
            }
            Math::Or([a, b]) => {
                if *x(a)? == 1 || *x(b)? == 1 {
                    1
                } else {
                    0
                }
            }

            _ => return None,
        })
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        let class = &mut egraph[id];
        if let Some(c) = class.data.clone() {
            let added = egraph.add(Math::Constant(c.clone()));
            let (id, _did_something) = egraph.union(id, added);
            // to not prune, comment this out
            egraph[id].nodes.retain(|n| n.is_leaf());

            assert!(
                !egraph[id].nodes.is_empty(),
                "empty eclass! {:#?}",
                egraph[id]
            );
            #[cfg(debug_assertions)]
            egraph[id].assert_unique_leaves();
        }
    }
}

pub fn is_const_pos(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var]].nodes.iter().any(|n| match n {
            Math::Constant(c) => c > &0,
            _ => return false,
        })
    }
}

pub fn is_const_neg(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var]].nodes.iter().any(|n| match n {
            Math::Constant(c) => c < &0,
            _ => return false,
        })
    }
}

pub fn is_not_zero(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    let zero = Math::Constant(0);
    move |egraph, _, subst| !egraph[subst[var]].nodes.contains(&zero)
}

pub fn compare_c0_c1(
    var: &str,
    var1: &str,
    comp: &'static str,
) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var: Var = var.parse().unwrap();
    let var1: Var = var1.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var1]].nodes.iter().any(|n1| match n1 {
            Math::Constant(c1) => egraph[subst[var]].nodes.iter().any(|n| match n {
                Math::Constant(c) => match comp {
                    "<" => c < c1,
                    "<a" => c < &c1.abs(),
                    "<=" => c <= c1,
                    "<=+1" => c <= &(c1 + 1),
                    "<=a" => c <= &c1.abs(),
                    "<=-a" => c <= &(-c1.abs()),
                    "<=-a+1" => c <= &(1 - c1.abs()),
                    ">" => c > c1,
                    ">a" => c > &c1.abs(),
                    ">=" => c >= c1,
                    ">=a" => c >= &(c1.abs()),
                    ">=a-1" => c >= &(c1.abs() - 1),
                    "!=" => c != c1,
                    "%0" => (*c1 != 0) && (c % c1 == 0),
                    "%0<" => (*c1 > 0) && (c % c1 == 0),
                    "%0>" => (*c1 < 0) && (c % c1 == 0),
                    _ => false,
                },
                _ => return false,
            }),
            _ => return false,
        })
    }
}

// Takes a JSON array of rules ids and return the vector of their associated Rewrites
pub fn filtered_rules(class: &json::JsonValue) -> Result<Vec<Rewrite>, Box<dyn Error>> {
    let add_rules = crate::rules::add::add();
    let and_rules = crate::rules::and::and();
    let andor_rules = crate::rules::andor::andor();
    let div_rules = crate::rules::div::div();
    let eq_rules = crate::rules::eq::eq();
    let ineq_rules = crate::rules::ineq::ineq();
    let lt_rules = crate::rules::lt::lt();
    let max_rules = crate::rules::max::max();
    let min_rules = crate::rules::min::min();
    let modulo_rules = crate::rules::modulo::modulo();
    let mul_rules = crate::rules::mul::mul();
    let not_rules = crate::rules::not::not();
    let or_rules = crate::rules::or::or();
    let sub_rules = crate::rules::sub::sub();

    let all_rules: Vec<Rewrite> = [
        &add_rules[..],
        &and_rules[..],
        &andor_rules[..],
        &div_rules[..],
        &eq_rules[..],
        &ineq_rules[..],
        &lt_rules[..],
        &max_rules[..],
        &min_rules[..],
        &modulo_rules[..],
        &mul_rules[..],
        &not_rules[..],
        &or_rules[..],
        &sub_rules[..],
    ]
    .concat();
    let rules_iter = all_rules.into_iter();
    let rules = rules_iter.filter(|rule| class.contains(rule.name()));
    return Ok(rules.collect());
}

#[rustfmt::skip]
pub fn rules(ruleset_class: i8) -> Vec<Rewrite> {
    let add_rules = crate::rules::add::add();
    let and_rules = crate::rules::and::and();
    let andor_rules = crate::rules::andor::andor();
    let div_rules = crate::rules::div::div();
    let eq_rules = crate::rules::eq::eq();
    let ineq_rules = crate::rules::ineq::ineq();
    let lt_rules = crate::rules::lt::lt();
    let max_rules = crate::rules::max::max();
    let min_rules = crate::rules::min::min();
    let modulo_rules = crate::rules::modulo::modulo();
    let mul_rules = crate::rules::mul::mul();
    let not_rules = crate::rules::not::not();
    let or_rules = crate::rules::or::or();
    let sub_rules = crate::rules::sub::sub();

    return match ruleset_class {
        0 =>
            [
                &add_rules[..],
                &div_rules[..],
                &modulo_rules[..],
                &mul_rules[..],
                &sub_rules[..],
            ].concat(),
        _ => [
            &add_rules[..],
            &and_rules[..],
            &andor_rules[..],
            &div_rules[..],
            &eq_rules[..],
            &ineq_rules[..],
            &lt_rules[..],
            &max_rules[..],
            &min_rules[..],
            &modulo_rules[..],
            &mul_rules[..],
            &not_rules[..],
            &or_rules[..],
            &sub_rules[..],
        ].concat()
    };
}

#[allow(dead_code)]
pub fn print_graph(egraph: &EGraph, name: &str) {
    println!("printing graph to svg");
    egraph
        .dot()
        .to_svg("results/".to_owned() + name + ".svg")
        .unwrap();
    println!("done printing graph to svg");
}

#[allow(dead_code)]
pub fn simplify(
    start_expression: &str,
    ruleset_class: i8,
    params: (usize, usize, u64),
    report: bool,
) {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let runner = Runner::default()
        .with_iter_limit(params.0)
        .with_node_limit(params.1)
        .with_time_limit(Duration::new(params.2, 0))
        .with_expr(&start)
        .run(rules(ruleset_class).iter());
    let id = runner.egraph.find(*runner.roots.last().unwrap());
    let mut extractor = Extractor::new(&runner.egraph, AstSize);
    let (_, best_expr) = extractor.find_best(id);
    println!(
        "Best Expr: {}",
        format!("{}", best_expr).bright_green().bold()
    );

    if report {
        runner.print_report();
    }
}

#[allow(dead_code)]
pub fn prove_equiv(
    start_expression: &str,
    end_expressions: &str,
    ruleset_class: i8,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> ResultStructure {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end: Pattern<Math> = end_expressions.parse().unwrap();
    let result: bool;
    let runner;
    let best_expr_string;
    if use_iteration_check {
        runner = Runner::default()
            .with_iter_limit(params.0)
            .with_node_limit(params.1)
            .with_time_limit(Duration::new(params.2, 0))
            .with_expr(&start)
            .run_check_iteration(rules(ruleset_class).iter(), &[end.clone()]);
    } else {
        runner = Runner::default()
            .with_iter_limit(params.0)
            .with_node_limit(params.1)
            .with_time_limit(Duration::new(params.2, 0))
            .with_expr(&start)
            .run(rules(ruleset_class).iter());
    }

    let id = runner.egraph.find(*runner.roots.last().unwrap());
    let matches = end.search_eclass(&runner.egraph, id);
    if matches.is_none() {
        let mut extractor = Extractor::new(&runner.egraph, AstDepth);
        let (_, best_expr) = extractor.find_best(id);
        best_expr_string = Some(best_expr.to_string());

        if report {
            println!(
                "{}\n{}\n",
                "Could not prove goal:".bright_red().bold(),
                end.pretty(40),
            );
            println!(
                "Best Expr: {}",
                format!("{}", best_expr).bright_green().bold()
            );
        }

        result = false;
    } else {
        if report {
            println!(
                "{}\n{}\n",
                "Proved goal:".bright_green().bold(),
                end.pretty(40)
            );
        }
        result = true;
        best_expr_string = Some(end.to_string())
    }
    let total_time: f64 = runner.iterations.iter().map(|i| i.total_time).sum();
    if report {
        runner.print_report();
    }

    let stop_reason = match runner.stop_reason.unwrap() {
        StopReason::Saturated => "Saturation".to_string(),
        StopReason::IterationLimit(iter) => format!("Iterations: {}", iter),
        StopReason::NodeLimit(nodes) => format!("Node Limit: {}", nodes),
        StopReason::TimeLimit(time) => format!("Time Limit : {}", time),
        StopReason::Other(reason) => reason,
    };

    ResultStructure::new(
        -1,
        start_expression.to_string(),
        end_expressions.to_string(),
        result,
        best_expr_string.unwrap_or_default(),
        ruleset_class as i64,
        runner.iterations.len(),
        runner.egraph.total_number_of_nodes(),
        runner.iterations.iter().map(|i| i.n_rebuilds).sum(),
        total_time,
        stop_reason,
        None,
    )
}

#[allow(dead_code)]
pub fn prove(
    index: i16,
    start_expression: &str,
    ruleset_class: i8,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> ResultStructure {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end_1: Pattern<Math> = "1".parse().unwrap();
    let end_0: Pattern<Math> = "0".parse().unwrap();
    let goals = [end_0.clone(), end_1.clone()];
    let impossible: Pattern<Math> = "(== ?a ?c)".parse().unwrap();
    let impossibles = [impossible];
    let runner: Runner<Math, ConstantFold>;
    let mut result = false;
    let mut proved_goal_index = 0;
    let id;
    let best_expr;

    // // print the ruleset used as a vector of strings
    // println!(
    //     "{:?}",
    //     rules(ruleset_class)
    //         .iter()
    //         .map(|rew| rew.name.clone())
    //         .collect::<Vec<String>>()
    // );

    if report {
        println!(
            "\n====================================\nProving Expression:\n {}\n",
            start_expression
        )
    }
    if use_iteration_check {
        runner = Runner::default()
            .with_iter_limit(params.0)
            .with_node_limit(params.1)
            .with_time_limit(Duration::new(params.2, 0))
            .with_expr(&start)
            .run_check_iteration(rules(ruleset_class).iter(), &goals);
    } else {
        runner = Runner::default()
            .with_iter_limit(params.0)
            .with_node_limit(params.1)
            .with_time_limit(Duration::new(params.2, 0))
            .with_expr(&start)
            .run(rules(ruleset_class).iter());
    }

    id = runner.egraph.find(*runner.roots.last().unwrap());
    for (goal_index, goal) in goals.iter().enumerate() {
        let boolean = (goal.search_eclass(&runner.egraph, id)).is_none();
        if !boolean {
            result = true;
            proved_goal_index = goal_index;
            break;
        }
    }

    if result {
        if report {
            println!(
                "{}\n{:?}",
                "Proved goal:".bright_green().bold(),
                goals[proved_goal_index].to_string()
            );
        }
        best_expr = Some(goals[proved_goal_index].to_string());
    } else {
        let mut extractor = Extractor::new(&runner.egraph, AstDepth);
        let now = Instant::now();
        let (_, best_exprr) = extractor.find_best(id);
        // let mut egraph = EGraph::default();
        // let a11 = egraph.add_expr(&best_exprr.to_string().parse().unwrap());
        // egraph.rebuild();
        // for (impo_index, impo) in impossibles.iter().enumerate() {
        //     let results = impo.search_eclass(&egraph, a11).unwrap();

        //     // for result in results {}
        //     let a: Var = "?a".parse().unwrap();
        //     let c: Var = "?c".parse().unwrap();

        //     println!("{:?}", egraph[results.substs[0][a]].nodes);
        //     println!(
        //         "{:?}",
        //         egraph[results.substs[0][a]].nodes.iter().any(|n| match n {
        //             Math::Symbol(_) => true,
        //             _ => return false,
        //         }) && egraph[results.substs[0][c]].nodes.iter().all(|n| match n {
        //             Math::Symbol(_) => false,
        //             _ => return true,
        //         })
        //     )
        //     // for subs in results.substs.iter() {
        //     //     println!("{:?}", subs);
        //     // }
        //     // if !boolean {
        //     //     result = true;
        //     //     proved_goal_index = goal_index;
        //     //     break;
        //     // }
        // }
        let extraction_time = now.elapsed().as_secs_f32();

        best_expr = Some(best_exprr.to_string());

        if report {
            println!("{}\n", "Could not prove any goal:".bright_red().bold(),);
            println!(
                "Best Expr: {}",
                format!("{}", best_exprr).bright_green().bold()
            );
            println!(
                "{} {}",
                "Extracting Best Expression took:".bright_red(),
                extraction_time.to_string().bright_green()
            );
        }
    }
    let total_time: f64 = runner.iterations.iter().map(|i| i.total_time).sum();
    if report {
        runner.print_report();
    }

    let stop_reason = match runner.stop_reason.unwrap() {
        StopReason::Saturated => "Saturation".to_string(),
        StopReason::IterationLimit(iter) => format!("Iterations: {}", iter),
        StopReason::NodeLimit(nodes) => format!("Node Limit: {}", nodes),
        StopReason::TimeLimit(time) => format!("Time Limit : {}", time),
        StopReason::Other(reason) => reason,
    };

    ResultStructure::new(
        index,
        start_expression.to_string(),
        "1/0".to_string(),
        result,
        best_expr.unwrap_or_default(),
        ruleset_class as i64,
        runner.iterations.len(),
        runner.egraph.total_number_of_nodes(),
        runner.iterations.iter().map(|i| i.n_rebuilds).sum(),
        total_time,
        stop_reason,
        None,
    )
}

#[allow(dead_code)]
pub fn prove_rule(
    rule: &Rule,
    ruleset_class: i8,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> ResultStructure {
    let mut result = prove_equiv(
        &rule.lhs,
        &rule.rhs,
        ruleset_class,
        params,
        use_iteration_check,
        report,
    );
    result.add_index_condition(rule.index, rule.condition.as_ref().unwrap().clone());
    result
}

pub fn prove_expression_with_file_classes(
    classes: &JsonValue,
    params: (usize, usize, u64),
    index: i16,
    start_expression: &str,
    use_iteration_check: bool,
    report: bool,
) -> Result<(ResultStructure, i64, Duration), Box<dyn Error>> {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    // let end: Pattern<Math> = end_expressions.parse().unwrap();
    let mut result: bool = false;
    let mut runner: egg::Runner<Math, ConstantFold>;
    let mut rules: Vec<Rewrite>;
    let mut proved_goal_index = 0;
    let id;
    let mut best_expr = Some("".to_string());
    let mut proving_class = -1;
    // First iter
    let end_1: Pattern<Math> = "1".parse().unwrap();
    let end_0: Pattern<Math> = "0".parse().unwrap();
    let goals = [end_0.clone(), end_1.clone()];
    let mut total_time: f64 = 0.0;

    let time_per_class = (params.2 as f64) / (classes.len() as f64);

    // rules = filtered_rules(&classes[0])?;
    let start_t = Instant::now();
    runner = Runner::default()
        .with_iter_limit(params.0)
        .with_node_limit(params.1)
        .with_time_limit(Duration::from_secs_f64(time_per_class))
        .with_expr(&start);
    id = runner.egraph.find(*runner.roots.last().unwrap());
    // End first iter
    for (i, class) in classes.members().enumerate() {
        rules = filtered_rules(class)?;
        if i > 0 {
            runner = Runner::default()
                .with_iter_limit(params.0)
                .with_node_limit(params.1)
                .with_time_limit(Duration::from_secs_f64(time_per_class))
                .with_egraph(runner.egraph)
        }

        if use_iteration_check {
            runner = runner.run_check_iteration_id(rules.iter(), &goals, id);
        } else {
            runner = runner.run(rules.iter());
        }
        let class_time: f64 = runner.iterations.iter().map(|i| i.total_time).sum();
        total_time += class_time;

        for (goal_index, goal) in goals.iter().enumerate() {
            let boolean = (goal.search_eclass(&runner.egraph, id)).is_none();
            if !boolean {
                result = true;
                proved_goal_index = goal_index;
                break;
            }
        }

        if result {
            if report {
                println!(
                    "{}\n{:?}\n class {}",
                    "Proved goal:".bright_green().bold(),
                    goals[proved_goal_index].to_string(),
                    i
                );
            }
            best_expr = Some(goals[proved_goal_index].to_string())
        } else {
            let mut extractor = Extractor::new(&runner.egraph, AstDepth);
            // We want to extract the best expression represented in the
            // same e-class as our initial expression, not from the whole e-graph.
            // Luckily the runner stores the eclass Id where we put the initial expression.
            let (_, best_exprr) = extractor.find_best(id);
            best_expr = Some(best_exprr.to_string());

            if report {
                println!("{}\n", "Could not prove any goal:".bright_red().bold(),);
                println!(
                    "Best Expr: {}",
                    format!("{}", best_exprr).bright_green().bold()
                );
            }
        }
        if report {
            runner.print_report();
            println!(
                "Execution took: {}\n",
                format!("{} s", total_time).bright_green().bold()
            );
        }
        if result {
            proving_class = i as i64;
            break;
        }
    }

    let stop_reason = match runner.stop_reason.unwrap() {
        StopReason::Saturated => "Saturation".to_string(),
        StopReason::IterationLimit(iter) => format!("Iterations: {}", iter),
        StopReason::NodeLimit(nodes) => format!("Node Limit: {}", nodes),
        StopReason::TimeLimit(time) => format!("Time Limit : {}", time),
        StopReason::Other(reason) => reason,
    };
    // // comparing prove to the prove_file_classes with all rules
    // prove(start_expression, -1, params, use_iteration_check, report);

    let result_struct = ResultStructure::new(
        index,
        start_expression.to_string(),
        "1/0".to_string(),
        result,
        best_expr.unwrap_or_default(),
        proving_class as i64,
        runner.iterations.len(),
        runner.egraph.total_number_of_nodes(),
        runner.iterations.iter().map(|i| i.n_rebuilds).sum(),
        total_time,
        stop_reason,
        None,
    );
    Ok((result_struct, proving_class, start_t.elapsed()))
}

#[allow(dead_code)]
pub fn fast_fail_prove_old(
    index: i16,
    start_expression: &str,
    ruleset_class: i8,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> ResultStructure {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end_1: Pattern<Math> = "1".parse().unwrap();
    let end_0: Pattern<Math> = "0".parse().unwrap();
    let goals = [end_0.clone(), end_1.clone()];
    let impossible: Pattern<Math> = "(== ?a ?c)".parse().unwrap();
    let impossibles = [impossible];
    let mut runner: Runner<Math, ConstantFold>;
    let mut result = false;
    let mut proved_goal_index = 0;
    let mut id;
    let best_expr;
    let mut total_time: f64 = 0.0;
    let threshold = 0.5;

    let mut time_limit = if (params.2 as f64) < threshold {
        params.2 as f64
    } else {
        threshold
    };

    if report {
        println!(
            "\n====================================\nProving Expression:\n {}\n",
            start_expression
        )
    }
    if use_iteration_check {
        runner = Runner::default()
            .with_iter_limit(params.0)
            .with_node_limit(params.1)
            .with_time_limit(Duration::from_secs_f64(time_limit))
            .with_expr(&start)
            .run_check_iteration(rules(ruleset_class).iter(), &goals);
    } else {
        runner = Runner::default()
            .with_iter_limit(params.0)
            .with_node_limit(params.1)
            .with_time_limit(Duration::from_secs_f64(time_limit))
            .with_expr(&start)
            .run(rules(ruleset_class).iter());
    }

    id = runner.egraph.find(*runner.roots.last().unwrap());
    for (goal_index, goal) in goals.iter().enumerate() {
        let boolean = (goal.search_eclass(&runner.egraph, id)).is_none();
        if !boolean {
            result = true;
            proved_goal_index = goal_index;
            break;
        }
    }

    if result {
        if report {
            println!(
                "{}\n{:?}",
                "Proved goal:".bright_green().bold(),
                goals[proved_goal_index].to_string()
            );
        }
        best_expr = Some(goals[proved_goal_index].to_string());
    } else {
        let mut extractor = Extractor::new(&runner.egraph, AstDepth);
        let now = Instant::now();
        let (_, best_exprr) = extractor.find_best(id);
        // let mut egraph = EGraph::default();
        // let a11 = egraph.add_expr(&best_exprr.to_string().parse().unwrap());
        // egraph.rebuild();
        // for (impo_index, impo) in impossibles.iter().enumerate() {
        //     let results = impo.search_eclass(&egraph, a11).unwrap();
        //     // for result in results {}
        //     let a: Var = "?a".parse().unwrap();
        //     let c: Var = "?c".parse().unwrap();
        //     println!("{:?}", egraph[results.substs[0][a]].nodes);
        //     println!(
        //         "{:?}",
        //         egraph[results.substs[0][a]].nodes.iter().any(|n| match n {
        //             Math::Symbol(_) => true,
        //             _ => return false,
        //         }) && egraph[results.substs[0][c]].nodes.iter().all(|n| match n {
        //             Math::Symbol(_) => false,
        //             _ => return true,
        //         })
        //     )
        //     // for subs in results.substs.iter() {
        //     //     println!("{:?}", subs);
        //     // }
        //     // if !boolean {
        //     //     result = true;
        //     //     proved_goal_index = goal_index;
        //     //     break;
        //     // }
        // }
        let extraction_time = now.elapsed().as_secs_f32();

        let first_exec_time: f64 = runner.iterations.iter().map(|i| i.total_time).sum();
        total_time += first_exec_time;
        if match &runner.stop_reason.as_ref().unwrap() {
            StopReason::Saturated => false,
            _ => true,
        } {
            println!(
                "Starting 2nd pass with Best Expr: {} in {}",
                format!("{}", best_exprr).bright_green().bold(),
                format!("{}", extraction_time).bright_green().bold()
            );
            if use_iteration_check {
                runner = Runner::default()
                    .with_iter_limit(params.0)
                    .with_node_limit(params.1)
                    .with_time_limit(Duration::from_secs_f64((params.2 as f64) - threshold))
                    .with_expr(&best_exprr)
                    .run_check_iteration(rules(ruleset_class).iter(), &goals);
            } else {
                runner = Runner::default()
                    .with_iter_limit(params.0)
                    .with_node_limit(params.1)
                    .with_time_limit(Duration::from_secs_f64((params.2 as f64) - threshold))
                    .with_expr(&best_exprr)
                    .run(rules(ruleset_class).iter());
            }
        }

        id = runner.egraph.find(*runner.roots.last().unwrap());
        for (goal_index, goal) in goals.iter().enumerate() {
            let boolean = (goal.search_eclass(&runner.egraph, id)).is_none();
            if !boolean {
                result = true;
                proved_goal_index = goal_index;
                break;
            }
        }

        if result {
            if report {
                println!(
                    "{}\n{:?}",
                    "Proved goal:".bright_green().bold(),
                    goals[proved_goal_index].to_string()
                );
            }
            best_expr = Some(goals[proved_goal_index].to_string());
        } else {
            let mut extractor = Extractor::new(&runner.egraph, AstDepth);
            let now = Instant::now();
            let (_, best_exprr) = extractor.find_best(id);
            // let mut egraph = EGraph::default();
            // let a11 = egraph.add_expr(&best_exprr.to_string().parse().unwrap());
            // egraph.rebuild();
            // for (impo_index, impo) in impossibles.iter().enumerate() {
            //     let results = impo.search_eclass(&egraph, a11).unwrap();
            //     // for result in results {}
            //     let a: Var = "?a".parse().unwrap();
            //     let c: Var = "?c".parse().unwrap();
            //     println!("{:?}", egraph[results.substs[0][a]].nodes);
            //     println!(
            //         "{:?}",
            //         egraph[results.substs[0][a]].nodes.iter().any(|n| match n {
            //             Math::Symbol(_) => true,
            //             _ => return false,
            //         }) && egraph[results.substs[0][c]].nodes.iter().all(|n| match n {
            //             Math::Symbol(_) => false,
            //             _ => return true,
            //         })
            //     )
            //     // for subs in results.substs.iter() {
            //     //     println!("{:?}", subs);
            //     // }
            //     // if !boolean {
            //     //     result = true;
            //     //     proved_goal_index = goal_index;
            //     //     break;
            //     // }
            // }
            let extraction_time = now.elapsed().as_secs_f32();

            best_expr = Some(best_exprr.to_string());
            if report {
                println!("{}\n", "Could not prove any goal:".bright_red().bold(),);
                println!(
                    "Best Expr: {}",
                    format!("{}", best_exprr).bright_green().bold()
                );
                println!(
                    "{} {}",
                    "Extracting Best Expression took:".bright_red(),
                    extraction_time.to_string().bright_green()
                );
            }
        }
    }
    let second_exec_time: f64 = runner.iterations.iter().map(|i| i.total_time).sum();
    total_time += second_exec_time;
    if report {
        runner.print_report();
    }

    let stop_reason = match runner.stop_reason.unwrap() {
        StopReason::Saturated => "Saturation".to_string(),
        StopReason::IterationLimit(iter) => format!("Iterations: {}", iter),
        StopReason::NodeLimit(nodes) => format!("Node Limit: {}", nodes),
        StopReason::TimeLimit(time) => format!("Time Limit : {}", time),
        StopReason::Other(reason) => reason,
    };

    ResultStructure::new(
        index,
        start_expression.to_string(),
        "1/0".to_string(),
        result,
        best_expr.unwrap_or_default(),
        ruleset_class as i64,
        runner.iterations.len(),
        runner.egraph.total_number_of_nodes(),
        runner.iterations.iter().map(|i| i.n_rebuilds).sum(),
        total_time,
        stop_reason,
        None,
    )
}

#[allow(dead_code)]
pub fn prove_multiple_passes(
    index: i16,
    start_expression: &str,
    ruleset_class: i8,
    params: (usize, usize, u64),
    threshold: f64,
    use_iteration_check: bool,
    report: bool,
) -> ResultStructure {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end_1: Pattern<Math> = "1".parse().unwrap();
    let end_0: Pattern<Math> = "0".parse().unwrap();
    let goals = [end_0.clone(), end_1.clone()];
    let mut runner: Runner<Math, ConstantFold>;
    let mut result = false;
    let mut proved_goal_index = 0;
    let mut id;
    let best_expr;
    let mut total_time: f64 = 0.0;
    let nbr_passes = (params.2 as f64) / threshold;

    if report {
        println!(
            "\n====================================\nProving Expression:\n {}\n",
            start_expression
        )
    }

    let mut i = 0.0;
    let mut exit = false;
    let mut expr = start;
    let mut runner = Runner::default()
        .with_iter_limit(params.0)
        .with_node_limit(params.1)
        .with_time_limit(Duration::from_secs_f64((params.2 as f64) - threshold))
        .with_expr(&expr);
    id = runner.egraph.find(*runner.roots.last().unwrap());
    while !exit {
        if i > 0.0 {
            let mut extractor;
            extractor = Extractor::new(&((&runner).egraph), AstDepth);
            let now = Instant::now();
            let (_, best_exprr) = extractor.find_best(id);
            let extraction_time = now.elapsed().as_secs_f64();
            expr = best_exprr;
            total_time += extraction_time;
            if report {
                println!(
                    "Starting pass {} with Expr: {} in {}",
                    i,
                    format!("{}", expr).bright_green().bold(),
                    format!("{}", extraction_time).bright_green().bold()
                );
            }
        }

        if use_iteration_check {
            runner = Runner::default()
                .with_iter_limit(params.0)
                .with_node_limit(params.1)
                .with_time_limit(Duration::from_secs_f64(threshold))
                .with_expr(&expr)
                .run_check_iteration(rules(ruleset_class).iter(), &goals);
        } else {
            runner = Runner::default()
                .with_iter_limit(params.0)
                .with_node_limit(params.1)
                .with_time_limit(Duration::from_secs_f64(threshold))
                .with_expr(&expr)
                .run(rules(ruleset_class).iter());
        }

        id = runner.egraph.find(*runner.roots.last().unwrap());
        for (goal_index, goal) in goals.iter().enumerate() {
            let boolean = (goal.search_eclass(&runner.egraph, id)).is_none();
            if !boolean {
                result = true;
                proved_goal_index = goal_index;
                break;
            }
        }

        let saturated = match &runner.stop_reason.as_ref().unwrap() {
            StopReason::Saturated => true,
            _ => false,
        };
        let exec_time: f64 = runner.iterations.iter().map(|i| i.total_time).sum();
        total_time += exec_time;
        if saturated || i > (nbr_passes - 1.0) || result {
            exit = true;
        } else {
            i += 1.0;
        }
    }
    if result {
        if report {
            println!(
                "{}\n{:?}",
                "Proved goal:".bright_green().bold(),
                goals[proved_goal_index].to_string()
            );
        }
        best_expr = Some(goals[proved_goal_index].to_string());
    } else {
        let mut extractor = Extractor::new(&runner.egraph, AstDepth);
        let now = Instant::now();
        let (_, best_exprr) = extractor.find_best(id);
        let extraction_time = now.elapsed().as_secs_f32();

        best_expr = Some(best_exprr.to_string());
        if report {
            println!("{}\n", "Could not prove any goal:".bright_red().bold(),);
            println!(
                "Best Expr: {}",
                format!("{}", best_exprr).bright_green().bold()
            );
            println!(
                "{} {}",
                "Extracting Best Expression took:".bright_red(),
                extraction_time.to_string().bright_green()
            );
        }
    }
    if report {
        runner.print_report();
    }

    let stop_reason = match runner.stop_reason.unwrap() {
        StopReason::Saturated => "Saturation".to_string(),
        StopReason::IterationLimit(iter) => format!("Iterations: {}", iter),
        StopReason::NodeLimit(nodes) => format!("Node Limit: {}", nodes),
        StopReason::TimeLimit(time) => format!("Time Limit : {}", time),
        StopReason::Other(reason) => reason,
    };

    ResultStructure::new(
        index,
        start_expression.to_string(),
        "1/0".to_string(),
        result,
        best_expr.unwrap_or_default(),
        ruleset_class as i64,
        runner.iterations.len(),
        runner.egraph.total_number_of_nodes(),
        runner.iterations.iter().map(|i| i.n_rebuilds).sum(),
        total_time,
        stop_reason,
        None,
    )
}
