table! {
    billing_info (client_id) {
        client_id -> Nullable<Integer>,
        phone_number -> Nullable<Text>,
        card_info -> Nullable<Integer>,
    }
}

table! {
    card_info (id) {
        id -> Nullable<Integer>,
        number -> Nullable<Integer>,
        expires -> Nullable<Integer>,
        cvv -> Nullable<Integer>,
    }
}

table! {
    clients (id) {
        id -> Nullable<Integer>,
        nickname -> Nullable<Text>,
        steam_id -> Nullable<Text>,
        vkid -> Nullable<Text>,
    }
}

table! {
    connected_modules (server_id) {
        server_id -> Nullable<Integer>,
        module_id -> Nullable<Integer>,
        status -> Nullable<Bool>,
    }
}

table! {
    modules (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        cost -> Nullable<Text>,
        author -> Nullable<Integer>,
    }
}

table! {
    servers (id) {
        id -> Nullable<Integer>,
        client_id -> Nullable<Integer>,
        name -> Nullable<Text>,
        ip -> Nullable<Integer>,
        password -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    billing_info,
    card_info,
    clients,
    connected_modules,
    modules,
    servers,
);
