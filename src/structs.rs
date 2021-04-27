use std::usize;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ResultStructure {
    index: i16,
    start_expression: String,
    end_expression: String,
    result: bool,
    best_expr: String,
    class: i64,
    iterations: usize,
    egraph_size: usize,
    rebuilds: usize,
    pub total_time: f64,
    stop_reason: String,
    condition: Option<String>,
}

impl ResultStructure {
    pub fn new(
        index: i16,
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
    ) -> Self {
        Self {
            index,
            start_expression,
            end_expression,
            result,
            best_expr,
            class,
            iterations,
            egraph_size,
            rebuilds,
            total_time,
            stop_reason,
            condition,
        }
    }

    pub fn add_index_condition(&mut self, index: i16, condition: String) {
        self.index = index;
        self.condition = Some(condition);
    }
}

#[derive(Serialize, Debug)]
pub struct ExpressionStruct {
    pub index: i16,
    pub expression: String,
}

impl ExpressionStruct {
    pub fn new(index: i16, expression: String) -> Self {
        Self { index, expression }
    }
}

#[derive(Serialize, Debug)]
pub struct Rule {
    pub index: i16,
    pub lhs: String,
    pub rhs: String,
    pub condition: Option<String>,
}

impl Rule {
    #[allow(dead_code)]
    pub fn new(index: i16, lhs: String, rhs: String, condition: Option<String>) -> Self {
        Self {
            index,
            lhs,
            rhs,
            condition,
        }
    }
}
