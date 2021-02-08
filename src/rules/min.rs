use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;

pub type Rewrite = egg::Rewrite<Math, ConstantFold>;

pub fn min() -> Vec<Rewrite> {
    vec![
        // MIN RULES
        rw!("comm-min";          "(min ?a ?b)"                   => "(min ?b ?a)"),
        rw!("min-x-x";           "(min ?x ?x)"                   => "?x"),
        rw!("min-min";           "(min (min ?x ?y) ?x)"          => "(min ?x ?y)"),
        rw!("min-max";           "(min (max ?x ?y) ?x)"          => "?x"),
        rw!("min-max-max-x";     "(min (max ?x ?y) (max ?x ?z))" => "(max (min ?y ?z) ?x)"),
        rw!("min-max-min2";      "(min (max (min ?x ?y) ?z) ?y)" => "(min (max ?x ?z) ?y)"),
        rw!("min-plus1";         "(+ (min ?x ?y) ?z)"           => "(min (+ ?x ?z) (+ ?y ?z))"),
        rw!("min-plus2";         "(min (+ ?x ?z) (+ ?y ?z))"    => "(+ (min ?x ?y) ?z)"),
        rw!("min-sub1";          "(- (min ?x ?y) ?z)"           => "(min (- ?x ?z) (- ?y ?z))"),
        rw!("min-sub2";          "(min (- ?x ?z) (- ?y ?z))"    => "(- (min ?x ?y) ?z)"),
        rw!("min-x-xsuby";       "(min ?x (+ ?x ?a))"                   => "?x" if crate::trs::is_const_pos("?a") ),
        rw!("min-x-xsuby-neg";   "(min ?x (+ ?x ?a))"                   => "(+ ?x ?a)" if crate::trs::is_const_neg("?a") ),
        rw!("min-mul-pos1";      "(* (min ?x ?y) ?z)"           => "(min (* ?x ?z) (* ?y ?z))" if crate::trs::is_const_pos("?z")),
        rw!("min-mul-pos2";      "(min (* ?x ?z) (* ?y ?z))"    => "(* (min ?x ?y) ?z)"  if crate::trs::is_const_pos("?z")),
        rw!("min-mul-neg1";      "(* (min ?x ?y) ?z)"           => "(max (* ?x ?z) (* ?y ?z))" if crate::trs::is_const_neg("?z")),
        rw!("min-mul-neg2";      "(max (* ?x ?z) (* ?y ?z))"    => "(* (min ?x ?y) ?z)" if crate::trs::is_const_neg("?z")),
        rw!("min-div-pos1";      "(min (/ ?x ?z) (/ ?y ?z))"    => "(/ (min ?x ?y) ?z)" if crate::trs::is_const_pos("?z")),
        rw!("min-div-pos2";      "(/ (min ?x ?y) ?z)"           => "(min (/ ?x ?z) (/ ?y ?z))" if crate::trs::is_const_pos("?z")),
        rw!("min-div-neg1";      "(min (/ ?x ?z) (/ ?y ?z))"    => "(/ (max ?x ?y) ?z)" if crate::trs::is_const_neg("?z")),
        rw!("min-div-neg2";      "(/ (max ?x ?y) ?z)"           => "(min (/ ?x ?z) (/ ?y ?z))"  if crate::trs::is_const_neg("?z")),
        rw!("min-ass1";          "(min (min ?x ?y) ?z)"         => "(min ?x (min ?y ?z))"),
        rw!("min-max-const";     "( min ( max ?x ?c0 ) ?c1 )"          => "?c1" if crate::trs::are_less_eq("?c1","?c0")),

        // TO VERIFY WITH ADEL
        // rw!("div-min-int"; "( min ( * ( / ?x ?c0 ) ?c0 ) ?x )" => "( * ( / ?x ?c0 ) ?c0 )" if crate::trs::is_const_pos("?c0")),
        // rw!("extract-x-2sides-min-add"; "(min ?x (+ ?x ?y))" => "(+ ?x (min 0 ?y))"),
        // rw!("extract-x-2sides-mul-min"; "(min ?x (* ?x ?y))" => "(* ?x (min 1 ?y))"),
         rw!("min-0-x-mod-plus-min"; "(min 0 (+ (% ?x ?c0) ?c1) )" => "0" if crate::trs::sum_is_great_zero_c0_abs("?c0", "?c1")),
        // rw!("div-remove-min"; "(min ?x (+ (% ?x ?c1) (+ ?x ?c0)))" => "?x" if crate::trs::is_const_pos("?c0")),
    ]
}