#[macro_use]
extern crate diesel;

use actix_web::{get, web, App, HttpServer, Result, Responder};

mod actions;
mod link;
mod models;
mod schema;

#[get("/{path}")]
async fn redirect(db: web::Data<link::Storage>, path: web::Path<String>) -> Result<impl Responder> {
    let path = path.into_inner();

    let url = web::block(move || {
        let db = db.into_inner();
        db.get_by_id(path.as_ref())
    })
    .await?;

    match url {
        Ok(url) => Ok(web::Redirect::to(url.url)),
        Err(e) => Err(e.into()), 
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let app_data = web::Data::new(link::Storage::new());

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(redirect))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
