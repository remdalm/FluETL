// @generated automatically by Diesel CLI.

diesel::table! {
    language_list (locale) {
        #[max_length = 5]
        locale -> Varchar,
        id -> Integer,
    }
}

diesel::table! {
    staging_customer (id_source_contact) {
        id_source_client -> Integer,
        id_source_contact -> Integer,
        id -> Nullable<Integer>,
        id_shop -> Integer,
        m_pricelist_id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        company -> Nullable<Varchar>,
        #[max_length = 128]
        email -> Varchar,
        active -> Bool,
        is_xxa_centrale -> Bool,
        free_shipping_amount -> Integer,
        update_client -> Datetime,
        update_contact -> Datetime,
        is_synchronised -> Bool,
        has_error -> Bool,
        force_update -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(language_list, staging_customer,);
