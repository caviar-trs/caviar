use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn add() -> Vec<Rewrite> {
    vec![
        // ADD RULES
        rw!("add-comm"      ; "(+ ?a ?b)"                   => "(+ ?b ?a)"),
        rw!("add-assoc"     ; "(+ ?a (+ ?b ?c))"            => "(+ (+ ?a ?b) ?c)"),
        rw!("add-zero"      ; "(+ ?a 0)"                    => "?a"),
        rw!("add-dist-mul"  ; "(* ?a (+ ?b ?c))"            => "(+ (* ?a ?b) (* ?a ?c))"),
        rw!("add-fact-mul"  ; "(+ (* ?a ?b) (* ?a ?c))"     => "(* ?a (+ ?b ?c))"),
        rw!("add-denom-mul" ; "(+ (/ ?a ?b) ?c)"            => "(/ (+ ?a (* ?b ?c)) ?b)"),
        rw!("add-denom-div" ; "(/ (+ ?a (* ?b ?c)) ?b)"     => "(+ (/ ?a ?b) ?c)"),
        rw!("add-div-mod"   ; "( + ( / ?x 2 ) ( % ?x 2 ) )" => "( / ( + ?x 1 ) 2 )"),
        // rw!("add-double"; "(+ ?a ?a)" => "(* 2 ?a)"), //NOTAXIOM
        // rw!("plus-max-min"    ; "(+ (min ?a ?b) (max ?a ?b))" => "(+ ?a ?b)"), //NOTAXIOM
        // rw!("add-zero"; "?a" => "(+ ?a 0)"),
    ]
}
