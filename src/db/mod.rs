pub mod models;
pub mod schema;

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
}

impl ClientInfo {
    pub fn insert<'a>(conn: &SqliteConnection, nickname: &'a str, steam_id: Option<&'a str>, vkid: Option<&'a str>) {
        let client_info = NewClientInfo { nickname, steam_id, vkid };
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
    pub fn insert(conn: &SqliteConnection, server_id: i32, module_id: i32, status: bool) {
        let cmodule_info = ConnectedModuleInfo { server_id, module_id, status };
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
    pub fn insert(conn: &SqliteConnection, name: &str, cost: i32, author: i32) {
        let module_info = NewModuleInfo { name, cost, author };
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
    pub fn insert<'a>(conn: &SqliteConnection, client_id: i32, name: &'a str, ip: &'a str, password: &'a str) {
        let nserver_info = NewServerInfo { client_id, name, ip, password };
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
}