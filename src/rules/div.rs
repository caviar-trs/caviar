use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn div() -> Vec<Rewrite> { vec![
    //DIV RULES
    // rw!("div-to-mul"; "(/ ?x ?y)" => "(* ?x (/ 1 ?y))"), //FLOAT
    rw!("0-div"; "(/ 0 ?x)" => "(0)"),
    rw!("cancel-div"; "(/ ?a ?a)" => "1" if crate::trs::is_not_zero("?a")),
    rw!("minus-one-1"; "(/ (* -1 ?a) ?b)" => "(/ ?a (* -1 ?b))"),
    rw!("minus-one-2"; "(/ ?a (* -1 ?b))" => "(/ (* -1 ?a) ?b)"),
    rw!("minus-one-mul-div-1"; "(* -1 (/ ?a ?b))" => "(/ (* -1 ?a) ?b)"),
    rw!("minus-one-mul-div-2"; "(/ (* -1 ?a) ?b)" => "(* -1 (/ ?a ?b))")
]}