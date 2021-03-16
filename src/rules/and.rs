use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn and() -> Vec<Rewrite> { vec![
    // AND RULES
    rw!("comm-and";  "(&& ?y ?x)"        => "(&& ?x ?y)"),
    rw!("assoc-and"; "(&& ?a (&& ?b ?c))" => "(&& (&& ?a ?b) ?c)"),
    rw!("x-1-and";  "(&& 1 ?x)"        => "?x"),
    // rw!("x-0-and";  "(&& 0 ?x)"        => "0"), //NOTAXIOM
    rw!("x-x-and";  "(&& ?x ?x)"        => "?x"),
    rw!("x-!x-and";  "(&& ?x (! ?x))" => "0"),

    rw!("and-eq";  "( && ( == ?x ?c0 ) ( == ?x ?c1 ) )" => "0" if crate::trs::compare_c0_c1("?c1", "?c0", "!=")),
    rw!("and-eq-not";  "( && ( != ?x ?c0 ) ( == ?x ?c1 ) )" => "( == ?x ?c1 )" if crate::trs::compare_c0_c1("?c1", "?c0", "!=")),
    

    rw!("min-and";  "(&& (< ?x ?y) (< ?x ?z))" => "(< ?x (min ?y ?z))"),
    rw!("and-min";  "(< ?x (min ?y ?z))" => "(&& (< ?x ?y) (< ?x ?z))"),

    rw!("min-and-eq";  "(&& (<= ?x ?y) (<= ?x ?z))" => "(<= ?x (min ?y ?z))"),
    rw!("and-min-eq";  "(<= ?x (min ?y ?z))" => "(&& (<= ?x ?y) (<= ?x ?z))"),

    rw!("max-and";  "(&& (< ?y ?x) (< ?z ?x))" => "(< (max ?y ?z) ?x)"),
    rw!("and-max";  "(> ?x (max ?y ?z))" => "(&& (< ?z ?x) (< ?y ?x))"),

    rw!("max-and-eq";  "(&& (<= ?y ?x) (<= ?z ?x))" => "(<= (max ?y ?z) ?x)"),
    rw!("and-max-eq";  "(>= ?x (max ?y ?z))" => "(&& (<= ?z ?x) (<= ?y ?x))"),

    rw!("and-lt-gt"; "( && ( < ?c0 ?x ) ( < ?x ?c1 ) )" => "(0)" if crate::trs::compare_c0_c1("?c1", "?c0", "<=+1")),
    rw!("and-lt-gt-eq"; "( && ( <= ?c0 ?x ) ( <= ?x ?c1 ) )" => "(0)" if crate::trs::compare_c0_c1("?c1", "?c0", "<")),
    rw!("and-lt-gt-eq-not"; "( && ( <= ?c0 ?x ) ( < ?x ?c1 ) )" => "(0)" if crate::trs::compare_c0_c1("?c1", "?c0", "<=")),

]}