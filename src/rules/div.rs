use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn div() -> Vec<Rewrite> { vec![
    //DIV RULES
    rw!("div-zero"      ; "(/ 0 ?x)"            => "(0)"),
    rw!("div-cancel"    ; "(/ ?a ?a)"           => "1" if crate::trs::is_not_zero("?a")),
    rw!("div-minus-down"; "(/ (* -1 ?a) ?b)"    => "(/ ?a (* -1 ?b))"),
    rw!("div-minus-up"  ; "(/ ?a (* -1 ?b))"    => "(/ (* -1 ?a) ?b)"),
    rw!("div-minus-in"  ; "(* -1 (/ ?a ?b))"    => "(/ (* -1 ?a) ?b)"),
    rw!("div-minus-out" ; "(/ (* -1 ?a) ?b)"    => "(* -1 (/ ?a ?b))")

    // rw!("div-to-mul"; "(/ ?x ?y)" => "(* ?x (/ 1 ?y))"), //FLOAT
]}