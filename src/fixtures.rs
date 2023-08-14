// Order

use crate::{
    domain::Order,
    infrastructure::{csv_reader::{CsvMappingClientDTO, CsvOrderDTO}, database::models::{order::OrderModel, mapping_client::MappingClientModel}},
};

pub const ORDER_CSV: &str= 
    "c_order_id;c_bpartner_id;name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Order 1;2023-08-01;Ref1;PoRef1;Origin1;30;done;done\n2;2;Order 2;2023-08-02;Ref2;PoRef2;Origin2;20;failed;done\n";
pub const MAPPING_CLIENT_CSV: &str =
    "c_bpartner_id;ad_user_id;name;company\n1;1;Order 1;2023-08-01\n1;2;Order 2;2023-08-02\n";

pub fn csv_order_dto_fixtures() -> [CsvOrderDTO; 2] {
    [
        CsvOrderDTO {
            c_order_id: 1.to_string(),
            c_bpartner_id: 1.to_string(),
            name: "Order 1".to_string(),
            date: "2023-08-01".to_string(),
            order_ref: "Ref1".to_string(),
            po_ref: "PoRef1".to_string(),
            origin: "Origin1".to_string(),
            completion: "30".to_string(),
            order_status: "done".to_string(),
            delivery_status: "done".to_string(),
        },
        CsvOrderDTO {
            c_order_id: 2.to_string(),
            c_bpartner_id: 2.to_string(),
            name: "Order 2".to_string(),
            date: "2023-08-02".to_string(),
            order_ref: "Ref2".to_string(),
            po_ref: "PoRef2".to_string(),
            origin: "Origin2".to_string(),
            completion: "20".to_string(),
            order_status: "failed".to_string(),
            delivery_status: "done".to_string(),
        },
    ]
}

pub fn order_fixtures() -> [Order; 2] {
    [
        Order::new(
            1,
            1,
            "Order 1".to_string(),
            chrono::NaiveDate::from_ymd_opt(2023, 8, 1).expect("Invalid date"),
            "Ref1".to_string(),
            "PoRef1".to_string(),
            "Origin1".to_string(),
            30,
            "done".to_string(),
            "done".to_string(),
        )
        .unwrap(),
        Order::new(
            2,
            2,
            "Order 2".to_string(),
            chrono::NaiveDate::from_ymd_opt(2023, 8, 2).expect("Invalid date"),
            "Ref2".to_string(),
            "PoRef2".to_string(),
            "Origin2".to_string(),
            20,
            "failed".to_string(),
            "done".to_string(),
        )
        .unwrap(),
    ]
}

pub fn order_model_fixture() -> OrderModel {
    OrderModel::new(1, 1, "Ref1".to_string(), chrono::Utc::now().naive_utc())
}

pub fn mapping_client_fixtures() -> [CsvMappingClientDTO; 2] {
    [
        CsvMappingClientDTO {
            c_bpartner_id: 1.to_string(),
            ad_user_id: 1.to_string(),
        },
        CsvMappingClientDTO {
            c_bpartner_id: 1.to_string(),
            ad_user_id: 2.to_string(),
        },
    ]
}

pub fn mapping_client_model_fixture() -> MappingClientModel {
    MappingClientModel::new(1, 1)
}
