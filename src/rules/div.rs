use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn div() -> Vec<Rewrite> {
    let mut rules = vec![
        rw!("div-cancel"    ; "(/ ?a ?a)"           => "1" if crate::trs::is_not_zero("?a")),
        rw!("div-zero"      ; "(/ 0 ?x)"            => "0"),
    ];
    rules.extend(
    vec![
        //DIV RULES
        rw!("div-minus-down"; "(/ (* -1 ?a) ?b)"    <=> "(/ ?a (* -1 ?b))"),
        rw!("div-minus-up"  ; "(/ ?a (* -1 ?b))"    <=> "(/ (* -1 ?a) ?b)"),
        rw!("div-minus-in"  ; "(* -1 (/ ?a ?b))"    <=> "(/ (* -1 ?a) ?b)"),
        rw!("div-minus-out" ; "(/ (* -1 ?a) ?b)"    <=> "(* -1 (/ ?a ?b))"),
        //FOLD
        rw!("div-consts-div"; "( / ( * ?x ?a ) ?b )" <=> "( / ?x ( / ?b ?a ) )" if crate::trs::compare_c0_c1("?b", "?a", "%0<")),
        rw!("div-consts-mul"; "( / ( * ?x ?a ) ?b )" <=> "( * ?x ( / ?a ?b ) )" if crate::trs::compare_c0_c1("?a", "?b", "%0<")),
        rw!("div-consts-add"; "( / ( + ( * ?x ?a ) ?y ) ?b )" <=> "( + ( * ?x ( / ?a ?b ) ) ( / ?y ?b ) )" if crate::trs::compare_c0_c1("?a", "?b", "%0<")),
        rw!("div-separate"  ; "( / ( + ?x ?a ) ?b )" <=> "( + ( / ?x ?b ) ( / ?a ?b ) )" if crate::trs::compare_c0_c1("?a", "?b", "%0<")),
    ].concat());
    rules
}
