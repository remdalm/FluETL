// @generated automatically by Diesel CLI.

diesel::table! {
    command (id_order) {
        id_order -> Unsigned<Integer>,
        id_client -> Unsigned<Integer>,
        #[max_length = 32]
        order_ref -> Varchar,
        date -> Datetime,
        #[max_length = 128]
        order_status -> Nullable<Varchar>,
        #[max_length = 128]
        delivery_status -> Nullable<Varchar>,
    }
}

diesel::table! {
    command_line (id_order_line) {
        id_order_line -> Unsigned<Integer>,
        id_order -> Unsigned<Integer>,
        #[max_length = 32]
        order_ref -> Varchar,
        #[max_length = 32]
        po_ref -> Varchar,
        #[max_length = 32]
        product_ref -> Varchar,
        #[max_length = 32]
        product_name -> Nullable<Varchar>,
        qty_ordered -> Unsigned<Integer>,
        qty_reserved -> Unsigned<Integer>,
        qty_delivered -> Unsigned<Integer>,
        due_date -> Datetime,
    }
}

diesel::table! {
    mapping_client_contact (idp_id_client) {
        idp_id_client -> Unsigned<Integer>,
        ps_id_customer -> Unsigned<Integer>,
    }
}

diesel::joinable!(command -> mapping_client_contact (id_client));
diesel::joinable!(command_line -> command (id_order));

diesel::allow_tables_to_appear_in_same_query!(command, command_line, mapping_client_contact,);
