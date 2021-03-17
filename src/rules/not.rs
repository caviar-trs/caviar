use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn not() -> Vec<Rewrite> { vec![
    // NOT RULES
    rw!("eqlt-to-not-gt";  "(<= ?x ?y)"     => "(! (< ?y ?x))" ),
    rw!("not-gt-to-eqlt";  "(! (< ?y ?x))"  => "(<= ?x ?y)" ),
    rw!("eqgt-to-not-lt";  "(>= ?x ?y)"     => "(! (< ?x ?y))" ),
    rw!("not-eq-to-ineq";  "(! (== ?x ?y))" => "(!= ?x ?y)" ),
    rw!("not-not"       ;  "(! (! ?x))"     => "?x" ),
    
    // rw!("not-dif";  "(! (!= ?x ?y))" => "(== ?x ?y)" ), //NOTAXIOM
    // rw!("not-eqgt-inv";  "(! (< ?x ?y))" => "(>= ?x ?y)" ),
    // rw!("not-eq-inv";  "(!= ?x ?y)" => "(! (== ?x ?y))" ),
    // rw!("not-dif-inv";  "(== ?x ?y)" => "(! (!= ?x ?y))" ),
]}