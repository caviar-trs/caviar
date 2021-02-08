use colored::*;
use egg::{*};
use ordered_float::NotNan;
use std::{cmp::Ordering, time::Instant};
use serde::{Serialize, Deserialize};
use num_traits::cast::ToPrimitive;

#[derive(Serialize)]
#[derive(Debug)]
pub struct ResultStructure {
    index: i16,
    start_expression: String,
    end_expressions: String,
    result: bool,
    best_expr: String,
    total_time: f64,
    condition: String,
}

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

#[derive(Default)]
#[derive(Clone)]
pub struct ConstantFold;

impl Analysis<Math> for ConstantFold {
    type Data = Option<Constant>;

    fn merge(&self, to: &mut Self::Data, from: Self::Data) -> bool {
        if let (Some(c1), Some(c2)) = (to.as_ref(), from.as_ref()) {
            assert_eq!(c1, c2);
        }
        merge_if_different(to, to.or(from))
    }

    fn make(egraph: &EGraph, enode: &Math) -> Self::Data {
        let x = |i: &Id| egraph[*i].data;
        Some(match enode {
            Math::Constant(c) => *c,
            Math::Add([a, b]) => x(a)? + x(b)?,
            Math::Sub([a, b]) => x(a)? - x(b)?,
            Math::Mul([a, b]) => x(a)? * x(b)?,
            Math::Div([a, b]) if x(b) != Some(0.0.into()) => NotNan::from((x(a)?.to_i64().unwrap() / x(b)?.to_i64().unwrap()) as f64),
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
            Math::Constant(c) => {
                egraph[subst[var]].nodes.iter().any(|n1| match n1 {
                    Math::Constant(c1) => (c1.cmp(c) == Ordering::Less) || (c1.cmp(c) == Ordering::Equal),
                    _ => return false,
                })
            }
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
            Math::Constant(c) => {
                egraph[subst[var]].nodes.iter().any(|n1| match n1 {
                    Math::Constant(c1) =>
                        {
                            println!("{:?}", [c1,c]);
                        (c1.abs() <= c.abs())
                        }

                    ,
                    _ => return false,
                })
            }
            _ => return false,
        })
    }
}

#[rustfmt::skip]
fn rules() -> Vec<Rewrite> {
    let add_rules = crate::rules::add::add();
    // let and_rules = crate::rules::and::and();
    // let andor_rules = crate::rules::andor::andor();
    // let div_rules = crate::rules::div::div();
    // let eq_rules = crate::rules::eq::eq();
    // let ineq_rules = crate::rules::ineq::ineq();
    // let lt_rules = crate::rules::lt::lt();
    // let max_rules = crate::rules::max::max();
    // let min_rules = crate::rules::min::min();
    // let modulo_rules = crate::rules::modulo::modulo();
    // let mul_rules = crate::rules::mul::mul();
    // let not_rules = crate::rules::not::not();
    // let or_rules = crate::rules::or::or();
    // let sub_rules = crate::rules::sub::sub();
    return [&add_rules[..],
        // &and_rules[..],
        // &andor_rules[..],
        // &div_rules[..],
        // &eq_rules[..],
        // &ineq_rules[..],
        // &lt_rules[..],
        // &max_rules[..],
        // &min_rules[..],
        // &modulo_rules[..],
        // &mul_rules[..],
        // &not_rules[..],
        // &or_rules[..],
        // &sub_rules[..],
    ].concat();
}

#[allow(dead_code)]
pub fn print_graph(egraph: &EGraph) {
    println!("printing graph to svg");
    // create a Dot and then compile it assuming `dot` is on the system
    egraph.dot().to_svg("target/foo.svg").unwrap();
    println!("done printing graph to svg");
}

#[allow(dead_code)]
pub fn prove(start_expression: &str, end_expressions: &str) -> bool {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end: Pattern<Math> = end_expressions.parse().unwrap();
    let runner = Runner::default().with_expr(&start).run(rules().iter());
    let id = runner.egraph.find(*runner.roots.last().unwrap());
    let matches = end.search_eclass(&runner.egraph, id);
    if matches.is_none() {
        println!(
            "{}\n{}\n",
            "Could not prove goal:".bright_red().bold(),
            end.pretty(40),
        );

        let mut extractor = Extractor::new(&runner.egraph, AstSize);

        // We want to extract the best expression represented in the
        // same e-class as our initial expression, not from the whole e-graph.
        // Luckily the runner stores the eclass Id where we put the initial expression.
        let (_, best_expr) = extractor.find_best(id);

        println!(
            "Best Expr: {}",
            format!("{}", best_expr).bright_green().bold()
        );

        false
    } else {
        println!(
            "{}\n{}\n",
            "Proved goal:".bright_green().bold(),
            end.pretty(40)
        );
        true
    }
}

