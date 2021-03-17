use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn andor() -> Vec<Rewrite> { vec![
    // AND-OR RULES
    rw!("and-over-or"   ;  "(&& ?a (|| ?b ?c))"        => "(|| (&& ?a ?b) (&& ?a ?c))"),
    rw!("or-over-and"   ;  "(|| ?a (&& ?b ?c))"        => "(&& (|| ?a ?b) (|| ?a ?c))"),
    rw!("or-x-and-x-y"  ;  "(|| ?x (&& ?x ?y))"        => "?x"),

    // rw!("and-over-or-inv";  "(|| (&& ?a ?b) (&& ?a ?c))" => "(&& ?a (|| ?b ?c))" ),//NOTAXIOM
    // rw!("or-over-and-inv";  "(&& (|| ?a ?b) (|| ?a ?c))" => "(|| ?a (&& ?b ?c))" ),//NOTAXIOM
    // rw!("x-xory-and";  "(&& ?x (|| ?x ?y))"        => "?x"),//NOTAXIOM
]}