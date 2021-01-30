use super::schema::*;

#[derive(Insertable, Queryable)]
#[table_name = "billing_info"]
pub struct BillingInfo<'a> {
    pub client_id: i32,
    pub phone_number: &'a str,
    pub card_info: i32,
}

#[derive(Insertable)]
#[table_name = "card_info"]
pub struct NewCardInfo {
    pub number: i32,
    pub expires: i32,
    pub cvv: i32,
}

#[derive(Queryable)]
pub struct CardInfo {
    pub id: i32,
    pub number: i32,
    pub expires: i32,
    pub cvv: i32,
}

#[derive(Insertable, Queryable)]
#[table_name = "clients"]
pub struct ClientInfo<'a> {
    pub id: i32,
    pub nickname: &'a str,
    pub steam_id: &'a str,
    pub vkid: &'a str
}

#[derive(Insertable, Queryable)]
#[table_name = "connected_modules"]
pub struct ConnectedModuleInfo {
    pub server_id: i32,
    pub module_id: i32,
    pub status: bool,
}

#[derive(Insertable)]
#[table_name = "modules"]
pub struct NewModuleInfo<'a> {
    pub name: &'a str,
    pub cost: i32,
    pub author: i32,
}

#[derive(Queryable)]
pub struct ModuleInfo<'a> {
    pub id: i32,
    pub name: &'a str,
    pub cost: i32,
    pub author: i32,
}

#[derive(Insertable)]
#[table_name = "servers"]
pub struct NewServerInfo<'a> {
    pub id: i32,
    pub client_id: i32,
    pub name: &'a str,
    pub ip: &'a str,
    pub password: &'a str,
}

#[derive(Queryable)]
pub struct ServerInfo<'a> {
    pub client_id: i32,
    pub name: &'a str,
    pub ip: &'a str,
    pub password: &'a str,
}

