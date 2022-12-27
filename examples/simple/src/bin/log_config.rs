use ohkami::prelude::*;

fn main() -> Result<()> {
    let config = Config {
        log_subscribe:
            Some(tracing_subscriber::fmt()
                .with_max_level(tracing::Level::TRACE)
            ),
        ..Default::default()
    };

    Server::with(config)
        .GET("/", || async {Response::OK("Hello!")})
        .serve_on(":5000")
}