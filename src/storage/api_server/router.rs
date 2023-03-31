// src/storage/api_server/router.rs

use crate::storage::api_server::handler::Handler;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Router {
    routes: HashMap<String, Arc<dyn Handler + Send + Sync>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<H>(&mut self, path: &str, handler: H)
    where
        H: Handler + Send + Sync + 'static,
    {
        self.routes.insert(path.to_string(), Arc::new(handler));
    }

    pub fn route(&self, request: &str) -> Option<Arc<dyn Handler + Send + Sync>> {
        // Implement a simple request parsing to extract the path from the request
        let path = request.split_whitespace().nth(1).unwrap_or("/").to_string();

        self.routes.get(&path).cloned()
    }
}
