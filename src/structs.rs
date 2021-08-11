use std::usize;

use serde::Serialize;

#[derive(Serialize, Debug)]
/// The `structs` module contains a number of useful structs.

///The `ResultStructure` type is used to represent the result of proving or simplifying an expression
pub struct ResultStructure {
    //index of the expression set to make debugging easier
    index: i32,
    // The expression to be proved or simplified
    start_expression: String,
    // The goal to prove
    end_expression: String,
    // The result of the prover true means we could prove it.
    pub result: bool,
    // The simplest representation extracted
    best_expr: String,
    //The AST depth of the end expression
    ast_depth: usize,
    //The id of the cluster that was used to prove the expression in case we used clusters
    class: i64,
    //Number of iterations used to prove the expression
    iterations: usize,
    //The size of the egraph used to prove the expression
    egraph_size: usize,
    //The number of rebuilds used to prove the expression
    rebuilds: usize,
    //The time it took to prove the expression
    pub total_time: f64,
    // The reason the execution stopped
    stop_reason: String,
    //The condition of the rule
    condition: Option<String>,
    // Halide's result for proving the expression
    halide_result: String,
    // The time it took halide to prove the expression
    halide_time: f64,
}

impl ResultStructure {
    //Constructor for the ResultStructure
    pub fn new(
        index: i32,
        start_expression: String,
        end_expression: String,
        result: bool,
        best_expr: String,
        class: i64,
        iterations: usize,
        egraph_size: usize,
        rebuilds: usize,
        total_time: f64,
        stop_reason: String,
        condition: Option<String>,
        // halide_result: bool,
        // halide_time: f64
    ) -> Self {
        Self {
            index,
            start_expression,
            end_expression,
            result,
            best_expr,
            ast_depth: 0,
            class,
            iterations,
            egraph_size,
            rebuilds,
            total_time,
            stop_reason,
            condition,
            halide_result: "false".to_string(),
            halide_time: 0.0,
        }
    }

    pub fn new_depth(
        index: i32,
        start_expression: String,
        end_expression: String,
        result: bool,
        best_expr: String,
        ast_depth: usize,
        class: i64,
        iterations: usize,
        egraph_size: usize,
        rebuilds: usize,
        total_time: f64,
        stop_reason: String,
        condition: Option<String>,
        // halide_result: bool,
        // halide_time: f64
    ) -> Self {
        Self {
            index,
            start_expression,
            end_expression,
            result,
            best_expr,
            ast_depth,
            class,
            iterations,
            egraph_size,
            rebuilds,
            total_time,
            stop_reason,
            condition,
            halide_result: "false".to_string(),
            halide_time: 0.0,
        }
    }

    //adds index and the condition to the result
    pub fn add_index_condition(&mut self, index: i32, condition: String) {
        self.index = index;
        self.condition = Some(condition);
    }

    pub fn add_halide(&mut self, halide_result: String, halide_time: f64) {
        self.halide_result = halide_result;
        self.halide_time = halide_time;
    }
}

//The `ExpressionStruct` type is used to represent an expression
#[derive(Serialize, Debug)]
pub struct ExpressionStruct {
    //index of the expression
    pub index: i32,
    // the string of the expression
    pub expression: String,
    // Halide's result for proving the expression
    pub halide_result: String,
    // The time it took halide to prove the expression
    pub halide_time: f64,
}

impl ExpressionStruct {
    //Constructor of ExpressionStruct
    pub fn new(index: i32, expression: String, halide_result: String, halide_time: f64) -> Self {
        Self {
            index,
            expression,
            halide_result,
            halide_time,
        }
    }
}

//The `Rule` type is used to represent a a Rule
#[derive(Serialize, Debug)]
pub struct Rule {
    //index of the rule
    pub index: i32,
    // the LHS of the rule
    pub lhs: String,
    // the RHS of the rule
    pub rhs: String,
    // The condition to apply the rule
    pub condition: Option<String>,
}

impl Rule {
    // Constructor of Rule
    #[allow(dead_code)]
    pub fn new(index: i32, lhs: String, rhs: String, condition: Option<String>) -> Self {
        Self {
            index,
            lhs,
            rhs,
            condition,
        }
    }
}

//a Structure used the result of  special expressions issued from halide for the implementation of the paper.
#[derive(Serialize, Debug)]
pub struct PaperResult {
    infix: String,
    prefix: String,
    result: i8,
}

impl PaperResult {
    pub fn new(infix: String, prefix: String, result: i8) -> Self {
        Self {
            infix,
            prefix,
            result,
        }
    }
}
