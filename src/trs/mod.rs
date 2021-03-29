// use ordered_float::NotNan;
use std::{cmp::Ordering, time::Instant, fmt};
use std::ops::Add;
use std::time::Duration;

use colored::*;
use egg::{*};
use num_traits::cast::ToPrimitive;

use crate::structs::{ExpressionStruct, ResultStructure, Rule};
use std::num::ParseIntError;
use std::str::FromStr;

pub type EGraph = egg::EGraph<Math, ConstantFold>;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub type Constant = i64;
pub type Boolean = bool;


#[derive(Debug, Clone, Eq, Ord, Hash)]
pub enum TRSDATA {
    Constant(i64),
    Boolean(bool),
}

impl Add for TRSDATA {
    type Output = Option<TRSDATA>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            TRSDATA::Constant(a) => {
                match rhs {
                    TRSDATA::Constant(b) => {
                        Some(TRSDATA::Constant(a + b))
                    }
                    _ => None
                }
            }
            _ => None
        }
    }
}

impl Add for &TRSDATA {
    type Output = Option<TRSDATA>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            TRSDATA::Constant(a) => {
                match rhs {
                    TRSDATA::Constant(b) => {
                        Some(TRSDATA::Constant(*a + *b))
                    }
                    _ => None
                }
            }
            _ => None
        }
    }
}

impl PartialEq for TRSDATA {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => {
                match other {
                    TRSDATA::Constant(b) => a == b,
                    TRSDATA::Boolean(_) => false
                }
            }
            TRSDATA::Boolean(a) => {
                match other {
                    TRSDATA::Constant(_) => false,
                    TRSDATA::Boolean(b) => a == b
                }
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => {
                match other {
                    TRSDATA::Constant(b) => a != b,
                    TRSDATA::Boolean(_) => false
                }
            }
            TRSDATA::Boolean(a) => {
                match other {
                    TRSDATA::Constant(_) => false,
                    TRSDATA::Boolean(b) => a != b
                }
            }
        }
    }
}

impl PartialOrd for TRSDATA{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            TRSDATA::Constant(a) => {
                match other {
                    TRSDATA::Constant(b) => a.partial_cmp(b),
                    TRSDATA::Boolean(_) => None
                }
            }
            TRSDATA::Boolean(a) => {
                match other {
                    TRSDATA::Constant(_) => None,
                    TRSDATA::Boolean(b) => a.partial_cmp(b)
                }
            }
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => {
                match other {
                    TRSDATA::Constant(b) => a < b,
                    _ => false
                }
            },
            _ => false
        }
    }

    fn le(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => {
                match other {
                    TRSDATA::Constant(b) => a <= b,
                    _ => false
                }
            },
            _ => false
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => {
                match other {
                    TRSDATA::Constant(b) => a > b,
                    _ => false
                }
            },
            _ => false
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match self {
            TRSDATA::Constant(a) => {
                match other {
                    TRSDATA::Constant(b) => a >= b,
                    _ => false
                }
            },
            _ => false
        }
    }
}

impl fmt::Display for TRSDATA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            TRSDATA::Boolean(b) => write!(f, "{:?}", b),
            TRSDATA::Constant(constant) => write!(f, "{:?}", constant)
        }
    }
}

impl FromStr for TRSDATA {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s {
            "true" => Ok(TRSDATA::Boolean(true)),
            "false" => Ok(TRSDATA::Boolean(false)),
            _ => Ok(TRSDATA::Constant(s.parse::<i64>()?))
        }
    }
}

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
        "==" = Equ([Id; 2]),
        "!=" = IEq([Id; 2]),
        "||" = Or([Id; 2]),
        "&&" = And([Id; 2]),
        Constant(TRSDATA),
        Symbol(Symbol),
    }
}

#[derive(Default)]
#[derive(Clone)]
pub struct ConstantFold;

