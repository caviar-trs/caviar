use serde::Serialize;

#[derive(Serialize)]
#[derive(Debug)]
pub struct ResultStructure {
    index: i16,
    start_expression: String,
    end_expression: String,
    result: bool,
    best_expr: String,
    pub total_time: f64,
    condition: Option<String>,
}

impl ResultStructure {
    pub fn new(index: i16,
               start_expression: String,
               end_expression: String,
               result: bool,
               best_expr: String,
               total_time: f64,
               condition: Option<String>) -> Self {
        Self {
            index,
            start_expression,
            end_expression,
            result,
            best_expr,
            total_time,
            condition,
        }
    }
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct ExpressionStruct {
    pub index: i16,
    pub expression: String,
}

impl ExpressionStruct {
    pub fn new(index: i16, expression: String) -> Self {
        Self {
            index,
            expression,
        }
    }
}

#[derive(Serialize)]
#[derive(Debug)]
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