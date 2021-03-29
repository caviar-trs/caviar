use std::time::Duration;

use std::cmp::Ordering;

use egg::{rewrite as rw, *};
use ordered_float::NotNan;
use colored::Colorize;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::env;

pub type EGraph = egg::EGraph<Math, ConstantFold>;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;

pub type Constant = NotNan<f64>;

define_language! {
    pub enum Math {
        "d" = Diff([Id; 2]),
        "i" = Integral([Id; 2]),

        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        "pow" = Pow([Id; 2]),
        "ln" = Ln(Id),
        "sqrt" = Sqrt(Id),

        "sin" = Sin(Id),
        "cos" = Cos(Id),
        "==" = Eq([Id;2]),

        Constant(Constant),
        Symbol(Symbol),
    }
}

// You could use egg::AstSize, but this is useful for debugging, since
// it will really try to get rid of the Diff operator
pub struct MathCostFn;
impl egg::CostFunction<Math> for MathCostFn {
    type Cost = usize;
    fn cost<C>(&mut self, enode: &Math, mut costs: C) -> Self::Cost
        where
            C: FnMut(Id) -> Self::Cost,
    {
        let op_cost = match enode {
            Math::Diff(..) => 100,
            Math::Integral(..) => 100,
            _ => 1,
        };
        enode.fold(op_cost, |sum, i| sum + costs(i))
    }
}

#[derive(Default)]
pub struct ConstantFold;
impl Analysis<Math> for ConstantFold {
    type Data = Option<Constant>;

    fn make(egraph: &EGraph, enode: &Math) -> Self::Data {
        let x = |i: &Id| egraph[*i].data;
        Some(match enode {
            Math::Constant(c) => *c,
            Math::Add([a, b]) => x(a)? + x(b)?,
            Math::Sub([a, b]) => x(a)? - x(b)?,
            Math::Mul([a, b]) => x(a)? * x(b)?,
            Math::Div([a, b]) if x(b) != Some(0.0.into()) => x(a)? / x(b)?,
            Math::Eq([a,b]) =>   NotNan::from((x(a)?.into_inner() == x(b)?.into_inner()) as i64 as f64),
            _ => return None,
        })
    }

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
    rw!("add-comm"      ; "(+ ?a ?b)"                   => "(+ ?b ?a)"),
    rw!("add-assoc"     ; "(+ ?a (+ ?b ?c))"            => "(+ (+ ?a ?b) ?c)"),
    // rw!("add-zero"      ; "(+ ?a 0)"                    => "?a"),

