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
        "max" = Max([Id; 2]),
        "min" = Min([Id; 2]),
        "<" = Lt([Id; 2]),
        ">" = Gt([Id; 2]),
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

fn is_const(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var]]
            .nodes
            .iter()
            .any(|n| matches!(n, Math::Constant(..)))
    }
}

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
pub fn rules() -> Vec<Rewrite> { vec![
    rw!("comm-add";  "(+ ?a ?b)"        => "(+ ?b ?a)"),
    rw!("comm-mul";  "(* ?a ?b)"        => "(* ?b ?a)"),
    rw!("comm-max";  "(max ?a ?b)"        => "(max ?b ?a)"),
    rw!("comm-min";  "(min ?a ?b)"        => "(min ?b ?a)"),
    rw!("assoc-add"; "(+ ?a (+ ?b ?c))" => "(+ (+ ?a ?b) ?c)"),
    rw!("assoc-mul"; "(* ?a (* ?b ?c))" => "(* (* ?a ?b) ?c)"),

    rw!("sub-canon"; "(- ?a ?b)" => "(+ ?a (* -1 ?b))"),
    // rw!("div-canon"; "(/ ?a ?b)" => "(* ?a (pow ?b -1))" if is_not_zero("?b")),
    // rw!("canon-sub"; "(+ ?a (* -1 ?b))"   => "(- ?a ?b)"),
    // rw!("canon-div"; "(* ?a (pow ?b -1))" => "(/ ?a ?b)" if is_not_zero("?b")),

    rw!("add-double"; "(+ ?a ?a)" => "(* 2 a)"),
    rw!("zero-add"; "(+ ?a 0)" => "?a"),
    rw!("zero-mul"; "(* ?a 0)" => "0"),
    rw!("one-mul";  "(* ?a 1)" => "?a"),

    rw!("add-zero"; "?a" => "(+ ?a 0)"),
    rw!("mul-one";  "?a" => "(* ?a 1)"),

    rw!("cancel-sub"; "(- ?a ?a)" => "0"),
    rw!("cancel-div"; "(/ ?a ?a)" => "1" if is_not_zero("?a")),

    rw!("distribute"; "(* ?a (+ ?b ?c))"        => "(+ (* ?a ?b) (* ?a ?c))"),
    rw!("factor"    ; "(+ (* ?a ?b) (* ?a ?c))" => "(* ?a (+ ?b ?c))"),
    // rw!("mul-max-min"; "* (max ?a ?b) (min ?a ?b)" => "(* ?a ?b)")


    ///////////////////////// TESTS

    rw!("div-to-mul"; "(/ ?x ?y)" => "(* ?x (/ 1 ?y))"),

    rw!("cancel-div-1surdiv"; "(* (/ 1 ?a) ?a)" => "1" if is_not_zero("?a")),
    
    rw!("cancel-mul-div"; "(/ (* ?y ?x) ?x)" => "?y"),


    // LT RULES
    rw!("cancel-lt";  "(< ?a ?a)" => "0"),
    rw!("lt-x-xminus";  "(< (- ?a ?y) ?a )" => "1" ),
    rw!("cancel-max-lt";  "(< (max ?a ?b) ?a)" => "0"),
    rw!("cancel-min-lt";  "(< ?a (min ?a ?b))" => "0"),
    rw!("cancel-min--max-lt";  "(< (max ?a ?c) (min ?a ?b))" => "0"),

    rw!("div-Gt-Lt-";  "(> ?x ?z)" => "(< (* -1 ?x) (* -1 ?z))"),


    rw!("change-side-c-lt";  "(< (+ ?x ?y) ?z)" => "(< ?x (- ?z ?y))" ),
    // rw!("change-side-c-lt";  "(< ?z (+ ?x ?y))" => "(< (- ?z ?y) ?x)" ),  //adding it causes an error

    rw!("div-c-lt";  "(< (* ?x ?y) ?z)" => "(< ?x (/ ?z ?y))"  if is_const_pos("?y")),
    rw!("div-c-neg-lt";  "(< (* ?x ?y) ?z)" => "(< (/ ?z ?y) ?x)"  if is_const_neg("?y")),
    
    // rw!("cancel-mul-pos-lt";  "(< (* ?x ?c) (* ?y ?c))" => "(< ?x ?y)" if is_const_pos("?c")),
    // rw!("cancel-mul-neg-lt";  "(< (* ?x ?c) (* ?y ?c))" => "(< ?y ?x)" if is_const_neg("?c")),



    
]}

// egg::test_fn! {
//     math_associate_adds, [
//         rw!("comm-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
//         rw!("assoc-add"; "(+ ?a (+ ?b ?c))" => "(+ (+ ?a ?b) ?c)"),
//     ],
//     runner = Runner::default()
//         .with_iter_limit(7)
//         .with_scheduler(SimpleScheduler),
//     "(+ 1 (+ 2 (+ 3 (+ 4 (+ 5 (+ 6 7))))))"
//     =>
//     "(+ 7 (+ 6 (+ 5 (+ 4 (+ 3 (+ 2 1))))))"
//     @check |r: Runner<Math, ()>| assert_eq!(r.egraph.number_of_classes(), 127)
// }

