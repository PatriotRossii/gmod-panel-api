use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use diesel::{SqliteConnection, r2d2::{self, ConnectionManager}, sqlite::Sqlite};
use serde::{Deserialize, Serialize};

use gmod_panel_api::db::{establish_connection, models::{BillingInfo, NewBillingInfo, NewCardInfo, CardInfo}};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// BILLING INFO 

pub fn insert_billing_info(pool: web::Data<DbPool>, path: web::Query<NewBillingInfo>) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");
    BillingInfo::insert(
        &connection,
        path.into_inner()
    );
    HttpResponse::Ok().body("success")
}
#[derive(Deserialize)]
struct GetBillingInfoQuery {
    pub client_id: i32,
}

async fn get_billing_info(pool: web::Data<DbPool>, path: web::Query<GetBillingInfoQuery>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let client_id = path.into_inner().client_id;

    let info = web::block(move || BillingInfo::find_by_id(&connection, client_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    
    if let Some(info) = info {
        Ok(HttpResponse::Ok().json(info))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with id: {}", client_id));
        Ok(res)
    }
}

// CARD INFO

pub fn insert_card_info(pool: web::Data<DbPool>, path: web::Query<NewCardInfo>) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");
    CardInfo::insert(
        &connection,
        path.into_inner()
    );
    HttpResponse::Ok().body("success")
}

#[derive(Deserialize)]
struct GetCardInfoQuery {
    pub id: i32,
}

async fn get_card_info(pool: web::Data<DbPool>, path: web::Query<GetCardInfoQuery>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let id = path.into_inner().id;

    let info = web::block(move || CardInfo::find_by_id(&connection, id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    
    if let Some(info) = info {
        Ok(HttpResponse::Ok().json(info))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with id: {}", id));
        Ok(res)
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let connspec = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/billing_info")
                    .service(web::resource("/insert").route(web::get().to(insert_billing_info)))
                    .service(web::resource("/get").route(web::get().to(get_billing_info)))
            )
            .service(
                web::scope("/card_info")
                    .service(web::resource("/insert").route(web::get().to(insert_card_info)))
                    .service(web::resource("/get").route(web::get().to(get_card_info)))
            )
    }).bind("127.0.0.1:8080")?.run().await
}