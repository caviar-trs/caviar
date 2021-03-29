use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn eq() -> Vec<Rewrite> { vec![
     // Equality RULES
     rw!("eq-comm"       ; "(== ?x ?y)"           => "(== ?y ?x)"),
     rw!("eq-swap"       ; "(== (+ ?x ?y) ?z)"    => "(== ?x (- ?z ?y))"),
     rw!("eq-x-x"        ; "(== ?x ?x)"           => "true"),
     rw!("eq-mul-x-y-0"  ; "(== (* ?x ?y) 0)"     => "(|| (== ?x 0) (== ?y 0))"),
     rw!("eq-max-lt"     ; "( == (max ?x ?y) ?y)" => "(<= ?x ?y)"),
     rw!("Eq-min-lt"     ; "( == (min ?x ?y) ?y)" => "(<= ?y ?x)"),
     rw!("Eq-lt-min"     ; "(<= ?y ?x)"           => "( == (min ?x ?y) ?y)"),
     rw!("Eq-max-c-pos"  ; "(== (max ?x ?c) 0)"   => "false" if crate::trs::is_const_pos("?c")),
     rw!("Eq-max-c-neg"  ; "(== (max ?x ?c) 0)"   => "(== ?x 0)" if crate::trs::is_const_neg("?c")),
     rw!("Eq-min-c-pos"  ; "(== (min ?x ?c) 0)"   => "true" if crate::trs::is_const_neg("?c")),
     rw!("Eq-min-c-neg"  ; "(== (min ?x ?c) 0)"   => "(== ?x 0)" if crate::trs::is_const_pos("?c")),
     
     // rw!("Eq-max-2"; "(<= ?x ?y)" => "( == (max ?x ?y) ?y)"),//NOTAXIOM
     // OLD RULES ( NOT SURE IF NEEDED OR NOT )
     // rw!("one-Eq";  "(== ?x 1)"        => "(?x)"),
     // rw!("zero-Eq";  "(== ?x 0)"        => "(! ?x)"),
     // rw!("sub-let-max-Eq";  "(- (max ?x ?y) ?y)"        => "(<= ?x ?y)"),
     // rw!("sub-let-min-Eq";  "(- (min ?x ?y) ?y)"        => "(<= ?y ?x)"),
     // rw!("sub-let-max-Eq-1";  "(- ?y (max ?x ?y))"        => "(<= ?x ?y)"),
     // rw!("sub-let-min-Eq-1";  "(- ?y (min ?x ?y))"        => "(<= ?y ?x)"),
]}