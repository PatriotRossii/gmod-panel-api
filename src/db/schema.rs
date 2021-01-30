table! {
    billing_info (client_id) {
        client_id -> Integer,
        phone_number -> Nullable<Text>,
        card_info -> Nullable<Integer>,
    }
}

table! {
    card_info (id) {
        id -> Integer,
        number -> Integer,
        expires -> Integer,
        cvv -> Integer,
    }
}

table! {
    clients (id) {
        id -> Integer,
        nickname -> Text,
        steam_id -> Nullable<Text>,
        vkid -> Nullable<Text>,
    }
}

table! {
    connected_modules (server_id) {
        server_id -> Integer,
        module_id -> Integer,
        status -> Bool,
    }
}

table! {
    modules (id) {
        id -> Integer,
        name -> Text,
        cost -> Integer,
        author -> Integer,
    }
}

table! {
    servers (id) {
        id -> Integer,
        client_id -> Integer,
        name -> Text,
        ip -> Text,
        password -> Text,
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
