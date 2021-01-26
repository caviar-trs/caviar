use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn ineq() -> Vec<Rewrite> { vec![
    // Inequality RULES
    rw!("comm-IEq";  "(!= ?x ?y)"      => "(!= ?y ?x)"),
    rw!("IEq-Eq";  "(!= ?x ?y)"        => "(! (== ?x ?y))"),
]}