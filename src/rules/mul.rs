use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn mul() -> Vec<Rewrite> { vec![
    //MUL RULES
    rw!("comm-mul";  "(* ?a ?b)"        => "(* ?b ?a)"),
    rw!("assoc-mul"; "(* ?a (* ?b ?c))" => "(* (* ?a ?b) ?c)"),
    rw!("zero-mul"; "(* ?a 0)" => "0"),
    rw!("one-mul";  "(* ?a 1)" => "?a"),
    rw!("mul-one";  "?a" => "(* ?a 1)"),
    rw!("cancel-div-1surdiv"; "(* (/ 1 ?a) ?a)" => "1" if crate::trs::is_not_zero("?a")),
    // rw!("cancel-mul-div"; "(/ (* ?y ?x) ?x)" => "?y"),
    rw!("mul-max-min"; "(* (max ?a ?b) (min ?a ?b))" => "(* ?a ?b)"),
]}