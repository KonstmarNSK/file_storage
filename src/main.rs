use actix_web::{App, HttpServer, web};

mod handlers;
mod structs;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(handlers::add_fs_handlers))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

