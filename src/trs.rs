use colored::*;
use egg::{rewrite as rw, *};
use ordered_float::NotNan;
use std::{cmp::Ordering, time::Instant};
pub type EGraph = egg::EGraph<Math, ConstantFold>;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;

pub type Constant = NotNan<f64>;

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
        Constant(Constant),
        Symbol(Symbol),
    }
}

#[derive(Default)]
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
            Math::Div([a, b]) if x(b) != Some(0.0.into()) => x(a)? / x(b)?,
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
fn is_const_or_distinct_var(v: &str, w: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
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

fn is_const_pos(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    let zero = NotNan::from(0.0);
    move |egraph, _, subst| {
        egraph[subst[var]].nodes.iter().any(|n| match n {
            Math::Constant(c) => c.cmp(&zero) == Ordering::Greater,
            _ => return false,
        })
    }
}
fn is_const_neg(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
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
fn is_const(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var]]
            .nodes
            .iter()
            .any(|n| matches!(n, Math::Constant(..)))
    }
}

#[allow(dead_code)]
fn is_sym(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var]]
            .nodes
            .iter()
            .any(|n| matches!(n, Math::Symbol(..)))
    }
}

fn is_not_zero(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    let zero = Math::Constant(0.0.into());
    move |egraph, _, subst| !egraph[subst[var]].nodes.contains(&zero)
}

