use crate::structs::{ExpressionStruct, ResultStructure, Rule};
use colored::*;
use egg::*;
use num_traits::cast::ToPrimitive;
use ordered_float::NotNan;
use std::time::Duration;
use std::{cmp::Ordering, time::Instant};
use std::ffi::OsString;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub type EGraph = egg::EGraph<Math, ConstantFold>;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub type Constant = NotNan<f64>;
pub type Boolean = bool;

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
        Constant(Constant),
        Boolean(Boolean),
        Symbol(Symbol),
    }
}

#[derive(Default, Clone)]
pub struct ConstantFold;

impl Analysis<Math> for ConstantFold {
    type Data = Option<Constant>;

    fn merge(&self, a: &mut Self::Data, b: Self::Data) -> Option<Ordering> {
        match (a.as_mut(), b) {
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
        let x = |i: &Id| egraph[*i].data;
        Some(match enode {
            Math::Constant(c) => *c,
            Math::Add([a, b]) => x(a)? + x(b)?,
            Math::Sub([a, b]) => x(a)? - x(b)?,
            Math::Mul([a, b]) => x(a)? * x(b)?,
            Math::Div([a, b]) if x(b) != Some(0.0.into()) => {
                NotNan::from((x(a)?.to_i64().unwrap() / x(b)?.to_i64().unwrap()) as f64)
            }
            //Math::Div([a, b]) if x(b) != Some(0.0.into()) => x(a)? / x(b)?,
            Math::Max([a, b]) => std::cmp::max(x(a)?, x(b)?),
            Math::Min([a, b]) => std::cmp::min(x(a)?, x(b)?),
            Math::Not(a) => NotNan::new(if x(a)?.cmp(&NotNan::from(0.0)) == Ordering::Equal {
                1.0
            } else {
                0.0
            })
            .unwrap(),

            Math::Lt([a, b]) => NotNan::new(if x(a)?.cmp(&x(b)?) == Ordering::Less {
                1.0
            } else {
                0.0
            })
            .unwrap(),

            Math::Gt([a, b]) => NotNan::new(if x(a)?.cmp(&x(b)?) == Ordering::Greater {
                1.0
            } else {
                0.0
            })
            .unwrap(),

            Math::Let([a, b]) => NotNan::new(
                if x(a)?.cmp(&x(b)?) == Ordering::Less || x(a)?.cmp(&x(b)?) == Ordering::Equal {
                    1.0
                } else {
                    0.0
                },
            )
            .unwrap(),

            Math::Get([a, b]) => NotNan::new(
                if x(a)?.cmp(&x(b)?) == Ordering::Greater || x(a)?.cmp(&x(b)?) == Ordering::Equal {
                    1.0
                } else {
                    0.0
                },
            )
            .unwrap(),

            Math::Mod([a, b]) => {
                if x(b)? == NotNan::from(0.0) {
                    NotNan::from(0.0)
                } else {
                    x(a)? % x(b)?
                }
            }

            Math::Eq([a, b]) => NotNan::new(if x(a)?.cmp(&x(b)?) == Ordering::Equal {
                1.0
            } else {
                0.0
            })
            .unwrap(),

            Math::IEq([a, b]) => NotNan::new(if x(a)?.cmp(&x(b)?) == Ordering::Equal {
                0.0
            } else {
                1.0
            })
            .unwrap(),

            Math::And([a, b]) => NotNan::new(
                if x(a)?.cmp(&NotNan::from(0.0)) == Ordering::Equal
                    || x(b)?.cmp(&NotNan::from(0.0)) == Ordering::Equal
                {
                    0.0
                } else {
                    1.0
                },
            )
            .unwrap(),

            Math::Or([a, b]) => NotNan::new(
                if x(a)?.cmp(&NotNan::from(1.0)) == Ordering::Equal
                    || x(b)?.cmp(&NotNan::from(1.0)) == Ordering::Equal
                {
                    1.0
                } else {
                    0.0
                },
            )
            .unwrap(),

            _ => return None,
        })
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        let class = &mut egraph[id];
        if let Some(c) = class.data {
            let added = egraph.add(Math::Constant(c));
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

#[allow(dead_code)]
pub fn is_const_or_distinct_var(v: &str, w: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let v = v.parse().unwrap();
    let w = w.parse().unwrap();
    move |egraph, _, subst| {
        egraph.find(subst[v]) != egraph.find(subst[w])
            && egraph[subst[v]]
                .nodes
                .iter()
                .any(|n| matches!(n, Math::Constant(..) | Math::Symbol(..)))
    }
}

pub fn is_const_pos(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    let zero = NotNan::from(0.0);
    move |egraph, _, subst| {
        egraph[subst[var]].nodes.iter().any(|n| match n {
            Math::Constant(c) => c.cmp(&zero) == Ordering::Greater,
            _ => return false,
        })
    }
}

pub fn is_const_neg(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    let zero = NotNan::from(0.0);
    move |egraph, _, subst| {
        egraph[subst[var]].nodes.iter().any(|n| match n {
            Math::Constant(c) => c.cmp(&zero) == Ordering::Less,
            _ => return false,
        })
    }
}

#[allow(dead_code)]
pub fn is_const(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var]]
            .nodes
            .iter()
            .any(|n| matches!(n, Math::Constant(..)))
    }
}

#[allow(dead_code)]
pub fn is_sym(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var]]
            .nodes
            .iter()
            .any(|n| matches!(n, Math::Symbol(..)))
    }
}

