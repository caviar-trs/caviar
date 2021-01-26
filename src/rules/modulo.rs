use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn modulo() -> Vec<Rewrite> { vec![
     //MOD RULES
     rw!("zero-mod"; "(% 0 ?x)" => "0"),
     rw!("mod-zero"; "(% ?x 0)" => "0"),
     rw!("x-x-mod"; "(% ?x ?x)" => "0"),
     rw!("mod-one"; "(% ?x 1)" => "0"),
     //rewrite((x - y) % 2, (x + y) % 2)  Addition and subtraction are the same modulo 2, because -1 == 1
     rw!("mod-two"; "(% (- ?x ?y) 2)" => "(% (+ ?x ?y) 2)"),
]}