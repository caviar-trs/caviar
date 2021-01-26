use egg::{rewrite as rw};
use crate::trs::Math;
use crate::trs::ConstantFold;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn eq() -> Vec<Rewrite> { vec![
     // Equality RULES
     rw!("comm-Eq";  "(== ?x ?y)"        => "(== ?y ?x)"),
     rw!("other-side-Eq";  "(== (+ ?x ?y) ?z)"        => "(== ?x (- ?z ?y))"),
     // rw!("one-Eq";  "(== ?x 1)"        => "(?x)"),
     // rw!("zero-Eq";  "(== ?x 0)"        => "(! ?x)"),
     rw!("x-x-Eq";  "(== ?x ?x)"        => "1"),
     rw!("sub-let-max-Eq";  "(- (max ?x ?y) ?y)"        => "(<= ?x ?y)"),
     rw!("sub-let-min-Eq";  "(- (min ?x ?y) ?y)"        => "(<= ?y ?x)"),
     rw!("sub-let-max-Eq-1";  "(- ?y (max ?x ?y))"        => "(<= ?x ?y)"),
     rw!("sub-let-min-Eq-1";  "(- ?y (min ?x ?y))"        => "(<= ?y ?x)"),
]}