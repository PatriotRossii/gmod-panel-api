use super::schema::*;

#[derive(Insertable)]
#[table_name = "billing_info"]
pub struct NewBillingInfo<'a> {
    pub client_id: i32,
    pub phone_number: Option<&'a str>,
    pub card_info: Option<i32>
}

#[derive(Queryable)]
pub struct BillingInfo {
    pub client_id: i32,
    pub phone_number: Option<String>,
    pub card_info: Option<i32>,
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

#[derive(Insertable)]
#[table_name = "clients"]
pub struct NewClientInfo<'a> {
    pub nickname: &'a str,
    pub steam_id: Option<&'a str>,
    pub vkid: Option<&'a str>,
}

#[derive(Queryable)]
pub struct ClientInfo {
    pub id: i32,
    pub nickname: String,
    pub steam_id: Option<String>,
    pub vkid: Option<String>
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
pub struct ModuleInfo {
    pub id: i32,
    pub name: String,
    pub cost: i32,
    pub author: i32,
}

#[derive(Insertable)]
#[table_name = "servers"]
pub struct NewServerInfo<'a> {
    pub client_id: i32,
    pub name: &'a str,
    pub ip: &'a str,
    pub password: &'a str,
}

#[derive(Queryable)]
pub struct ServerInfo {
    pub id: i32,
    pub client_id: i32,
    pub name: String,
    pub ip: String,
    pub password: String,
}

