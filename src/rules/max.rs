use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn max() -> Vec<Rewrite> { vec![
    // MAX RULES
    rw!("max-to-min"; "(max ?a ?b)" => "(* -1 (min (* -1 ?a) (* -1 ?b)))"),
]}