impl Analysis<Math> for ConstantFold {
    type Data = Option<TRSDATA>;

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
            Math::Constant(c) => (*c).clone(),
            Math::Add([a, b]) => (x(a)? + x(b)?).unwrap(),
            // Math::Sub([a, b]) => x(a)? - x(b)?,
            // Math::Mul([a, b]) => x(a)? * x(b)?,
            // Math::Div([a, b]) if x(b) != Some(0) => (x(a)?.to_i64().unwrap() / x(b)?.to_i64().unwrap()),
            // //Math::Div([a, b]) if x(b) != Some(0.0.into()) => x(a)? / x(b)?,
            // Math::Max([a, b]) => std::cmp::max(x(a)?, x(b)?),
            // Math::Min([a, b]) => std::cmp::min(x(a)?, x(b)?),
            // Math::Not(a) => if x(a)? == 0  {
            //     1
            // } else {
            //     0
            // },
            //
            // Math::Lt([a, b]) => if x(a)? < x(b)?  {
            //     1
            // } else {
            //     0
            // },
            //
            // Math::Gt([a, b]) => if x(a)? > x(b)?  {
            //     1
            // } else {
            //     0
            // },
            //
            // Math::Let([a, b]) => if x(a)? <= x(b)?  {
            //     1
            // } else {
            //     0
            // },
            //
            // Math::Get([a, b]) => if x(a)? >= x(b)?  {
            //     1
            // } else {
            //     0
            // },
            //
            // Math::Mod([a, b]) => {
            //     if x(b)? == 0 {
            //         0
            //     } else {
            //         x(a)? % x(b)?
            //     }
            // }
            //
            // Math::Eq([a, b]) => if x(a)? == x(b)? {
            //     1
            // } else {
            //     0
            // },
            //
            // Math::IEq([a, b]) => if x(a)? == x(b)? {
            //     0
            // } else {
            //     1
            // },
            //
            // Math::And([a, b]) =>
            //     if x(a)? == 0
            //         || x(b)? == 0
            //     {
            //         0
            //     } else {
            //         1
            //     },
            //
            // Math::Or([a, b]) =>
            //     if x(a)? == 1
            //         || x(b)? == 1
            //     {
            //         1
            //     } else {
            //         0
            //     },

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
    move |egraph, _, subst| {
        egraph[subst[var]].nodes.iter().any(|n| match n {
            Math::Constant(c) => match *c {
                TRSDATA::Constant(c_v) => c_v > 0,
                _ => false
            },
            _ => return false,
        })
    }
}

