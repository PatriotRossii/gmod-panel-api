use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

use gmod_panel_api::db::{models::{BillingInfo, NewBillingInfo}, establish_connection};

pub fn insert_billing_info(path: web::Query<NewBillingInfo>) -> HttpResponse {
    let connection = establish_connection();
    BillingInfo::insert(
        &connection,
        path.into_inner()
    );
    HttpResponse::Ok().body("success")
}

async fn get_billing_info(path: web::Path<(i32,)>) -> Result<HttpResponse, Error> {
    let connection = establish_connection();
    let user_id = path.into_inner().0;

    let info = web::block(move || BillingInfo::find_by_id(&connection, user_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    
    if let Some(info) = info {
        Ok(HttpResponse::Ok().json(info))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with id: {}", user_id));
        Ok(res)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/billing_info")
                    .service(web::resource("/insert").route(web::get().to(insert_billing_info)))
                    .service(web::resource("/{id}").route(web::get().to(get_billing_info)))
            )
    }).bind("127.0.0.1:8080")?.run().await
}