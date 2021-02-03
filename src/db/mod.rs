pub mod models;
pub mod schema;

use actix_web::dev::Server;
use diesel::{prelude::*, sqlite::SqliteConnection};
use models::{*};

pub fn establish_connection() -> SqliteConnection {
    let db: &str = "./database.sqlite";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

impl BillingInfo {
    pub fn insert(conn: &SqliteConnection, billing_info: NewBillingInfo) {
        diesel::insert_into(schema::billing_info::table)
            .values(&billing_info)
            .execute(conn)
            .expect("Error inserting new billing info");
    }
    pub fn query(conn: &SqliteConnection) -> Vec<Self> {
        schema::billing_info::table
            .load::<BillingInfo>(conn)
            .expect("Error loading billing info")
    }
    pub fn find_by_id(
        conn: &SqliteConnection,
        id: i32
    ) -> Result<Option<BillingInfo>, diesel::result::Error> {
        use schema::billing_info::dsl::*;

        let info =
            billing_info
                .filter(client_id.eq(id))
                .first::<BillingInfo>(conn)
                .optional()?;
        Ok(info)        
    }
}

impl CardInfo {
    pub fn insert(conn: &SqliteConnection, card_info: NewCardInfo) {
        diesel::insert_into(schema::card_info::table)
            .values(&card_info)
            .execute(conn)
            .expect("Error inserting new card info");
    }
    pub fn query(conn: &SqliteConnection) -> Vec<Self> {
        schema::card_info::table
            .load::<CardInfo>(conn)
            .expect("Error loading card info")
    }
    pub fn find_by_id(
        conn: &SqliteConnection,
        id_: i32
    ) -> Result<Option<CardInfo>, diesel::result::Error> {
        use schema::card_info::dsl::*;
        let info =
            card_info
                .filter(id.eq(id_))
                .first::<CardInfo>(conn)
                .optional()?;
        Ok(info)
    }
}

impl ClientInfo {
    pub fn insert<'a>(conn: &SqliteConnection, client_info: NewClientInfo) {
        diesel::insert_into(schema::clients::table)
            .values(&client_info)
            .execute(conn)
            .expect("Error inserting new client info");
    }
    pub fn query(conn: &SqliteConnection) -> Vec<Self> {
        schema::clients::table
            .load::<ClientInfo>(conn)
            .expect("Error loading client info")
    }
}

impl ConnectedModuleInfo {
    pub fn insert(conn: &SqliteConnection, cmodule_info: ConnectedModuleInfo) {
        diesel::insert_into(schema::connected_modules::table)
            .values(&cmodule_info)
            .execute(conn)
            .expect("Error inserting new connected module info");
    }
    pub fn query(conn: &SqliteConnection) -> Vec<Self> {
        schema::connected_modules::table
            .load::<ConnectedModuleInfo>(conn)
            .expect("Error loading new connected module info")
    }
}

impl ModuleInfo {
    pub fn insert(conn: &SqliteConnection, module_info: NewModuleInfo) {
        diesel::insert_into(schema::modules::table)
            .values(&module_info)
            .execute(conn)
            .expect("Error inserting new module info");
    }
    pub fn query(conn: &SqliteConnection) -> Vec<Self> {
        schema::modules::table
            .load::<ModuleInfo>(conn)
            .expect("Error loading new module info")
    }
}

impl ServerInfo {
    pub fn insert<'a>(conn: &SqliteConnection, nserver_info: NewServerInfo) {
        diesel::insert_into(schema::servers::table)
            .values(&nserver_info)
            .execute(conn)
            .expect("Error inserting new server info");
    }
    pub fn query(conn: &SqliteConnection) -> Vec<Self> {
        schema::servers::table
            .load::<ServerInfo>(conn)
            .expect("Error loading new server info")
    }
    pub fn find_by_id(conn: &SqliteConnection, id_: i32) -> Result<Option<ServerInfo>, diesel::result::Error> {
        use schema::servers::dsl::*;
        let info = servers
            .filter(id.eq(id_))
            .first::<ServerInfo>(conn)
            .optional()?;
        Ok(info)        
    }
    pub fn find_by_client_id(conn: &SqliteConnection, client_id_: i32) -> Result<Option<ServerInfo>, diesel::result::Error> {
        use schema::servers::dsl::*;
        let info = servers
            .filter(client_id.eq(client_id_))
            .first::<ServerInfo>(conn)
            .optional()?;
        Ok(info)
    }
}