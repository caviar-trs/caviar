use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn not() -> Vec<Rewrite> {
    vec![
        // NOT RULES
        rw!("eqlt-to-not-gt";  "(<= ?x ?y)"     <=> "(! (< ?y ?x))" ),
        rw!("not-gt-to-eqlt";  "(! (< ?y ?x))"  <=> "(<= ?x ?y)" ),
        rw!("eqgt-to-not-lt";  "(>= ?x ?y)"     <=> "(! (< ?x ?y))" ),
        rw!("not-eq-to-ineq";  "(! (== ?x ?y))" <=> "(!= ?x ?y)" ),
        rw!("not-not"       ;  "(! (! ?x))"     <=> "?x" ),
    ]
    .concat()
}
