use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn sub() -> Vec<Rewrite> { vec![
    // SUB RULES
    rw!("sub-canon"; "(- ?a ?b)" => "(+ ?a (* -1 ?b))"),
    rw!("canon-sub"; "(+ ?a (* -1 ?b))"   => "(- ?a ?b)"),
    rw!("canon-sub-const"; "(+ ?a ?b)"   => "(- ?a (* -1 ?b))"),
    rw!("cancel-sub"; "(- ?a ?a)" => "0"),
    rw!("zero-sub"; "(- ?a 0)" => "?a"),
    rw!("minus-max"; "(* -1 (max ?x ?y))" => "(min (* -1 ?x) (* -1 ?y))"),

    rw!("sub-zero"; "?a" => "(- ?a 0)"),
]}