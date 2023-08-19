// @generated automatically by Diesel CLI.

diesel::table! {
    mapping_client_contact (id_customer) {
        id_customer -> Unsigned<Integer>,
        idp_id_client -> Unsigned<Integer>,
    }
}

diesel::table! {
    order (id_order) {
        id_order -> Unsigned<Integer>,
        id_client -> Unsigned<Integer>,
        #[max_length = 32]
        order_ref -> Varchar,
        date -> Datetime,
        #[max_length = 255]
        po_ref -> Nullable<Varchar>,
        completion -> Nullable<Unsigned<Integer>>,
        #[max_length = 128]
        order_status -> Nullable<Varchar>,
        #[max_length = 128]
        delivery_status -> Nullable<Varchar>,
    }
}

diesel::table! {
    order_line (id_order_line) {
        id_order_line -> Unsigned<Integer>,
        id_order -> Unsigned<Integer>,
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

diesel::joinable!(order_line -> order (id_order));

diesel::allow_tables_to_appear_in_same_query!(mapping_client_contact, order, order_line,);