pub fn is_const_neg(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var]].nodes.iter().any(|n| match n {
            Math::Constant(c) => match *c {
                TRSDATA::Constant(c_v) => c_v < 0,
                _ => false
            },
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
    let zero = Math::Constant(TRSDATA::Constant(0));
    move |egraph, _, subst| !egraph[subst[var]].nodes.contains(&zero)
}

// pub fn are_less_eq(var: &str, var1: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
//     let var: Var = var.parse().unwrap();
//     let var1: Var = var1.parse().unwrap();
//     move |egraph, _, subst| {
//         egraph[subst[var1]].nodes.iter().any(|n| match n {
//             Math::Constant(c) => {
//                 egraph[subst[var]].nodes.iter().any(|n1| match n1 {
//                     Math::Constant(c1) => (c1.cmp(c) == Ordering::Less) || (c1.cmp(c) == Ordering::Equal),
//                     _ => return false,
//                 })
//             }
//             _ => return false,
//         })
//     }
// }
//
// // return true if v <= | v1 |
// pub fn are_less_eq_absolute(var: &str, var1: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
//     let var: Var = var.parse().unwrap();
//     let var1: Var = var1.parse().unwrap();
//     move |egraph, _, subst| {
//         egraph[subst[var1]].nodes.iter().any(|n| match n {
//             Math::Constant(c) => {
//                 egraph[subst[var]].nodes.iter().any(|n1| match n1 {
//                     Math::Constant(c1) => (c1 <= &c.abs()),
//                     _ => return false,
//                 })
//             }
//             _ => return false,
//         })
//     }
// }

pub fn compare_c0_c1(var: &str, var1: &str, comp: &'static str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var: Var = var.parse().unwrap();
    let var1: Var = var1.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var1]].nodes.iter().any(|n1| match n1 {
            Math::Constant(c1_d) => {
                match *c1_d {
                    TRSDATA::Constant(c1) => egraph[subst[var]].nodes.iter().any(|n| match n {
                        Math::Constant(c_d) => {
                            match *c_d {
                                TRSDATA::Constant(c) => match comp {
                                    "<" => {
                                        c < c1
                                    }
                                    "<a" => {
                                        c < c1.abs()
                                    }
                                    "<=" => {
                                        c <= c1
                                    }
                                    "<=+1" => {
                                        c <= c1 + 1
                                    }
                                    "<=a" => {
                                        c <= c1.abs()
                                    }
                                    "<=-a" => {
                                        c <= -c1.abs()
                                    }
                                    "<=-a+1" => {
                                        c <= 1 - c1.abs()
                                    }
                                    ">" => {
                                        c > c1
                                    }
                                    ">a" => {
                                        c > c1.abs()
                                    }
                                    ">=" => {
                                        c >= c1
                                    }
                                    ">=a" => {
                                        c >= (c1.abs())
                                    }
                                    ">=a-1" => {
                                        c >= (c1.abs() - 1)
                                    }
                                    "!=" => {
                                        c != c1
                                    }
                                    _ => false
                                },
                                _ => false
                            }
                        }
                        _ => return false,
                    }),
                    _ => false
                }
            }
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
pub fn prove_equiv(start_expression: &str, end_expressions: &str, ruleset_class: i8, params: (usize, usize, u64), use_iteration_check: bool, report: bool) -> (bool, f64, Option<String>) {
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
pub fn prove(start_expression: &str, ruleset_class: i8, params: (usize, usize, u64), use_iteration_check: bool, report: bool) -> (bool, f64, Option<String>) {
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
        println!("\n==================================\nProving Expression:\n {}\n", start_expression)
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
                "Proved goal:".bright_green().bold(), goals[proved_goal_index].to_string()
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
            println!(
                "{}\n",
                "Could not prove any goal:".bright_red().bold(),
            );
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
pub fn prove_all_classes(start_expression: &str, end_expressions: &str, start_class: i8, report: bool) -> bool {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end: Pattern<Math> = end_expressions.parse().unwrap();
    let result: bool = false;
    let mut i = start_class;

    let start_t = Instant::now();
    let mut runner = Runner::default().with_expr(&start).run(rules(start_class).iter());
    let id = runner.egraph.find(*runner.roots.last().unwrap());

    while (!result) && (i < 3) {
        let start_t1 = Instant::now();
        if i > start_class {
            runner = Runner::default().with_egraph(runner.egraph).run(rules(i).iter());
        }
        if report {
            println!("Time elapsed from start is: {:?}, just for this class: {:?}", start_t.elapsed(), start_t1.elapsed());
        }
        let matches = end.search_eclass(&runner.egraph, id);
        if matches.is_none() {
            println!("{} {} {}", "Class".bright_red(), i, "didn't work".bright_red());
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
pub fn prove_rule(rule: &Rule, ruleset_class: i8, params: (usize, usize, u64), use_iteration_check: bool, report: bool) -> ResultStructure {
    let (result, total_time, best_expr) = prove_equiv(&rule.lhs, &rule.rhs, ruleset_class, params, use_iteration_check, report);
    let best_expr_string = match best_expr {
        Some(expr) => expr,
        None => "".to_string()
    };
    ResultStructure::new(rule.index, rule.lhs.clone(), rule.rhs.clone(), result, best_expr_string, total_time, rule.condition.clone())
}

pub fn prove_expr(expression: &ExpressionStruct, ruleset_class: i8, params: (usize, usize, u64), use_iteration_check: bool, report: bool) -> ResultStructure {
    let (result, total_time, best_expr) = prove(&(expression.expression)[..], ruleset_class, params, use_iteration_check, report);
    let best_expr_string = match best_expr {
        Some(expr) => expr,
        None => "".to_string()
    };
    ResultStructure::new(expression.index, expression.expression.clone(), "1/0".to_string(), result, best_expr_string, total_time, None)
}
