use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn or() -> Vec<Rewrite> {
    vec![
        // OR RULES
        rw!("or-to-and" ;"(|| ?x ?y)"        => "(! (&& (! ?x) (! ?y)))"),
        rw!("or-comm"   ;"(|| ?y ?x)"        => "(|| ?x ?y)"),
    ]
}
