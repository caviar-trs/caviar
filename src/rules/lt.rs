use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn lt() -> Vec<Rewrite> { vec![
    // LT RULES
    rw!("gt-to-lt"      ;  "(> ?x ?z)"              => "(< ?z ?x)"),
    rw!("lt-to-zero"    ;  "(< ?a ?a)"              => "false"),
    rw!("lt-swap-in"    ;  "(< (+ ?x ?y) ?z)"       => "(< ?x (- ?z ?y))" ),
    rw!("lt-swap-out"   ;  "(< ?z (+ ?x ?y))"       => "(< (- ?z ?y) ?x)" ),  //adding it causes an error
    rw!("lt-x-x-sub-a"  ;  "(< (- ?a ?y) ?a )"      => "true" if crate::trs::is_const_pos("?y")),
    rw!("lt-const-pos"  ;  "(< 0 ?y )"              => "true" if crate::trs::is_const_pos("?y")),
    rw!("lt-const-neg"  ;  "(< ?y 0 )"              => "true" if crate::trs::is_const_neg("?y")),
    rw!("min-lt-cancel" ;  "( < ( min ?x ?y ) ?x )" => "( < ?y ?x )"),


    rw!("lt-min-mutual-term"    ; "( < ( min ?z ?y ) ( min ?x ?y ) )"           => "( < ?z ( min ?x ?y ) )"),
    rw!("lt-max-mutual-term"    ; "( < ( max ?z ?y ) ( max ?x ?y ) )"           => "( < ( max ?z ?y ) ?x )"),
    rw!("lt-min-term-term+pos"  ; "( < ( min ?z ?y ) ( min ?x ( + ?y ?c0 ) ) )" => "( < ( min ?z ?y ) ?x )" if crate::trs::is_const_pos("?c0")),
    rw!("lt-max-term-term+pos"  ; "( < ( max ?z ( + ?y ?c0 ) ) ( max ?x ?y ) )" => "( < ( max ?z ( + ?y ?c0 ) ) ?x )" if crate::trs::is_const_pos("?c0")),
    rw!("lt-min-term+neg-term"  ; "( < ( min ?z ( + ?y ?c0 ) ) ( min ?x ?y ) )" => "( < ( min ?z ( + ?y ?c0 ) ) ?x )" if crate::trs::is_const_neg("?c0")),
    rw!("lt-max-term+neg-term"  ; "( < ( max ?z ?y ) ( max ?x ( + ?y ?c0 ) ) )" => "( < ( max ?z ?y ) ?x )" if crate::trs::is_const_neg("?c0")),
    rw!("lt-min-term+cpos"      ; "( < ( min ?x ?y ) (+ ?x ?c0) )"              => "true" if crate::trs::is_const_pos("?c0")),
    rw!("lt-min-max-cancel"     ; "(< (max ?a ?c) (min ?a ?b))"                 => "false"),
    
    rw!("lt-mul-pos-cancel"     ; "(< (* ?x ?y) ?z)"                            => "(< ?x (/ ?z ?y))"  if crate::trs::is_const_pos("?y")), //adding it causes an error
    rw!("lt-mul-div-cancel"     ; "(< ?x (/ ?z ?y))"                            => "(< (* ?x ?y) ?z))"  if crate::trs::is_const_pos("?y")), //adding it causes an error
    
    // VERIFY WITH ADEL
    // rw!("cancel-mul-pos-lt";  "(< (* ?x ?y) ?z)" => "(< ?x (/ ?z ?y))"  if crate::trs::is_const_pos("?y")), //adding it causes an error
    // rw!("cancel-mul-div-lt";  "(< ?x (/ ?z ?y))" => "(< (* ?x ?y) ?z)"  if crate::trs::is_const_pos("?y")), //adding it causes an error
    // rw!("change-side-const-lt"; "( < ( min ?x (+ ?y ?c) ) ( min ?z ?g ) )" => "( < ( min (- ?x ?c) ?y ) ( min (- ?z ?c) (- ?g ?c) ) )"),
    // rw!("cancel-max-lt";  "(< (max ?a ?b) ?a)" => "0"), //adding it prevents proving
    // rw!("cancel-min-lt";  "(< ?a (min ?a ?b))" => "0"), //adding it prevents proving
    // rw!("cancel-mul-neg-lt";  "(< (* ?x ?y) ?z)" => "(< (/ ?z ?y) ?x)"  if crate::trs::is_const_neg("?y")), //no need to add it EGG is powerfull enough to switch sides and make the constant positive
]}