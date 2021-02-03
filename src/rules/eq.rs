use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn eq() -> Vec<Rewrite> { vec![
     // Equality RULES
     rw!("comm-Eq";  "(== ?x ?y)"        => "(== ?y ?x)"),
     rw!("other-side-Eq";  "(== (+ ?x ?y) ?z)"        => "(== ?x (- ?z ?y))"),
     rw!("x-x-Eq";  "(== ?x ?x)"        => "1"),
     rw!("x-y-0";  "(== (* ?x ?y) 0)"        => "(|| (== ?x 0) (== ?y 0))"),
     rw!("Eq-max-1"; "( == (max ?x ?y) ?y)" => "(<= ?x ?y)"),
     rw!("Eq-max-2"; "(<= ?x ?y)" => "( == (max ?x ?y) ?y)"),
     rw!("Eq-min-1"; "( == (min ?x ?y) ?y)" => "(<= ?y ?x)"),
     rw!("Eq-min-2"; "(<= ?y ?x)" => "( == (min ?x ?y) ?y)"),
     
     rw!("Eq-max-c-1"; "(== (max ?x ?c) 0)" => "0" if crate::trs::is_const_pos("?c")),
     rw!("Eq-max-c-2"; "(== (max ?x ?c) 0)" => "(== ?x 0)" if crate::trs::is_const_neg("?c")),
     
     rw!("Eq-min-c-1"; "(== (min ?x ?c) 0)" => "0" if crate::trs::is_const_neg("?c")),
     rw!("Eq-min-c-2"; "(== (min ?x ?c) 0)" => "(== ?x 0)" if crate::trs::is_const_pos("?c")),

     // OLD RULES ( NOT SURE IF NEEDED OR NOT )
     // rw!("one-Eq";  "(== ?x 1)"        => "(?x)"),
     // rw!("zero-Eq";  "(== ?x 0)"        => "(! ?x)"),
     // rw!("sub-let-max-Eq";  "(- (max ?x ?y) ?y)"        => "(<= ?x ?y)"),
     // rw!("sub-let-min-Eq";  "(- (min ?x ?y) ?y)"        => "(<= ?y ?x)"),
     // rw!("sub-let-max-Eq-1";  "(- ?y (max ?x ?y))"        => "(<= ?x ?y)"),
     // rw!("sub-let-min-Eq-1";  "(- ?y (min ?x ?y))"        => "(<= ?y ?x)"),
]}