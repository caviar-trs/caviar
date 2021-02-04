use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn lt() -> Vec<Rewrite> { vec![
    // LT RULES
    rw!("Gt-Lt";  "(> ?x ?z)" => "(< ?z ?x)"),

    rw!("cancel-lt";  "(< ?a ?a)" => "0"),
    rw!("change-side-c-lt";  "(< (+ ?x ?y) ?z)" => "(< ?x (- ?z ?y))" ),
    rw!("change-side-c-lt-1";  "(< ?z (+ ?x ?y))" => "(< (- ?z ?y) ?x)" ),  //adding it causes an error
    rw!("lt-x-xminus";  "(< (- ?a ?y) ?a )" => "1" if crate::trs::is_const_pos("?y")),
    rw!("lt-const";  "(< 0 ?y )" => "1" if crate::trs::is_const_pos("?y")),
    rw!("lt-const-1";  "(< ?y 0 )" => "1" if crate::trs::is_const_neg("?y")),

    rw!("cancel-min-lt";  "( < ( min ?x ?y ) ?x )" => "( < ?y ?x )"),


    // VERIFY WITH ADEL
    rw!("mutual-term-min-lt";  "( < ( min ?z ?y ) ( min ?x ?y ) )" => "( < ?z ( min ?x ?y ) )"),
    rw!("term-term+pos-min-lt";  "( < ( min ?z ?y ) ( min ?x ( + ?y ?c0 ) ) )" => "( < ( min ?z ?y ) ?x )" if crate::trs::is_const_pos("?c0")),

    rw!("term+neg-term-min-lt"; "( < ( min ?z ( + ?y ?c0 ) ) ( min ?x ?y ) )" => "( < ( min ?z ( + ?y ?c0 ) ) ?x )" if crate::trs::is_const_neg("?c0")),

    // rw!("cancel-max-lt";  "(< (max ?a ?b) ?a)" => "0"), //adding it prevents proving
    // rw!("cancel-min-lt";  "(< ?a (min ?a ?b))" => "0"), //adding it prevents proving
    rw!("cancel-min-max-lt";  "(< (max ?a ?c) (min ?a ?b))" => "0"),



    rw!("cancel-mul-pos-lt";  "(< (* ?x ?y) ?z)" => "(< ?x (/ ?z ?y))"  if crate::trs::is_const_pos("?y")), //adding it causes an error
    // rw!("cancel-mul-neg-lt";  "(< (* ?x ?y) ?z)" => "(< (/ ?z ?y) ?x)"  if crate::trs::is_const_neg("?y")), //no need to add it EGG is powerfull enough to switch sides and make the constant positive
]}