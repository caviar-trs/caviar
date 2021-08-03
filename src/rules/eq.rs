use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn eq() -> Vec<Rewrite> {
    let mut rules = vec![
        rw!("eq-x-x"        ; "(== ?x ?x)"           => "1"),
        rw!("Eq-a-b"        ; "(== (* ?a ?x) ?b)"    => "0" if crate::trs::compare_c0_c1("?b", "?a", "!%0")),
        rw!("Eq-max-c-pos"  ; "(== (max ?x ?c) 0)"   => "0" if crate::trs::is_const_pos("?c")),
        rw!("Eq-max-c-neg"  ; "(== (max ?x ?c) 0)"   => "(== ?x 0)" if crate::trs::is_const_neg("?c")),
        rw!("Eq-min-c-pos"  ; "(== (min ?x ?c) 0)"   => "0" if crate::trs::is_const_neg("?c")),
        rw!("Eq-min-c-neg"  ; "(== (min ?x ?c) 0)"   => "(== ?x 0)" if crate::trs::is_const_pos("?c")),
    ];
    rules.extend(
        vec![
            // Equality RULES
            rw!("eq-comm"       ; "(== ?x ?y)"           <=> "(== ?y ?x)"),
            rw!("eq-x-y-0"      ; "(== ?x ?y)"           <=> "(== (- ?x ?y) 0)"),
            rw!("eq-swap"       ; "(== (+ ?x ?y) ?z)"    <=> "(== ?x (- ?z ?y))"),
            rw!("eq-mul-x-y-0"  ; "(== (* ?x ?y) 0)"     <=> "(|| (== ?x 0) (== ?y 0))"),
            rw!("eq-max-lt"     ; "( == (max ?x ?y) ?y)" <=> "(<= ?x ?y)"),
            rw!("Eq-min-lt"     ; "( == (min ?x ?y) ?y)" <=> "(<= ?y ?x)"),
            rw!("Eq-lt-min"     ; "(<= ?y ?x)"           <=> "( == (min ?x ?y) ?y)"),
        ]
        .concat(),
    );
    rules
}
