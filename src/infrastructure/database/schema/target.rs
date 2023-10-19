// @generated automatically by Diesel CLI.

diesel::table! {
    delivery_slip (id_delivery_slip) {
        id_delivery_slip -> Unsigned<Integer>,
        id_client -> Unsigned<Integer>,
        #[max_length = 32]
        reference -> Varchar,
        shipping_date -> Nullable<Date>,
        #[max_length = 255]
        po_ref -> Nullable<Varchar>,
        #[max_length = 255]
        carrier_name -> Nullable<Varchar>,
        #[max_length = 128]
        status -> Nullable<Varchar>,
        #[max_length = 255]
        tracking_number -> Nullable<Varchar>,
        #[max_length = 255]
        tracking_link -> Nullable<Varchar>,
    }
}

diesel::table! {
    invoice (id_invoice) {
        id_invoice -> Unsigned<Integer>,
        id_client -> Unsigned<Integer>,
        #[max_length = 255]
        client_name -> Nullable<Varchar>,
        #[max_length = 32]
        invoice_ref -> Varchar,
        date -> Date,
        #[max_length = 255]
        file_name -> Nullable<Varchar>,
        #[max_length = 255]
        po_ref -> Nullable<Varchar>,
        #[sql_name = "type"]
        #[max_length = 128]
        type_ -> Varchar,
        total_tax_excl -> Decimal,
        total_tax_incl -> Decimal,
    }
}

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
        #[max_length = 255]
        client_name -> Nullable<Varchar>,
        #[max_length = 32]
        order_ref -> Varchar,
        date -> Datetime,
        #[max_length = 255]
        po_ref -> Nullable<Varchar>,
        #[max_length = 255]
        origin -> Nullable<Varchar>,
        completion -> Nullable<Unsigned<Integer>>,
        #[max_length = 2]
        order_status -> Nullable<Varchar>,
    }
}

diesel::table! {
    order_line (id_order_line) {
        id_order_line -> Unsigned<Integer>,
        id_order -> Unsigned<Integer>,
        #[max_length = 64]
        product_ref -> Varchar,
        #[max_length = 255]
        product_name -> Nullable<Varchar>,
        qty_ordered -> Unsigned<Integer>,
        qty_reserved -> Unsigned<Integer>,
        qty_delivered -> Unsigned<Integer>,
        due_date -> Nullable<Date>,
    }
}

diesel::joinable!(order_line -> order (id_order));

diesel::allow_tables_to_appear_in_same_query!(
    delivery_slip,
    invoice,
    mapping_client_contact,
    order,
    order_line,
);
