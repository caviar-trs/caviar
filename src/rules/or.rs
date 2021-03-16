use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn or() -> Vec<Rewrite> { vec![
    // OR RULES
    rw!("or-to-and";  "(|| ?x ?y)"        => "(! (&& (! ?x) (! ?y)))"),
    rw!("comm-or";  "(|| ?y ?x)"        => "(|| ?x ?y)"),

    // rw!("assoc-or"; "(|| ?a (|| ?b ?c))" => "(|| (|| ?a ?b) ?c)"),
    // rw!("x-1-or";  "(|| 1 ?x)"        => "1"),
    // rw!("x-0-or";  "(|| 0 ?x)" => "?x"),
    // rw!("x-x-or";  "(|| ?x ?x)" => "?x"),
    // rw!("x-!x-or";  "(|| ?x (! ?x))" => "1"),
    // rw!("max-or";  "(|| (< ?x ?y) (< ?x ?z))" => "(< ?x (max ?y ?z))"),
    // rw!("or-max";  "(< ?x (max ?y ?z))" => "(|| (< ?x ?y) (< ?x ?z))"),
    // rw!("min-or";  "(|| (< ?y ?x) (< ?z ?x))" => "(< (min ?y ?z) ?x)"),
    // rw!("or-min";  "(< (min ?y ?z) ?x)" => "(|| (< ?y ?x) (< ?z ?x))"),
]}