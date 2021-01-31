use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn max() -> Vec<Rewrite> { vec![
    // MAX RULES
    rw!("comm-max";     "(max ?a ?b)"                   => "(max ?b ?a)"),
    rw!("max-x-x";      "(max ?x ?x)"                   => "?x"),
    rw!("max-max";      "(max (max ?x ?y) ?x)"          => "(max ?x ?y)"),
    rw!("max-min";      "(max (min ?x ?y) ?x)"          => "?x"),
    rw!("max-max-min";  "(max (max ?x ?y) (min ?x ?y))" => "(max ?x ?y)"),
    rw!("max-ass1";      "(max (max ?x ?y) ?z)"         => "(max ?x (max ?y ?z))"),
    // rw!("max-ass2";      "(max ?x (max ?y ?z))"         => "(max (max ?x ?y) ?z)"),
    // rw!("max-min-min";    "(max (min ?x ?y) (min ?x ?z))" => "(min ?x (max ?y ?z) )"),
    // rw!("max-min-max";    "(max (min (max ?x ?y) ?z) ?y)" => "(max (min ?x ?z) ?y)"),
    rw!("max-plus1";         "(+ (max ?x ?y) ?z)"           => "(max (+ ?x ?z) (+ ?y ?z))"), //this
    // rw!("max-plus2";         "(max (+ ?x ?z) (+ ?y ?z))"    => "(+ (max ?x ?y) ?z)"), //this
    // rw!("max-sub1";          "(- (max ?x ?y) ?z)"           => "(max (- ?x ?z) (- ?y ?z))"), //this
    // rw!("max-sub2";          "(max (- ?x ?z) (- ?y ?z))"    => "(- (max ?x ?y) ?z)"), //this
    // rw!("max-mul-pos1";      "(* (max ?x ?y) ?z)"           => "(max (* ?x ?z) (* ?y ?z))" if crate::trs::is_const_pos("?z")),
    // rw!("max-mul-pos2";      "(max (* ?x ?z) (* ?y ?z))"    => "(* (max ?x ?y) ?z)"  if crate::trs::is_const_pos("?z")),
    // rw!("max-mul-neg1";      "(* (max ?x ?y) ?z)"           => "(min (* ?x ?z) (* ?y ?z))" if crate::trs::is_const_neg("?z")), // this
    // rw!("max-mul-neg2";      "(min (* ?x ?z) (* ?y ?z))"    => "(* (max ?x ?y) ?z)" if crate::trs::is_const_neg("?z")),
    // rw!("max-div-pos1";      "(min (/ ?x ?z) (/ ?y ?z))"    => "(/ (max ?x ?y) ?z)" if crate::trs::is_const_pos("?z")),
    // rw!("max-div-pos2";      "(/ (max ?x ?y) ?z)"           => "(min (/ ?x ?z) (/ ?y ?z))" if crate::trs::is_const_pos("?z")),
    // rw!("max-div-neg1";      "(min (/ ?x ?z) (/ ?y ?z))"    => "(/ (max ?x ?y) ?z)" if crate::trs::is_const_neg("?z")),
    // rw!("max-div-neg2";      "(/ (max ?x ?y) ?z)"           => "(min (/ ?x ?z) (/ ?y ?z))"  if crate::trs::is_const_neg("?z")),
]}