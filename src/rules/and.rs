use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn and() -> Vec<Rewrite> {
    vec![
        // AND RULES
        rw!("and-comm"          ;  "(&& ?y ?x)"                         => "(&& ?x ?y)"),
        rw!("and-assoc"         ;  "(&& ?a (&& ?b ?c))"                 => "(&& (&& ?a ?b) ?c)"),
        rw!("and-x-1"           ;  "(&& 1 ?x)"                          => "?x"),
        rw!("and-x-x"           ;  "(&& ?x ?x)"                         => "?x"),
        rw!("and-x-not-x"       ;  "(&& ?x (! ?x))"                     => "0"),
        rw!("and-eq-eq"         ;  "( && ( == ?x ?c0 ) ( == ?x ?c1 ) )" => "0" if crate::trs::compare_c0_c1("?c1", "?c0", "!=")),
        rw!("and-ineq-eq"       ;  "( && ( != ?x ?c0 ) ( == ?x ?c1 ) )" => "( == ?x ?c1 )" if crate::trs::compare_c0_c1("?c1", "?c0", "!=")),
        rw!("and-lt-to-min"     ;  "(&& (< ?x ?y) (< ?x ?z))"           => "(< ?x (min ?y ?z))"),
        rw!("and-min-to-lt"     ;  "(< ?x (min ?y ?z))"                 => "(&& (< ?x ?y) (< ?x ?z))"),
        rw!("and-eqlt-to-min"   ;  "(&& (<= ?x ?y) (<= ?x ?z))"         => "(<= ?x (min ?y ?z))"),
        rw!("and-min-to-eqlt"   ;  "(<= ?x (min ?y ?z))"                => "(&& (<= ?x ?y) (<= ?x ?z))"),
        rw!("and-lt-to-max"     ;  "(&& (< ?y ?x) (< ?z ?x))"           => "(< (max ?y ?z) ?x)"),
        rw!("and-max-to-lt"     ;  "(> ?x (max ?y ?z))"                 => "(&& (< ?z ?x) (< ?y ?x))"),
        rw!("and-eqlt-to-max"   ;  "(&& (<= ?y ?x) (<= ?z ?x))"         => "(<= (max ?y ?z) ?x)"),
        rw!("and-max-to-eqlt"   ;  "(>= ?x (max ?y ?z))"                => "(&& (<= ?z ?x) (<= ?y ?x))"),
        rw!("and-lt-gt-to-0"    ; "( && ( < ?c0 ?x ) ( < ?x ?c1 ) )"    => "0" if crate::trs::compare_c0_c1("?c1", "?c0", "<=+1")),
        rw!("and-eqlt-eqgt-to-0"; "( && ( <= ?c0 ?x ) ( <= ?x ?c1 ) )"  => "0" if crate::trs::compare_c0_c1("?c1", "?c0", "<")),
        rw!("and-eqlt-gt-to-0"  ; "( && ( <= ?c0 ?x ) ( < ?x ?c1 ) )"   => "0" if crate::trs::compare_c0_c1("?c1", "?c0", "<=")),
    ]
}
