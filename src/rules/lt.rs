use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn lt() -> Vec<Rewrite> {
    vec![
        // LT RULES
        rw!("gt-to-lt"      ;  "(> ?x ?z)"              => "(< ?z ?x)"),
        rw!("lt-to-zero"    ;  "(< ?a ?a)"              => "0"),
        rw!("lt-swap-in"    ;  "(< (+ ?x ?y) ?z)"       => "(< ?x (- ?z ?y))" ),
        rw!("lt-swap-out"   ;  "(< ?z (+ ?x ?y))"       => "(< (- ?z ?y) ?x)" ), //adding it causes an error
        rw!("lt-x-x-sub-a"  ;  "(< (- ?a ?y) ?a )"      => "1" if crate::trs::is_const_pos("?y")),
        rw!("lt-const-pos"  ;  "(< 0 ?y )"              => "1" if crate::trs::is_const_pos("?y")),
        rw!("lt-const-neg"  ;  "(< ?y 0 )"              => "1" if crate::trs::is_const_neg("?y")),
        rw!("min-lt-cancel" ;  "( < ( min ?x ?y ) ?x )" => "( < ?y ?x )"),
        rw!("lt-min-mutual-term"    ; "( < ( min ?z ?y ) ( min ?x ?y ) )"           => "( < ?z ( min ?x ?y ) )"),
        rw!("lt-max-mutual-term"    ; "( < ( max ?z ?y ) ( max ?x ?y ) )"           => "( < ( max ?z ?y ) ?x )"),
        rw!("lt-min-term-term+pos"  ; "( < ( min ?z ?y ) ( min ?x ( + ?y ?c0 ) ) )" => "( < ( min ?z ?y ) ?x )" if crate::trs::is_const_pos("?c0")),
        rw!("lt-max-term-term+pos"  ; "( < ( max ?z ( + ?y ?c0 ) ) ( max ?x ?y ) )" => "( < ( max ?z ( + ?y ?c0 ) ) ?x )" if crate::trs::is_const_pos("?c0")),
        rw!("lt-min-term+neg-term"  ; "( < ( min ?z ( + ?y ?c0 ) ) ( min ?x ?y ) )" => "( < ( min ?z ( + ?y ?c0 ) ) ?x )" if crate::trs::is_const_neg("?c0")),
        rw!("lt-max-term+neg-term"  ; "( < ( max ?z ?y ) ( max ?x ( + ?y ?c0 ) ) )" => "( < ( max ?z ?y ) ?x )" if crate::trs::is_const_neg("?c0")),
        rw!("lt-min-term+cpos"      ; "( < ( min ?x ?y ) (+ ?x ?c0) )"              => "1" if crate::trs::is_const_pos("?c0")),
        rw!("lt-min-max-cancel"     ; "(< (max ?a ?c) (min ?a ?b))"                 => "0"),
        rw!("lt-mul-pos-cancel"     ; "(< (* ?x ?y) ?z)"                            => "(< ?x (/ ?z ?y))"  if crate::trs::is_const_pos("?y")),
        rw!("lt-mul-div-cancel"     ; "(< ?x (/ ?z ?y))"                            => "(< (* ?x ?y) ?z))"  if crate::trs::is_const_pos("?y")),
    ]
}
