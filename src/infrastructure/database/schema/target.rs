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
        total_tax_excl -> Decimal,
        total_tax_incl -> Decimal,
    }
}

diesel::table! {
    invoice_lang (id_invoice, id_lang) {
        id_invoice -> Unsigned<Integer>,
        id_lang -> Unsigned<Integer>,
        #[max_length = 255]
        type_name -> Varchar,
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
        qty_ordered -> Unsigned<Integer>,
        qty_reserved -> Unsigned<Integer>,
        qty_delivered -> Unsigned<Integer>,
        due_date -> Nullable<Date>,
    }
}

diesel::table! {
    order_line_lang (id_order_line, id_lang) {
        id_order_line -> Unsigned<Integer>,
        id_lang -> Unsigned<Integer>,
        #[max_length = 255]
        product_name -> Varchar,
    }
}

diesel::table! {
    product_substitute (id_product, id_substitute) {
        id_product -> Unsigned<Integer>,
        id_substitute -> Unsigned<Integer>,
    }
}

diesel::joinable!(invoice_lang -> invoice (id_invoice));
diesel::joinable!(order_line -> order (id_order));
diesel::joinable!(order_line_lang -> order_line (id_order_line));

diesel::allow_tables_to_appear_in_same_query!(
    delivery_slip,
    invoice,
    invoice_lang,
    mapping_client_contact,
    order,
    order_line,
    order_line_lang,
    product_substitute,
);
