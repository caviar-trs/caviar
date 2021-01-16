use egg::{rewrite as rw, *};
use ordered_float::NotNan;
use std::time::Instant;

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
    // rw!("max-min"; "* (max ?x ?b) (min ?x ?b)" => "(* ?x ?b)")
    

    // rw!("pow-mul"; "(* (pow ?a ?b) (pow ?a ?c))" => "(pow ?a (+ ?b ?c))"),
    // rw!("pow0"; "(pow ?x 0)" => "1"
    //     if is_not_zero("?x")),
    // rw!("pow1"; "(pow ?x 1)" => "?x"),
    // rw!("pow2"; "(pow ?x 2)" => "(* ?x ?x)"),
    // rw!("pow-recip"; "(pow ?x -1)" => "(/ 1 ?x)"
    //     if is_not_zero("?x")),
    // rw!("recip-mul-div"; "(* ?x (/ 1 ?x))" => "1" if is_not_zero("?x")),

    // rw!("d-variable"; "(d ?x ?x)" => "1" if is_sym("?x")),
    // rw!("d-constant"; "(d ?x ?c)" => "0" if is_sym("?x") if is_const_or_distinct_var("?c", "?x")),

    // rw!("d-add"; "(d ?x (+ ?a ?b))" => "(+ (d ?x ?a) (d ?x ?b))"),
    // rw!("d-mul"; "(d ?x (* ?a ?b))" => "(+ (* ?a (d ?x ?b)) (* ?b (d ?x ?a)))"),

    // rw!("d-sin"; "(d ?x (sin ?x))" => "(cos ?x)"),
    // rw!("d-cos"; "(d ?x (cos ?x))" => "(* -1 (sin ?x))"),

    // rw!("d-ln"; "(d ?x (ln ?x))" => "(/ 1 ?x)" if is_not_zero("?x")),

    // rw!("d-power";
    //     "(d ?x (pow ?f ?g))" =>
    //     "(* (pow ?f ?g)
    //         (+ (* (d ?x ?f)
    //               (/ ?g ?f))
    //            (* (d ?x ?g)
    //               (ln ?f))))"
    //     if is_not_zero("?f")
    //     if is_not_zero("?g")
    // ),

    // rw!("i-one"; "(i 1 ?x)" => "?x"),
    // rw!("i-power-const"; "(i (pow ?x ?c) ?x)" =>
    //     "(/ (pow ?x (+ ?c 1)) (+ ?c 1))" if is_const("?c")),
    // rw!("i-cos"; "(i (cos ?x) ?x)" => "(sin ?x)"),
    // rw!("i-sin"; "(i (sin ?x) ?x)" => "(* -1 (cos ?x))"),
    // rw!("i-sum"; "(i (+ ?f ?g) ?x)" => "(+ (i ?f ?x) (i ?g ?x))"),
    // rw!("i-dif"; "(i (- ?f ?g) ?x)" => "(- (i ?f ?x) (i ?g ?x))"),
    // rw!("i-parts"; "(i (* ?a ?b) ?x)" =>
    //     "(- (* ?a (i ?b ?x)) (i (* (d ?x ?a) (i ?b ?x)) ?x))"),
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
//     test_add, rules(), "(+ a (+ a (+ a (+ a (+ a a)))))" => "(* 6 a)"
// }

// fn print_graph(egraph: &EGraph) {
//     println!("printing graph to svg");
//     // create a Dot and then compile it assuming `dot` is on the system
//     egraph.dot().to_svg("target/foo.svg").unwrap();
//     println!("done printing graph to svg");
// }

fn main() {
    let start: RecExpr<Math> = "* (max 2 3) (min 2 3)".parse().unwrap();
    let mut end: RecExpr<Math> = "10".parse().unwrap();

    let now = Instant::now();
    // That's it! We can run equality saturation now.
    let runner = Runner::default().with_expr(&start).run(rules().iter());
    // print_graph(&runner.egraph);
    println!("Saturation took: {} ms", now.elapsed().as_millis());
    let mut eclasses = runner.egraph.equivs(&start, &end);
    if eclasses.is_empty() {
        println!("{} and {} are not equivalent", start, end);
    } else {
        println!("{} and {} are equivalent", start, end);
    }

    end = "(* 3 x)".parse().unwrap();

    eclasses = runner.egraph.equivs(&start, &end);
    if eclasses.is_empty() {
        println!("{} and {} are not equivalent", start, end);
    } else {
        println!("{} and {} are equivalent", start, end);
    }

    /// Graph Printing
    // let egraph = &runner.egraph;

    // Extractors can take a user-defined cost function,
    // we'll use the egg-provided AstSize for now
    let now1 = Instant::now();
    let mut extractor = Extractor::new(&runner.egraph, AstSize);

    // We want to extract the best expression represented in the
    // same e-class as our initial expression, not from the whole e-graph.
    // Luckily the runner stores the eclass Id where we put the initial expression.
    let (_, best_expr) = extractor.find_best(runner.egraph.find(*runner.roots.last().unwrap()));

    println!(
        "Best Expr: {} found in {} ms (without saturation) {} ms (with saturation)",
        best_expr,
        now1.elapsed().as_millis(),
        now.elapsed().as_millis()
    );
}
