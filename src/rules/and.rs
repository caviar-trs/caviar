use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn and() -> Vec<Rewrite> { vec![
    // AND RULES
    rw!("comm-and";  "(&& ?y ?x)"        => "(&& ?x ?y)"),
    rw!("assoc-and"; "(&& ?a (&& ?b ?c))" => "(&& (&& ?a ?b) ?c)"),
    rw!("x-1-and";  "(&& 1 ?x)"        => "?x"),
    rw!("x-0-and";  "(&& 0 ?x)"        => "0"),
    rw!("x-x-and";  "(&& ?x ?x)"        => "?x"),
    rw!("x-!x-and";  "(&& ?x (! ?x))" => "0"),
    rw!("min-and";  "(&& (< ?x ?y) (< ?x ?z))" => "(< ?x (min ?y ?z))"),
    rw!("and-min";  "(< ?x (min ?y ?z))" => "(&& (< ?x ?y) (< ?x ?z))"),
    // rw!("min-and";  "(&& (< ?y ?x) (< ?z ?x))" => "(< (max ?y ?z) ?x)"), //this
    // rw!("and-max";  "(> ?x (max ?y ?z))" => "(&& (< ?z ?x) (< ?y ?x))"),
]}