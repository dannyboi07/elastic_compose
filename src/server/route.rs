use std::collections::HashMap;

use axum::routing::{get, post, MethodRouter};

pub struct BasicRouter {
    pub routes: HashMap<&'static str, MethodRouter>,
}

impl BasicRouter {
    pub fn new() -> BasicRouter {
        let mut routes: HashMap<&'static str, MethodRouter> = HashMap::new();
        routes.insert("/", get(BasicRouter::hello_world));
        BasicRouter { routes }
    }

    async fn hello_world() -> &'static str {
        return "Hello, World!";
    }
}
