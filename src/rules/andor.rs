use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn andor() -> Vec<Rewrite> { vec![
    // AND-OR RULES
    rw!("and-over-or";  "(&& ?a (|| ?b ?c))" => "(|| (&& ?a ?b) (&& ?a ?c))"),
    rw!("and-over-or-inv";  "(|| (&& ?a ?b) (&& ?a ?c))" => "(&& ?a (|| ?b ?c))" ),
    rw!("or-over-and";  "(|| ?a (&& ?b ?c))" => "(&& (|| ?a ?b) (|| ?a ?c))"),
    rw!("or-over-and-inv";  "(&& (|| ?a ?b) (|| ?a ?c))" => "(|| ?a (&& ?b ?c))" ),
    rw!("x-xandy-or";  "(|| ?x (&& ?x ?y))"        => "?x"),
    rw!("x-xory-and";  "(&& ?x (|| ?x ?y))"        => "?x"),
]}