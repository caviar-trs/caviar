use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn sub() -> Vec<Rewrite> {
    vec![
        // SUB RULES
        rw!("sub-to-add"; "(- ?a ?b)"   => "(+ ?a (* -1 ?b))"),
        // rw!("add-to-sub"; "(+ ?a ?b)"   => "(- ?a (* -1 ?b))"),
        // NON
        rw!("sub-non-1"; "( - ( * ( / ( + ?x ?c0 ) ?c1 ) ?c1 ) ?x )"   => "( % (* ?x -1) ?c1 )" if crate::trs::compare_c0_c1("?c0","?c1","+1==")),
        rw!("sub-non-2"; "( - ( / ( + ?x ?c1 ) ?c0 ) ( / ( - ?x ?y ) ?c0 ) )"   => "( / ( - ( + ?y (- ( + ?c0 ?c1 ) 1 ) ) ( % ( + ?x ( % ?c1 ?c0 ) ) ?c0 ) ) ?c0 )" if crate::trs::is_const_pos("?c0")),
        
        rw!("sub-non-3"; "( - ?x ( / ( + ?x ?y ) ?c0 ) )"   => "( / ( + ( - ( * ?x (- ?c0 1 ) ) ?y ) (- ?c0 1 ) ) ?c0 )" if crate::trs::is_const_pos("?c0")),
        rw!("sub-non-4"; "( - ?x ( / ( - ?x ?y ) ?c0 ) )"   => "( / ( + ( + ( * ?x (- ?c0 1 ) ) ?y ) (- ?c0 1 ) ) ?c0 )" if crate::trs::is_const_pos("?c0")),
        rw!("sub-non-5"; "( - ?x ( / ( + ?y ?x ) ?c0 ) )"   => "( / ( + ( - ( * ?x (- ?c0 1 ) ) ?y ) (- ?c0 1 ) ) ?c0 )" if crate::trs::is_const_pos("?c0")),
        rw!("sub-non-6"; "( - ?x ( / ( - ?y ?x ) ?c0 ) )"   => "( / ( + ( - ( * ?x (+ ?c0 1 ) ) ?y ) (- ?c0 1 ) ) ?c0 )" if crate::trs::is_const_pos("?c0")),
    ]
}
