use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn andor() -> Vec<Rewrite> {
    vec![
        // AND-OR RULES
        rw!("and-over-or"   ;  "(&& ?a (|| ?b ?c))"        => "(|| (&& ?a ?b) (&& ?a ?c))"),
        rw!("or-over-and"   ;  "(|| ?a (&& ?b ?c))"        => "(&& (|| ?a ?b) (|| ?a ?c))"),
        rw!("or-x-and-x-y"  ;  "(|| ?x (&& ?x ?y))"        => "?x"),
    ]
}
