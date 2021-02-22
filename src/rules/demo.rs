use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;

pub type Rewrite = egg::Rewrite<Math, ConstantFold>;

pub fn demo() -> Vec<Rewrite> {
    vec![
        rw!("x-x-Eq";  "(== ?x ?x)"        => "1"),
        rw!("Eq-min-2"; "(<= ?y ?x)" => "( == (min ?x ?y) ?y)"),
        rw!("comm-min";          "(min ?a ?b)"                   => "(min ?b ?a)"),
        rw!("min-min";           "(min (min ?x ?y) ?x)"          => "(min ?x ?y)"),
        rw!("min-div-pos2";      "(/ (min ?x ?y) ?z)"           => "(min (/ ?x ?z) (/ ?y ?z))" if crate::trs::is_const_pos("?z"))
    ]
}