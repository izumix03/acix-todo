use actix_web::{get, middleware, App, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

#[get("/hello")]
async fn hello_world() -> impl Responder {
    "Hello World@".to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut listenfd = ListenFd::from_env();
    let server = HttpServer::new(|| {
        App::new()
            .service(hello_world)
            .wrap(middleware::Logger::default())
    });

    env_logger::init();

    // systemfd による起動は指定コマンドに従い、それ以外は .env の設定に従う
    let bound_server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    bound_server.run().await
}
