use serde::{Serialize};

#[derive(Serialize)]
#[derive(Debug)]
pub struct ResultStructure {
    index: i16,
    start_expression: String,
    end_expressions: String,
    result: bool,
    best_expr: String,
    total_time: f64,
    condition: String,
}

impl ResultStructure{
    pub fn new(index: i16,
               start_expression: String,
               end_expressions: String,
               result: bool,
               best_expr: String,
               total_time: f64,
               condition: String)->Self{
        Self{
            index,
            start_expression,
            end_expressions,
            result,
            best_expr,
            total_time,
            condition,
        }
    }
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct ExpressionStruct{
    index: i16,
    expression: String,
}

impl ExpressionStruct{
    pub fn new(index: i16, expression: String) -> Self{
        Self{
            index, expression
        }
    }
}