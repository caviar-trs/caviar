use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn min() -> Vec<Rewrite> { vec![
    // MIN RULES
    rw!("comm-min";         "(min ?a ?b)"                   => "(min ?b ?a)"),
    rw!("min-x-x";          "(min ?x ?x)"                   => "(?x)"),
    rw!("min-min";          "(min (min ?x ?y) ?x)"          => "(min ?x ?y)"),
    rw!("min-max";          "(min (max ?x ?y) ?x)"          => "(?x)"),
    // rw!("min-max-max-x";    "(min (max ?x ?y) (max ?x ?z))" => "(max (min ?y ?z) ?x)"),
    // rw!("min-max-min2";     "(min (max (min ?x ?y) ?z) ?y)" => "(min (max ?x ?z) ?y)"),
    rw!("min-plus1";         "(+ (min ?x ?y) ?z)"           => "(min (+ ?x ?z) (+ ?y ?z))"),
    // rw!("min-plus2";         "(min (+ ?x ?z) (+ ?y ?z))"    => "(+ (min ?x ?y) ?z)"),
    // rw!("min-sub1";          "(- (min ?x ?y) ?z)"           => "(min (- ?x ?z) (- ?y ?z))"),
    // rw!("min-sub2";          "(min (- ?x ?z) (- ?y ?z))"    => "(- (min ?x ?y) ?z)"),
    // rw!("min-mul-pos1";      "(* (min ?x ?y) ?z)"           => "(min (* ?x ?z) (* ?y ?z))" if crate::trs::is_const_pos("?z")),
    // rw!("min-mul-pos2";      "(min (* ?x ?z) (* ?y ?z))"    => "(* (min ?x ?y) ?z)"  if crate::trs::is_const_pos("?z")),
    // rw!("min-mul-neg1";      "(* (min ?x ?y) ?z)"           => "(max (* ?x ?z) (* ?y ?z))" if crate::trs::is_const_neg("?z")),
    // rw!("min-mul-neg2";      "(max (* ?x ?z) (* ?y ?z))"    => "(* (min ?x ?y) ?z)" if crate::trs::is_const_neg("?z")),
    // rw!("min-div-pos1";      "(max (/ ?x ?z) (/ ?y ?z))"    => "(/ (min ?x ?y) ?z)" if crate::trs::is_const_pos("?z")),
    // rw!("min-div-pos2";      "(/ (min ?x ?y) ?z)"           => "(max (/ ?x ?z) (/ ?y ?z))" if crate::trs::is_const_pos("?z")),
    // rw!("min-div-neg1";      "(max (/ ?x ?z) (/ ?y ?z))"    => "(/ (min ?x ?y) ?z)" if crate::trs::is_const_neg("?z")),
    // rw!("min-div-neg2";      "(/ (min ?x ?y) ?z)"           => "(max (/ ?x ?z) (/ ?y ?z))"  if crate::trs::is_const_neg("?z")),
    rw!("min-ass1";          "(min (min ?x ?y) ?z)"         => "(min ?x (min ?y ?z))"),

]}