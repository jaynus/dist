#![feature(futures_api, arbitrary_self_types, await_macro, async_await, proc_macro_hygiene)]

use dist_services::Service;
use futures::future::{TryFutureExt, FutureExt};

async fn run() -> std::io::Result<()> {
    let router = await!(dist_services::router::new())?;

    // register 3 workers
    let _ = await!(dist_services::worker::new(router.address()))?;

    let _ = await!(dist_services::worker::new(router.address()))?;

    let _ = await!(dist_services::worker::new(router.address()))?;

    // Register the worker
    println!("Done?");

    Ok(())
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    /*
    use slog::{o, Drain};

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());

    // slog_stdlog uses the logger from slog_scope, so set a logger there
    let _guard = slog_scope::set_global_logger(logger);

    // register slog_stdlog as the log handler with the log crate
    slog_stdlog::init().unwrap();
    */

    tarpc::init(futures::compat::TokioDefaultSpawner);

    tokio::run(run()
                   .map_err(|e| eprintln!("Oh no: {}", e))
                   .boxed()
                   .compat(),
    );
    Ok(())
    //pool.run(run(pool.clone()).map_err(|e| eprintln!("Oh no: {}", e))).map_err(|_| { std::io::Error::new(std::io::ErrorKind::Other, "lol")})
}
