// src/database/query/executor.rs

use crate::database::query::planner::PlanNode;
use crate::database::{Database, Record};

pub struct Executor<'a> {
    database: &'a mut Database,
}

impl<'a> Executor<'a> {
    pub fn new(database: &'a mut Database) -> Self {
        Self { database }
    }

    pub fn execute(&mut self, plan: &PlanNode) -> Result<Vec<Record>, String> {
        // Implement your query execution logic here
        // Based on the input execution plan, execute the query
        // and return the result as a vector of records
    }
}