#[allow(dead_code)]
pub fn find_best(start_expression: &str) {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let runner = Runner::default().with_expr(&start).run(rules().iter());
    let id = runner.egraph.find(*runner.roots.last().unwrap());
    let mut extractor = Extractor::new(&runner.egraph, AstSize);

    // We want to extract the best expression represented in the
    // same e-class as our initial expression, not from the whole e-graph.
    // Luckily the runner stores the eclass Id where we put the initial expression.
    let (_, best_expr) = extractor.find_best(id);

    println!(
        "Best Expr: {}",
        format!("{}", best_expr).bright_green().bold()
    );
}

#[allow(dead_code)]
pub fn find_best_time(start_expression: &str) {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let now = Instant::now();
    // That's it! We can run equality saturation now.
    let runner = Runner::default().with_expr(&start).run(rules().iter());
    println!(
        "Saturation took: {}\n",
        format!("{} ms", now.elapsed().as_millis())
            .bright_green()
            .bold()
    );
    let id = runner.egraph.find(*runner.roots.last().unwrap());
    let mut extractor = Extractor::new(&runner.egraph, AstSize);
    // We want to extract the best expression represented in the
    // same e-class as our initial expression, not from the whole e-graph.
    // Luckily the runner stores the eclass Id where we put the initial expression.
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
pub fn prove_time(start_expression: &str, end_expressions: &str) -> bool {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end: Pattern<Math> = end_expressions.parse().unwrap();
    let result: bool;
    // That's it! We can run equality saturation now.
    let runner = Runner::default().with_expr(&start).run(rules().iter());
    let id = runner.egraph.find(*runner.roots.last().unwrap());
    let matches = end.search_eclass(&runner.egraph, id);
    if matches.is_none() {
        println!(
            "{}\n{}\n",
            "Could not prove goal:".bright_red().bold(),
            end.pretty(40),
        );

        let mut extractor = Extractor::new(&runner.egraph, AstDepth);
        // We want to extract the best expression represented in the
        // same e-class as our initial expression, not from the whole e-graph.
        // Luckily the runner stores the eclass Id where we put the initial expression.
        let (_, best_expr) = extractor.find_best(id);

        println!(
            "Best Expr: {}",
            format!("{}", best_expr).bright_green().bold()
        );

        result = false;
    } else {
        println!(
            "{}\n{}\n",
            "Proved goal:".bright_green().bold(),
            end.pretty(40)
        );
        result = true;
    }
    let total_time: f64 = runner.iterations.iter().map(|i| i.total_time).sum();
    println!(
        "Execution took: {}\n",
        format!("{} s", total_time).bright_green().bold()
    );
    result
}


#[allow(dead_code)]
pub fn prove_for_csv(index: i16, start_expression: &str, end_expression: &str, condition: &str) -> ResultStructure {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end: Pattern<Math> = end_expression.parse().unwrap();
    let result: bool;
    let mut best_expr = String::from("");
    // That's it! We can run equality saturation now.
    let runner = Runner::default().with_expr(&start).run(rules().iter());
    let id = runner.egraph.find(*runner.roots.last().unwrap());
    let matches = end.search_eclass(&runner.egraph, id);
    if matches.is_none() {
        println!(
            "{}\n{}\n",
            "Could not prove goal:".bright_red().bold(),
            end.pretty(40),
        );

        let mut extractor = Extractor::new(&runner.egraph, AstDepth);
        // We want to extract the best expression represented in the
        // same e-class as our initial expression, not from the whole e-graph.
        // Luckily the runner stores the eclass Id where we put the initial expression.
        let (_, best_expr_temp) = extractor.find_best(id);
        best_expr = best_expr_temp.to_string();

        println!(
            "Best Expr: {}",
            format!("{}", best_expr).bright_green().bold()
        );
        result = false;
    } else {
        println!(
            "{}\n{}\n",
            "Proved goal:".bright_green().bold(),
            end.pretty(40)
        );
        result = true;
        let mut extractor = Extractor::new(&runner.egraph, AstDepth);
        // We want to extract the best expression represented in the
        // same e-class as our initial expression, not from the whole e-graph.
        // Luckily the runner stores the eclass Id where we put the initial expression.
        let (_, best_expr_temp) = extractor.find_best(id);
        best_expr = best_expr_temp.to_string();
    }
    let total_time: f64 = runner.iterations.iter().map(|i| i.total_time).sum();
    println!(
        "Execution took: {}\n",
        format!("{} s", total_time).bright_green().bold()
    );
    ResultStructure { index, start_expression: String::from(start_expression), end_expressions: String::from(end_expression), result, best_expr: String::from(best_expr), total_time, condition: String::from(condition) }
}
// fn main() {
//     prove_time("(min (- x z) (- y z))", "(- (min x y) z)");
// }
