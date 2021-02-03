use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn not() -> Vec<Rewrite> { vec![
    // NOT RULES
    rw!("cancel-eqlt";  "(<= ?x ?y)" => "(! (< ?y ?x))" ),
    rw!("inv-cancel-eqlt";  "(! (< ?y ?x))" => "(<= ?x ?y)" ),
    rw!("not-eqgt";  "(>= ?x ?y)" => "(! (< ?x ?y))" ),
    rw!("not-eqgt-inv";  "(! (< ?x ?y))" => "(>= ?x ?y)" ),
    rw!("not-eq";  "(! (== ?x ?y))" => "(!= ?x ?y)" ),
    rw!("not-eq-inv";  "(!= ?x ?y)" => "(! (== ?x ?y))" ),
    rw!("not-dif";  "(! (!= ?x ?y))" => "(== ?x ?y)" ),
    rw!("not-not-x";  "(! (! ?x))" => "?x" ),
    rw!("not-dif-inv";  "(== ?x ?y)" => "(! (!= ?x ?y))" ),
]}