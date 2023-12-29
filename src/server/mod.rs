mod route;
mod schema;

use axum::Router;
use core::fmt;
use route::BasicRouter;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

pub enum ServerError {
    ListenError(String),
    ServeError(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::ListenError(err) => {
                writeln!(f, "Failed to initialize TCP listener, err: {}", err)
            }
            ServerError::ServeError(err) => {
                writeln!(f, "Failed to initialize server, err: {}", err)
            }
        }
    }
}

pub struct Server {
    port: u16,
    // router: Router,
}

impl Server {
    fn get_router() -> Router {
        let mut router = Router::new();
        for (route, handler) in BasicRouter::new().routes {
            // Using std::mem::take() since axum::Router::route() takes ownership of the router
            router = std::mem::take(&mut router).route(route, handler);
        }

        return router;
    }

    fn mount_middlewares(router: &mut Router) -> Router {
        tracing_subscriber::fmt::init();

        return std::mem::take(router)
            .layer(ServiceBuilder::new())
            .layer(TraceLayer::new_for_http())
            .layer(TimeoutLayer::new(Duration::from_secs(5)));
    }

    pub fn new(port: u16) -> Server {
        return Server { port };
    }

    #[tokio::main]
    pub async fn start(&self) -> Result<(), ServerError> {
        let listener = match tokio::net::TcpListener::bind(SocketAddrV4::new(
            Ipv4Addr::new(0, 0, 0, 0),
            self.port,
        ))
        .await
        {
            Err(err) => return Err(ServerError::ListenError(err.to_string())),
            Ok(listener) => listener,
        };

        let router = Server::mount_middlewares(&mut Server::get_router());

        match axum::serve(listener, router).await {
            Err(err) => return Err(ServerError::ServeError(err.to_string())),
            Ok(_) => {
                println!("Server started on port {}", self.port);
            }
        };

        return Ok(());
    }
}
