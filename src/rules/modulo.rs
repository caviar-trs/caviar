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
     //rw!("mod-const"; "(% (+ ?x ?c0) ?c1)" => "(% (+ ?x (% ?c0 ?c1)) ?c1)"),
     rw!("mod-const-plus"; "(% ?x ?c1)" => "(% (+ ?x ?c1) ?c1)" if crate::trs::are_less_eq_absolute("?c1", "?x")),
     rw!("mod-const-neg"; "(% ?x ?c1)" => "(% (- ?x ?c1) ?c1)" if crate::trs::are_less_eq_absolute("?c1", "?x")),

     rw!("mod-minus-1"; "(% (* ?x -1) ?c)" => "(* -1 (% ?x ?c))"),
     rw!("mod-minus-2"; "(* -1 (% ?x ?c))" => "(% (* ?x -1) ?c)"),

     rw!("mod-two"; "(% (- ?x ?y) 2)" => "(% (+ ?x ?y) 2)"),
     rw!("mod-add-const"; "(+ ?a ( % (+ ?x ?b) ?c))" => "(% (+ ?x (+ ?a ?b)) ?c)" if crate::trs::are_less_eq("?a", "?c"))
]}