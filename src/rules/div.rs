use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn div() -> Vec<Rewrite> { vec![
    //DIV RULES
    rw!("div-to-mul"; "(/ ?x ?y)" => "(* ?x (/ 1 ?y))"),
    rw!("cancel-div"; "(/ ?a ?a)" => "1" if crate::trs::is_not_zero("?a")),
]}