// egg::test_fn! {
//     #[should_panic(expected = "Could not prove goal 0")]
//     math_fail, rules(),
//     "(+ x y)" => "(/ x y)"
// }

// egg::test_fn! {math_simplify_add, rules(), "(+ x (+ x (+ x x)))" => "(* 4 x)" }
// egg::test_fn! {math_powers, rules(), "(* (pow 2 x) (pow 2 y))" => "(pow 2 (+ x y))"}

// egg::test_fn! {
//     math_simplify_const, rules(),
//     "(+ 1 (- a (* (- 2 1) a)))" => "1"
// }

// egg::test_fn! {
//     math_simplify_root, rules(),
//     runner = Runner::default().with_node_limit(75_000),
//     r#"
//     (/ 1
//        (- (/ (+ 1 (sqrt five))
//              2)
//           (/ (- 1 (sqrt five))
//              2)))"#
//     =>
//     "(/ 1 (sqrt five))"
// }

// egg::test_fn! {
//     math_simplify_factor, rules(),
//     "(* (+ x 3) (+ x 1))"
//     =>
//     "(+ (+ (* x x) (* 4 x)) 3)"
// }

// egg::test_fn! {math_diff_same,      rules(), "(d x x)" => "1"}
// egg::test_fn! {math_diff_different, rules(), "(d x y)" => "0"}
// egg::test_fn! {math_diff_simple1,   rules(), "(d x (+ 1 (* 2 x)))" => "2"}
// egg::test_fn! {math_diff_simple2,   rules(), "(d x (+ 1 (* y x)))" => "y"}
// egg::test_fn! {math_diff_ln,        rules(), "(d x (ln x))" => "(/ 1 x)"}

// egg::test_fn! {
//     diff_power_simple, rules(),
//     "(d x (pow x 3))" => "(* 3 (pow x 2))"
// }

// egg::test_fn! {
//     diff_power_harder, rules(),
//     runner = Runner::default()
//         .with_time_limit(std::time::Duration::from_secs(10))
//         .with_iter_limit(60)
//         .with_node_limit(100_000)
//         // HACK this needs to "see" the end expression
//         .with_expr(&"(* x (- (* 3 x) 14))".parse().unwrap()),
//     "(d x (- (pow x 3) (* 7 (pow x 2))))"
//     =>
//     "(* x (- (* 3 x) 14))"
// }

// egg::test_fn! {
//     integ_one, rules(), "(i 1 x)" => "x"
// }

// egg::test_fn! {
//     integ_sin, rules(), "(i (cos x) x)" => "(sin x)"
// }

// egg::test_fn! {
//     integ_x, rules(), "(i (pow x 1) x)" => "(/ (pow x 2) 2)"
// }

// egg::test_fn! {
//     integ_part1, rules(), "(i (* x (cos x)) x)" => "(+ (* x (sin x)) (cos x))"
// }

// egg::test_fn! {
//     integ_part2, rules(),
//     "(i (* (cos x) x) x)" => "(+ (* x (sin x)) (cos x))"
// }

// egg::test_fn! {
//     integ_part3, rules(), "(i (ln x) x)" => "(- (* x (ln x)) x)"
// }

// egg::test_fn! {
//     test_add, rules(), "(< 5 0)" => "(1)"
// }

// fn print_graph(egraph: &EGraph) {
//     println!("printing graph to svg");
//     // create a Dot and then compile it assuming `dot` is on the system
//     egraph.dot().to_svg("target/foo.svg").unwrap();
//     println!("done printing graph to svg");
// }

fn main() {
    let start: RecExpr<Math> = "(< (* 2 x) y)".parse().unwrap();
    let end: Pattern<Math> = "(<  x (/ y 2))".parse().unwrap();

    let now = Instant::now();
    // That's it! We can run equality saturation now.
    let runner = Runner::default().with_expr(&start).run(rules().iter());
    println!(
        "Saturation took: {}",
        format!("{} ms", now.elapsed().as_millis())
            .bright_green()
            .bold()
    );

    // print_graph(&runner.egraph);
    let id = runner.egraph.find(*runner.roots.last().unwrap());
    let matches = end.search_eclass(&runner.egraph, id);
    if matches.is_none() {
        println!(
            "{}\n{}\n",
            "Could not prove goal:".bright_red().bold(),
            end.pretty(40),
        );
    } else {
        println!(
            "{}\n{}\n",
            "Proved goal:".bright_green().bold(),
            end.pretty(40)
        );
    }

    // Extractors can take a user-defined cost function,
    // we'll use the egg-provided AstSize for now
    let now1 = Instant::now();
    let mut extractor = Extractor::new(&runner.egraph, AstSize);

    // We want to extract the best expression represented in the
    // same e-class as our initial expression, not from the whole e-graph.
    // Luckily the runner stores the eclass Id where we put the initial expression.
    let (_, best_expr) = extractor.find_best(id);

    println!(
        "Best Expr: {} found in {} ms (without saturation) {} (with saturation)",
        format!("{}", best_expr).bright_green().bold(),
        now1.elapsed().as_millis(),
        format!("{} ms", now.elapsed().as_millis())
            .bright_green()
            .bold()
    );
}
