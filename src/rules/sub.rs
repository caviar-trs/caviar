use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn sub() -> Vec<Rewrite> {
    vec![
        // SUB RULES
        rw!("sub-to-add"; "(- ?a ?b)"   => "(+ ?a (* -1 ?b))"),
        rw!("add-to-sub"; "(+ ?a ?b)"   => "(- ?a (* -1 ?b))"),
        // rw!("canon-sub"; "(+ ?a (* -1 ?b))"   => "(- ?a ?b)"),//NOTAXIOM
        // rw!("cancel-sub"; "(- ?a ?a)" => "0"),//NOTAXIOM
        // rw!("zero-sub"; "(- ?a 0)" => "?a"),//NOTAXIOM
        // rw!("minus-max"; "(* -1 (max ?x ?y))" => "(min (* -1 ?x) (* -1 ?y))"),//NOTAXIOM
        // rw!("sub-zero"; "?a" => "(- ?a 0)"),//NOTAXIOM
    ]
}