#[rustfmt::skip]
fn rules() -> Vec<Rewrite> { vec![

    // ADD RULES
    rw!("comm-add";  "(+ ?a ?b)"        => "(+ ?b ?a)"),
    rw!("assoc-add"; "(+ ?a (+ ?b ?c))" => "(+ (+ ?a ?b) ?c)"),

    rw!("add-double"; "(+ ?a ?a)" => "(* 2 a)"),
    rw!("zero-add"; "(+ ?a 0)" => "?a"),
    rw!("add-zero"; "?a" => "(+ ?a 0)"),

    rw!("distribute"; "(* ?a (+ ?b ?c))"        => "(+ (* ?a ?b) (* ?a ?c))"),
    rw!("factor"    ; "(+ (* ?a ?b) (* ?a ?c))" => "(* ?a (+ ?b ?c))"),
    

    // SUB RULES
    rw!("sub-canon"; "(- ?a ?b)" => "(+ ?a (* -1 ?b))"),
    rw!("canon-sub"; "(+ ?a (* -1 ?b))"   => "(- ?a ?b)"),
    rw!("canon-sub-const"; "(+ ?a ?b)"   => "(- ?a (* -1 ?b))"),
    rw!("cancel-sub"; "(- ?a ?a)" => "0"),
    rw!("sub-zero"; "?a" => "(- ?a 0)"),
    rw!("zero-sub"; "(- ?a 0)" => "?a"),

    //MUL RULES
    rw!("comm-mul";  "(* ?a ?b)"        => "(* ?b ?a)"),
    rw!("assoc-mul"; "(* ?a (* ?b ?c))" => "(* (* ?a ?b) ?c)"),
    rw!("zero-mul"; "(* ?a 0)" => "0"),
    rw!("one-mul";  "(* ?a 1)" => "?a"),
    rw!("mul-one";  "?a" => "(* ?a 1)"),
    rw!("cancel-div-1surdiv"; "(* (/ 1 ?a) ?a)" => "1" if is_not_zero("?a")),
    rw!("cancel-mul-div"; "(/ (* ?y ?x) ?x)" => "?y"),

    rw!("mul-max-min"; "(* (max ?a ?b) (min ?a ?b))" => "(* ?a ?b)"),

    //DIV RULES
    rw!("div-to-mul"; "(/ ?x ?y)" => "(* ?x (/ 1 ?y))"),
    rw!("cancel-div"; "(/ ?a ?a)" => "1" if is_not_zero("?a")),

     //MOD RULES
    rw!("zero-mod"; "(% 0 ?x)" => "0"),
    rw!("mod-zero"; "(% ?x 0)" => "0"),
    rw!("x-x-mod"; "(% ?x ?x)" => "0"),
    rw!("mod-one"; "(% ?x 1)" => "0"),
    //rewrite((x - y) % 2, (x + y) % 2)  Addition and subtraction are the same modulo 2, because -1 == 1
    rw!("mod-two"; "(% (- ?x ?y) 2)" => "(% (+ ?x ?y) 2)"),
    rw!("mod-two"; "(% (+ ?x ?y) 2)" => "(% (- ?x ?y) 2)"),


    // LT RULES
    rw!("cancel-lt";  "(< ?a ?a)" => "0"),
    rw!("lt-x-xminus";  "(< (- ?a ?y) ?a )" => "1" ),
    rw!("cancel-max-lt";  "(< (max ?a ?b) ?a)" => "0"),
    rw!("cancel-min-lt";  "(< ?a (min ?a ?b))" => "0"),
    rw!("cancel-min-max-lt";  "(< (max ?a ?c) (min ?a ?b))" => "0"),
    rw!("div-Gt-Lt";  "(> ?x ?z)" => "(< ?z ?x)"),
    rw!("change-side-c-lt";  "(< (+ ?x ?y) ?z)" => "(< ?x (- ?z ?y))" ),
    // rw!("change-side-c-lt";  "(< ?z (+ ?x ?y))" => "(< (- ?z ?y) ?x)" ),  //adding it causes an error
    rw!("cancel-mul-pos-lt";  "(< (* ?x ?y) ?z)" => "(< ?x (/ ?z ?y))"  if is_const_pos("?y")),
    rw!("cancel-mul-neg-lt";  "(< (* ?x ?y) ?z)" => "(< (/ ?z ?y) ?x)"  if is_const_neg("?y")),



    // MIN RULES
    rw!("comm-min";  "(min ?a ?b)"        => "(min ?b ?a)"),
    rw!("min-x-x";          "(min ?x ?x)"                   => "(?x)"),
    rw!("min-min";          "(min (min ?x ?y) ?x)"          => "(min ?x ?y)"),
    rw!("min-max";          "(min (max ?x ?y) ?x)"          => "(?x)"),
    rw!("min-max-min";      "(min (max ?x ?y) (min ?x ?y))" => "(min ?x ?y)"),
    rw!("min-min-min-x";    "(min (min ?x ?y) (min ?x ?z))" => "(min (min ?y ?z) ?x)"),
    rw!("min-min-min";      "(min (min ?x ?y) (min ?z ?w))" => "(min (min (min ?x ?y) ?z) ?w)"),
    rw!("min-max-max-x";    "(min (max ?x ?y) (max ?x ?z))" => "(max (min ?y ?z) ?x)"),
    rw!("min-max-min2";     "(min (max (min ?x ?y) ?z) ?y)" => "(min (max ?x ?z) ?y)"),

    rw!("min-min-c0-pos";   "(min (+ (min ?x ?y) ?c) ?x)"   => "(min ?x (+ ?y ?c))" if is_const_pos("?c")),
    rw!("min-min-c0-neg";   "(min (+ (min ?x ?y) ?c) ?x)"   => "(+ (min ?x ?y) ?c))" if is_const_neg("?c")),

    rw!("min-plus1";         "(+ (min ?x ?y) ?z)"           => "(min (+ ?x ?z) (+ ?y ?z))"),
    rw!("min-plus2";         "(min (+ ?x ?z) (+ ?y ?z))"    => "(+ (min ?x ?y) ?z)"),
    rw!("min-sub1";          "(- (min ?x ?y) ?z)"           => "(min (- ?x ?z) (- ?y ?z))"),
    rw!("min-sub2";          "(min (- ?x ?z) (- ?y ?z))"    => "(- (min ?x ?y) ?z)"),
    rw!("min-mul-pos1";      "(* (min ?x ?y) ?z)"           => "(min (* ?x ?z) (* ?y ?z))" if is_const_pos("?z")),
    rw!("min-mul-pos2";      "(min (* ?x ?z) (* ?y ?z))"    => "(* (min ?x ?y) ?z)"  if is_const_pos("?z")),
    rw!("min-mul-neg1";      "(* (min ?x ?y) ?z)"           => "(max (* ?x ?z) (* ?x ?y))" if is_const_neg("?z")),
    rw!("min-mul-neg2";      "(max (* ?x ?z) (* ?y ?z))"    => "(* (min ?x ?y) ?z)" if is_const_neg("?z")),
    rw!("min-div-pos1";      "(max (/ ?x ?z) (/ ?y ?z))"    => "(/ (min ?x ?y) ?z)" if is_const_pos("?z")),
    rw!("min-div-pos2";      "(/ (min ?x ?y) ?z)"           => "(max (/ ?x ?z) (/ ?y ?z))" if is_const_pos("?z")),
    rw!("min-div-neg1";      "(max (/ ?x ?z) (/ ?y ?z))"    => "(/ (min ?x ?y) ?z)" if is_const_neg("?z")),
    rw!("min-div-neg2";      "(/ (min ?x ?y) ?z)"           => "(max (/ ?x ?z) (/ ?y ?z))"  if is_const_neg("?z")),

    rw!("min-ass1";          "(min (min ?x ?y) ?z)"         => "(min ?x (min ?y ?z))"),
    rw!("min-ass2";          "(min ?x (min ?y ?z))"         => "(min (min ?x ?y) ?z)"),

    //rw!("min-to-max";       "(min ?x ?y)"                   => "(max (* -1 ?x) (* -1 ?y))"),


    // MAX RULES
    rw!("comm-max";  "(max ?a ?b)"        => "(max ?b ?a)"),


   

    // NOT RULES
    rw!("cancel-eqlt";  "(<= ?x ?y)" => "(! (< ?y ?x))" ),
    rw!("not-eqgt";  "(>= ?x ?y)" => "(! (< ?x ?y))" ),
    // rw!("not-eq";  "(! (== x y))" => "!= y x" ),
    // rw!("not-dif";  "(! (!= x y))" => "<= y x" ),


    // Equality RULES
    rw!("comm-Eq";  "(== ?x ?y)"        => "(== ?y ?x)"),
    rw!("one-Eq";  "(== ?x 1)"        => "(?x)"),
    rw!("zero-Eq";  "(== ?x 0)"        => "(! ?x)"),
    rw!("x-x-Eq";  "(== ?x ?x)"        => "1"),
    // rw!("x-x-Eq";  "(== (* ?x ?y) 0)"        => "1"),
    

]}

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

// fn main() {
//     prove_time("(min (- x z) (- y z))", "(- (min x y) z)");
// }
