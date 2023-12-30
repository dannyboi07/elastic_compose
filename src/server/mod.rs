mod route;
mod schema;

use axum::Router;
use core::fmt;
use route::BasicRouter;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};
use tokio::signal;
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
    shutdown_handler: Option<Box<dyn Fn() + Send>>,
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

    fn mount_middlewares(router: Router) -> Router {
        tracing_subscriber::fmt::init();

        return router
            .layer(ServiceBuilder::new())
            .layer(TraceLayer::new_for_http())
            .layer(TimeoutLayer::new(Duration::from_secs(5)));
    }

    pub fn new(port: u16) -> Server {
        return Server {
            port,
            shutdown_handler: None,
        };
    }

    pub fn with_shutdown_handler(mut self, f: impl Fn() + Send + 'static) -> Self {
        self.shutdown_handler = Some(Box::new(f));
        return self;
    }

    /// Consumes `self`!
    #[tokio::main]
    pub async fn start(self) -> Result<(), ServerError> {
        let listener = match tokio::net::TcpListener::bind(SocketAddrV4::new(
            Ipv4Addr::new(0, 0, 0, 0),
            self.port,
        ))
        .await
        {
            Err(err) => return Err(ServerError::ListenError(err.to_string())),
            Ok(listener) => listener,
        };

        let router = Server::mount_middlewares(Server::get_router());
        println!("Server starting on port: {}...", self.port);
        match axum::serve(listener, router)
            .with_graceful_shutdown(self.handle_shutdown_signal()) // Moving of `self`! Moving self instead of using references since `with_graceful_shutdown` only accepts `impl Future` instead of `&impl Future`
            .await
        {
            Err(err) => return Err(ServerError::ServeError(err.to_string())),
            Ok(_) => {
                println!("Server shutdown");
            }
        };

        return Ok(());
    }

    fn handle_shutdown_cleanup(&self) {
        if let Some(handler) = &self.shutdown_handler {
            handler();
        };
    }

    async fn handle_shutdown_signal(self) {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler")
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to install SIGTERM handler")
                .recv()
                .await
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {
                println!("Received Ctrl+C, shutting down...");
                self.handle_shutdown_cleanup();
            },
            _ = terminate => {
                println!("Received SIGTERM, shutting down...");
                self.handle_shutdown_cleanup();
            },
        }
    }
}
