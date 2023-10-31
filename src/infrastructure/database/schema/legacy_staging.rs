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

diesel::table! {
    staging_product (id_source) {
        id_source -> Integer,
        id -> Nullable<Integer>,
        id_source_manufacturer -> Nullable<Integer>,
        isbom -> Bool,
        id_tax_rule -> Integer,
        #[max_length = 255]
        name_fr -> Varchar,
        #[max_length = 64]
        reference -> Varchar,
        #[max_length = 255]
        reference_irrijardin -> Nullable<Varchar>,
        price -> Decimal,
        active -> Bool,
        #[max_length = 255]
        description_fr -> Nullable<Varchar>,
        weight -> Nullable<Decimal>,
        discontinued -> Bool,
        diametre_ext -> Integer,
        diametre_int -> Integer,
        entraxe_2_fixations -> Integer,
        entraxe_diam -> Integer,
        entraxe_largeur -> Integer,
        entraxe_longueur -> Integer,
        epaisseur -> Integer,
        hauteur -> Integer,
        largeur_ext -> Integer,
        largeur_int -> Integer,
        longueur_ext -> Integer,
        longueur_int -> Integer,
        diametre_int_bas -> Integer,
        diametre_int_haut -> Integer,
        replenishment_time -> Nullable<Integer>,
        available_date -> Nullable<Datetime>,
        has_trace_warehouse -> Bool,
        update_date -> Datetime,
        is_synchronised -> Bool,
        has_error -> Bool,
        force_update -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(language_list, staging_customer, staging_product,);
