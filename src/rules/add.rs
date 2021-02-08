use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn add() -> Vec<Rewrite> { vec![
    // ADD RULES
    rw!("comm-add";  "(+ ?a ?b)"        => "(+ ?b ?a)"),
    rw!("assoc-add"; "(+ ?a (+ ?b ?c))" => "(+ (+ ?a ?b) ?c)"),
    rw!("add-double"; "(+ ?a ?a)" => "(* 2 ?a)"),
    rw!("zero-add"; "(+ ?a 0)" => "?a"),
    rw!("add-zero"; "?a" => "(+ ?a 0)"),
    rw!("distribute"; "(* ?a (+ ?b ?c))"        => "(+ (* ?a ?b) (* ?a ?c))"),
    rw!("factor"    ; "(+ (* ?a ?b) (* ?a ?c))" => "(* ?a (+ ?b ?c))"),
    rw!("plus-max-min"    ; "(+ (min ?a ?b) (max ?a ?b))" => "(+ ?a ?b)"),
    rw!("denominator1"; "(+ (/ ?a ?b) ?c)" => "(/ (+ ?a (* ?b ?c)) ?b)"),
    rw!("denominator2"; "(/ (+ ?a (* ?b ?c)) ?b)" => "(+ (/ ?a ?b) ?c)"),
    rw!("add-div-mod"    ; "( + ( / ?x 2 ) ( % ?x 2 ) )" => "( / ( + ?x 1 ) 2 )"),
]}