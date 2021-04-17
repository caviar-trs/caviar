use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn ineq() -> Vec<Rewrite> {
    vec![
        // Inequality RULES
        rw!("ineq-to-eq";  "(!= ?x ?y)"        => "(! (== ?x ?y))"),
    ]
}