pub fn is_not_zero(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    let zero = Math::Constant(0.0.into());
    move |egraph, _, subst| !egraph[subst[var]].nodes.contains(&zero)
}

pub fn are_less_eq(var: &str, var1: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var: Var = var.parse().unwrap();
    let var1: Var = var1.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var1]].nodes.iter().any(|n| match n {
            Math::Constant(c) => egraph[subst[var]].nodes.iter().any(|n1| match n1 {
                Math::Constant(c1) => {
                    (c1.cmp(c) == Ordering::Less) || (c1.cmp(c) == Ordering::Equal)
                }
                _ => return false,
            }),
            _ => return false,
        })
    }
}

// return true if v <= | v1 |
pub fn are_less_eq_absolute(var: &str, var1: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var: Var = var.parse().unwrap();
    let var1: Var = var1.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var1]].nodes.iter().any(|n| match n {
            Math::Constant(c) => egraph[subst[var]].nodes.iter().any(|n1| match n1 {
                Math::Constant(c1) => (c1.to_f64().unwrap() <= c.abs()),
                _ => return false,
            }),
            _ => return false,
        })
    }
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
                    "<" => c.to_f64().unwrap() < c1.to_f64().unwrap(),
                    "<a" => c.to_f64().unwrap() < c1.abs(),
                    "<=" => c.to_f64().unwrap() <= c1.to_f64().unwrap(),
                    "<=+1" => c.to_f64().unwrap() <= c1.to_f64().unwrap() + 1.0,
                    "<=a" => c.to_f64().unwrap() <= c1.abs(),
                    "<=-a" => c.to_f64().unwrap() <= (-c1.abs()),
                    "<=-a+1" => c.to_f64().unwrap() <= (-c1.abs() + 1.0),
                    ">" => c.to_f64().unwrap() > c1.to_f64().unwrap(),
                    ">a" => c.to_f64().unwrap() > c1.abs(),
                    ">=" => c.to_f64().unwrap() >= c1.to_f64().unwrap(),
                    ">=a" => c.to_f64().unwrap() >= c1.abs(),
                    ">=a-1" => c.to_f64().unwrap() >= (c1.abs() - 1.0),
                    "!=" => c.to_f64().unwrap() != c1.to_f64().unwrap(),
                    _ => false,
                },
                _ => return false,
            }),
            _ => return false,
        })
    }
}

// pub fn sum_is_great_zero_c0_abs(var: &str, var1: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
//     let var: Var = var.parse().unwrap();
//     let var1: Var = var1.parse().unwrap();
//     move |egraph, _, subst| {
//         egraph[subst[var1]].nodes.iter().any(|n| match n {
//             Math::Constant(c) => {
//                 egraph[subst[var]].nodes.iter().any(|n1| match n1 {
//                     Math::Constant(c1) => (c.to_f64().unwrap() >= c1.abs()),
//                     _ => return false,
//                 })
//             }
//             _ => return false,
//         })
//     }
// }