    rw!("eq-x-x"        ; "(== ?x ?x)"           => "1"),
    rw!("sub-to-add"; "(- ?a ?b)"   => "(+ ?a (* -1 ?b))"),
    // rw!("cancel-div"; "(/ ?a ?a)" => "1" if is_not_zero("?a")),
    //
    // rw!("distribute"; "(* ?a (+ ?b ?c))"        => "(+ (* ?a ?b) (* ?a ?c))"),
    // rw!("factor"    ; "(+ (* ?a ?b) (* ?a ?c))" => "(* ?a (+ ?b ?c))"),
    //
    // rw!("pow-mul"; "(* (pow ?a ?b) (pow ?a ?c))" => "(pow ?a (+ ?b ?c))"),
    // rw!("pow0"; "(pow ?x 0)" => "1"
    //     if is_not_zero("?x")),
    // rw!("pow1"; "(pow ?x 1)" => "?x"),
    // rw!("pow2"; "(pow ?x 2)" => "(* ?x ?x)"),
    // rw!("pow-recip"; "(pow ?x -1)" => "(/ 1 ?x)"
    //     if is_not_zero("?x")),
    // rw!("recip-mul-div"; "(* ?x (/ 1 ?x))" => "1" if is_not_zero("?x")),
    //
    // rw!("d-variable"; "(d ?x ?x)" => "1" if is_sym("?x")),
    // rw!("d-constant"; "(d ?x ?c)" => "0" if is_sym("?x") if is_const_or_distinct_var("?c", "?x")),
    //
    // rw!("d-add"; "(d ?x (+ ?a ?b))" => "(+ (d ?x ?a) (d ?x ?b))"),
    // rw!("d-mul"; "(d ?x (* ?a ?b))" => "(+ (* ?a (d ?x ?b)) (* ?b (d ?x ?a)))"),
    //
    // rw!("d-sin"; "(d ?x (sin ?x))" => "(cos ?x)"),
    // rw!("d-cos"; "(d ?x (cos ?x))" => "(* -1 (sin ?x))"),
    //
    // rw!("d-ln"; "(d ?x (ln ?x))" => "(/ 1 ?x)" if is_not_zero("?x")),
    //
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
    //
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


#[allow(dead_code)]
pub fn prove(start_expression: &str, ruleset_class: i8, params: (usize, usize, u64), use_iteration_check: bool, report: bool) -> (bool, f64, Option<String>) {
    let start: RecExpr<Math> = start_expression.parse().unwrap();
    let end_1: Pattern<Math> = "1".parse().unwrap();
    let end_0: Pattern<Math> = "0".parse().unwrap();
    let goals = [end_0.clone(), end_1.clone()];
    // That's it! We can run equality saturation now.
    // let runner = Runner::default().with_expr(&start).run(rules(ruleset_class).iter());
    let mut runner: Runner<Math, ConstantFold>;
    let mut result = false;
    let mut proved_goal_index = 0;
    let id;
    let best_expr;

    if report{
        println!("\n==================================\nProving Expression:\n {}\n",start_expression)
    }
    if use_iteration_check {
        runner = Runner::default()
            .with_iter_limit(params.0)
            .with_node_limit(params.1)
            .with_time_limit(Duration::new(params.2, 0))
            .with_expr(&start)
            .run_check_iteration(rules().iter(), &goals);

    } else {
        runner = Runner::default()
            .with_iter_limit(params.0)
            .with_node_limit(params.1)
            .with_time_limit(Duration::new(params.2, 0))
            .with_expr(&start);
        print_graph("start", &runner.egraph);
        runner = runner.run(rules().iter());
        print_graph(&format!("iter{}",params.0), &runner.egraph);
    }

    id = runner.egraph.find(*runner.roots.last().unwrap());
    for (goal_index, goal) in goals.iter().enumerate() {

        let boolean = (goal.search_eclass(&runner.egraph, id)).is_none();
        if !boolean{
            result = true;
            proved_goal_index = goal_index;
            break;
        }
    }

    if result {
        if report{
            println!(
                "{}\n{:?}",
                "Proved goal:".bright_green().bold(),goals[proved_goal_index].to_string()
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

    (result,total_time,best_expr)
}


#[allow(dead_code)]
pub fn print_graph(name: &str, egraph: &EGraph) {
    println!("printing graph to svg");
    egraph.dot().to_svg("results/".to_string() + name + ".svg").unwrap();
    println!("done printing graph to svg");
}


pub fn get_runner_params(start: usize) -> Result<(usize, usize, u64), Box<dyn Error>> {
    let iter =
        match env::args_os().nth(start) {
            None => 30,
            Some(i) => i.into_string().unwrap().parse::<usize>().unwrap(),
        };

    let nodes =
        match env::args_os().nth(start + 1) {
            None => 10000,
            Some(i) => i.into_string().unwrap().parse::<usize>().unwrap(),
        };
    let time =
        match env::args_os().nth(start + 2) {
            None => 5,
            Some(i) => i.into_string().unwrap().parse::<u64>().unwrap(),
        };

    return Ok((iter, nodes, time));
}


pub fn get_start_end() -> Result<(String, String), Box<dyn Error>> {
    let mut file = File::open("./tmp/exprs.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let v: Vec<&str> = s.split("\n").collect();
    return Ok((v[0].to_string(), v[1].to_string()));
}


fn main() {
    let params = get_runner_params(1).unwrap();
    let (start, end) = get_start_end().unwrap();
    println!("Simplifying expression:\n {}\n to {}", start,end);
    prove(&start, -2, params, false, true);
}