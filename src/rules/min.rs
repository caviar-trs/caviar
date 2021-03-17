use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;

pub type Rewrite = egg::Rewrite<Math, ConstantFold>;

pub fn min() -> Vec<Rewrite> {
    vec![
        // MIN RULES
        rw!("min-comm"      ; "(min ?a ?b)"                         => "(min ?b ?a)"),
        rw!("min-ass"       ; "(min (min ?x ?y) ?z)"                => "(min ?x (min ?y ?z))"),
        rw!("min-x-x"       ; "(min ?x ?x)"                         => "?x"),
        rw!("min-max"       ; "(min (max ?x ?y) ?x)"                => "?x"),
        rw!("min-max-max-x" ; "(min (max ?x ?y) (max ?x ?z))"       => "(max (min ?y ?z) ?x)"),
        rw!("min-max-min-y" ; "(min (max (min ?x ?y) ?z) ?y)"       => "(min (max ?x ?z) ?y)"),
        rw!("min-sub-both"  ; "(min (+ ?a ?b) ?c)"                  => "(+ (min ?b (- ?c ?a)) ?a)"),
        rw!("min-add-both"  ; "(+ (min ?x ?y) ?z)"                  => "(min (+ ?x ?z) (+ ?y ?z))"),
        
        rw!("min-x-x-plus-a-pos"; "(min ?x (+ ?x ?a))"               => "?x" if crate::trs::is_const_pos("?a") ),
        rw!("min-x-x-plus-a-neg"; "(min ?x (+ ?x ?a))"               => "(+ ?x ?a)" if crate::trs::is_const_neg("?a") ),
        rw!("min-mul-in-pos"    ; "(* (min ?x ?y) ?z)"               => "(min (* ?x ?z) (* ?y ?z))" if crate::trs::is_const_pos("?z")),
        rw!("min-mul-out-pos"   ; "(min (* ?x ?z) (* ?y ?z))"        => "(* (min ?x ?y) ?z)"  if crate::trs::is_const_pos("?z")),
        rw!("min-mul-in-neg"    ; "(* (min ?x ?y) ?z)"               => "(max (* ?x ?z) (* ?y ?z))" if crate::trs::is_const_neg("?z")),
        rw!("min-mul-out-neg"   ; "(max (* ?x ?z) (* ?y ?z))"        => "(* (min ?x ?y) ?z)" if crate::trs::is_const_neg("?z")),
        rw!("min-div-in-pos"    ; "(/ (min ?x ?y) ?z)"               => "(min (/ ?x ?z) (/ ?y ?z))" if crate::trs::is_const_pos("?z")),
        rw!("min-div-out-pos"   ; "(min (/ ?x ?z) (/ ?y ?z))"        => "(/ (min ?x ?y) ?z)" if crate::trs::is_const_pos("?z")),
        rw!("min-div-in-neg"    ; "(/ (max ?x ?y) ?z)"               => "(min (/ ?x ?z) (/ ?y ?z))"  if crate::trs::is_const_neg("?z")),
        rw!("min-div-out-neg"   ; "(min (/ ?x ?z) (/ ?y ?z))"        => "(/ (max ?x ?y) ?z)" if crate::trs::is_const_neg("?z")),
        rw!("min-max-const"     ; "( min ( max ?x ?c0 ) ?c1 )"       => "?c1" if crate::trs::are_less_eq("?c1","?c0")),
        
        
        rw!("min-div-mul"               ; "( min ( * ( / ?x ?c0 ) ?c0 ) ?x )"    => "( * ( / ?x ?c0 ) ?c0 )" if  crate::trs::is_const_pos("?c0")),
        rw!("min-mod-const-to-mod"      ; "(min (% ?x ?c0) ?c1)"                         => "(% ?x ?c0)" if crate::trs::compare_c0_c1("?c1","?c0",">=a-1")),
        rw!("min-mod-const-to-const"    ; "(min (% ?x ?c0) ?c1)" => "?c1" if crate::trs::compare_c0_c1("?c1","?c0","<=-a+1")), // c1 <= - |c0| + 1
        rw!("min-max-switch"            ; "( min ( max ?x ?c0 ) ?c1 )" => "( max ( min ?x ?c1 ) ?c0 )" if crate::trs::compare_c0_c1("?c0","?c1","<=") ),
        rw!("max-min-switch"            ; "( max ( min ?x ?c1 ) ?c0 )" => "( min ( max ?x ?c0 ) ?c1 )" if crate::trs::compare_c0_c1("?c0","?c1","<=") ),
        
        // rw!("min-plus2";         "(min (+ ?x ?z) (+ ?y ?z))"    => "(+ (min ?x ?y) ?z)"),//NOTAXIOM
        // rw!("min-sub1";          "(- (min ?x ?y) ?z)"           => "(min (- ?x ?z) (- ?y ?z))"),//NOTAXIOM
        // rw!("min-sub2";          "(min (- ?x ?z) (- ?y ?z))"    => "(- (min ?x ?y) ?z)"),//NOTAXIOM
        // TO VERIFY WITH ADEL
        //  rw!("min-0-x-mod-plus-min"; "(min 0 (+ (% ?x ?c0) ?c1) )" => "0" if crate::trs::compare_c0_c1("?c1", "?c0",">=a")),
        // rw!("min-min";           "(min (min ?x ?y) ?x)"          => "(min ?x ?y)"),//NOTAXIOM
        // rw!("div-remove-min"; "(min ?x (+ (% ?x ?c1) (+ ?x ?c0)))" => "?x" if crate::trs::is_const_pos("?c0")),
        ]
    }