// Takes a JSON array of rules ids and return the vector of their associated Rewrites
pub fn filtered_rules(class: &json::JsonValue) -> Result< Vec<Rewrite>, Box<dyn Error> > {
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
    ].concat();
    let rules_iter = all_rules.into_iter();
    let rules = rules_iter.filter(|rule| class.contains(rule.name()));
    return Ok(rules.collect());
}

pub fn prove_expression_with_file_classes(
    params: (usize, usize, u64),
    index: i16,
    start_expression: &str,
    end_expressions: &str,
    path: &OsString,
    report: bool,
) -> Result<ResultStructure, Box<dyn Error>> {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end: Pattern<Math> = end_expressions.parse().unwrap();
    let mut result: bool = false;
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let classes = json::parse(&s)?;
    let mut runner: egg::Runner<Math, ConstantFold>;
    let mut rules: Vec<Rewrite>;
    let mut matches: Option<egg::SearchMatches>;
    // First iter
    rules = filtered_rules(&classes[0])?;
    let start_t = Instant::now();
    runner = Runner::default()
        .with_iter_limit(params.0)
        .with_node_limit(params.1)
        .with_time_limit(Duration::new(params.2, 0))
        .with_expr(&start)
        .run(rules.iter());
    let id = runner.egraph.find(*runner.roots.last().unwrap());
    // End first iter
    for (i,class) in classes.members().enumerate(){
        rules = filtered_rules(class)?;
        if i > 0 {
            runner = Runner::default()
                .with_iter_limit(params.0)
                .with_node_limit(params.1)
                .with_time_limit(Duration::new(params.2, 0))
                .with_egraph(runner.egraph)
                .run(rules.iter());
        }
        matches = end.search_eclass(&runner.egraph, id);
        let start_t1 = Instant::now();
        if report {
            println!(
                "Time elapsed from start is: {:?}, just for this class: {:?}",
                start_t.elapsed(),
                start_t1.elapsed()
            );
        }
        if matches.is_none() {
            if report {
                println!(
                    "{} {} {}",
                    "Class".bright_red(),
                    i,
                    "didn't work".bright_red()
                );
                runner.print_report();
            }
            println!("======================\n\n");
        } else {
            result = true;
            if report {
                println!(
                    "{}\n{}\n{}",
                    "Proved goal:".bright_green().bold(),
                    end.pretty(40),
                    format!("Class {} worked", i).bright_green().bold()
                );
                runner.print_report();
            }
            break;
        }
    }
    let result_struct = ResultStructure::new(index, start_expression.to_string(), end_expressions.to_string(), false, "".to_string(), start_t.elapsed().as_secs_f64(), Some(0.to_string()));
    return Ok(result_struct);
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
pub fn print_graph(egraph: &EGraph) {
    println!("printing graph to svg");
    egraph.dot().to_svg("foo.svg").unwrap();
    println!("done printing graph to svg");
}

#[allow(dead_code)]
pub fn simplify(start_expression: &str) {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let runner = Runner::default().with_expr(&start).run(rules(-1).iter());
    let id = runner.egraph.find(*runner.roots.last().unwrap());
    let mut extractor = Extractor::new(&runner.egraph, AstSize);
    let (_, best_expr) = extractor.find_best(id);
    println!(
        "Best Expr: {}",
        format!("{}", best_expr).bright_green().bold()
    );
}

#[allow(dead_code)]
pub fn simplify_time(start_expression: &str) {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let runner = Runner::default().with_expr(&start).run(rules(-1).iter());
    let id = runner.egraph.find(*runner.roots.last().unwrap());
    let mut extractor = Extractor::new(&runner.egraph, AstSize);
    let (_, best_expr) = extractor.find_best(id);
    println!(
        "Best Expr: {}",
        format!("{}", best_expr).bright_green().bold()
    );
    let total_time: f64 = runner.iterations.iter().map(|i| i.total_time).sum();
    println!(
        "Execution took: {}\n",
        format!("{} s", total_time).bright_green().bold()
    );
}

#[allow(dead_code)]
pub fn prove_equiv(
    start_expression: &str,
    end_expressions: &str,
    ruleset_class: i8,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> (bool, f64, Option<String>) {
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
        // We want to extract the best expression represented in the
        // same e-class as our initial expression, not from the whole e-graph.
        // Luckily the runner stores the eclass Id where we put the initial expression.
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
        println!(
            "Execution took: {}\n",
            format!("{} s", total_time).bright_green().bold()
        );
    }

    (result, total_time, best_expr_string)
}

#[allow(dead_code)]
pub fn prove(
    start_expression: &str,
    ruleset_class: i8,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> (bool, f64, Option<String>) {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end_1: Pattern<Math> = "1".parse().unwrap();
    let end_0: Pattern<Math> = "0".parse().unwrap();
    let goals = [end_0.clone(), end_1.clone()];
    // That's it! We can run equality saturation now.
    // let runner = Runner::default().with_expr(&start).run(rules(ruleset_class).iter());
    let runner: Runner<Math, ConstantFold>;
    let mut result = false;
    let mut proved_goal_index = 0;
    let id;
    let best_expr;

    if report {
        println!(
            "\n==================================\nProving Expression:\n {}\n",
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
    let total_time: f64 = runner.iterations.iter().map(|i| i.total_time).sum();
    if report {
        runner.print_report();
        println!(
            "Execution took: {}\n",
            format!("{} s", total_time).bright_green().bold()
        );
    }

    (result, total_time, best_expr)
}

//Not yet refactored should be refactored when needed, as their argument might change
#[allow(dead_code)]
pub fn prove_all_classes(
    start_expression: &str,
    end_expressions: &str,
    start_class: i8,
    report: bool,
) -> bool {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end: Pattern<Math> = end_expressions.parse().unwrap();
    let result: bool = false;
    let mut i = start_class;

    let start_t = Instant::now();
    let mut runner = Runner::default()
        .with_expr(&start)
        .run(rules(start_class).iter());
    let id = runner.egraph.find(*runner.roots.last().unwrap());

    while (!result) && (i < 3) {
        let start_t1 = Instant::now();
        if i > start_class {
            runner = Runner::default()
                .with_egraph(runner.egraph)
                .run(rules(i).iter());
        }
        if report {
            println!(
                "Time elapsed from start is: {:?}, just for this class: {:?}",
                start_t.elapsed(),
                start_t1.elapsed()
            );
        }
        let matches = end.search_eclass(&runner.egraph, id);
        if matches.is_none() {
            println!(
                "{} {} {}",
                "Class".bright_red(),
                i,
                "didn't work".bright_red()
            );
            if report {
                runner.print_report();
            }
            println!("======================\n\n");
            i += 1;
        } else {
            println!(
                "{}\n{}\n{}",
                "Proved goal:".bright_green().bold(),
                end.pretty(40),
                format!("Class {} worked", i).bright_green().bold()
            );
            if report {
                runner.print_report();
            }
            i += 1;
            // result = true;
        }
    }
    result
}

#[allow(dead_code)]
pub fn prove_rule(
    rule: &Rule,
    ruleset_class: i8,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> ResultStructure {
    let (result, total_time, best_expr) = prove_equiv(
        &rule.lhs,
        &rule.rhs,
        ruleset_class,
        params,
        use_iteration_check,
        report,
    );
    let best_expr_string = match best_expr {
        Some(expr) => expr,
        None => "".to_string(),
    };
    ResultStructure::new(
        rule.index,
        rule.lhs.clone(),
        rule.rhs.clone(),
        result,
        best_expr_string,
        total_time,
        rule.condition.clone(),
    )
}

pub fn prove_expr(
    expression: &ExpressionStruct,
    ruleset_class: i8,
    params: (usize, usize, u64),
    use_iteration_check: bool,
    report: bool,
) -> ResultStructure {
    let (result, total_time, best_expr) = prove(
        &(expression.expression)[..],
        ruleset_class,
        params,
        use_iteration_check,
        report,
    );
    let best_expr_string = match best_expr {
        Some(expr) => expr,
        None => "".to_string(),
    };
    ResultStructure::new(
        expression.index,
        expression.expression.clone(),
        "1/0".to_string(),
        result,
        best_expr_string,
        total_time,
        None,
    )
}
