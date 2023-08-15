use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::{
    domain::Order,
    infrastructure::{csv_reader::CsvOrderDTO, database::models::{order::OrderModel, mapping_client::MappingClientModel}},
};

pub const ORDER_FLAWLESS_CSV: &str= 
    "c_order_id;c_bpartner_id;name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Order 1;2023-08-01;Ref1;PoRef1;Origin1;30;done;done\n2;2;Order 2;2023-08-02;Ref2;PoRef2;Origin2;20;failed;done\n";
pub const ORDER_WITH_EMPTY_FIELD_CSV: &str =
    "c_order_id;c_bpartner_id;name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Order 1;2023-08-01;Ref1;PoRef1;Origin1;30;;\n3;1;Order 3;2023-08-03;Ref3;PoRef3;Origin3;0;;done\n";

pub const ORDER_WITH_MISSING_DATA_CSV: &str =
    "c_order_id;c_bpartner_id;name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Order 1;2023-08-01;Ref1;PoRef1;Origin1;30\n2;2;Order 2;2023-08-02;Ref2;PoRef2;Origin2;20\n";

pub fn csv_order_dto_fixtures() -> [CsvOrderDTO; 3] {
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
        CsvOrderDTO {
            c_order_id: 3.to_string(),
            c_bpartner_id: 1.to_string(),
            name: "Order 3".to_string(),
            date: "2023-08-03".to_string(),
            order_ref: "Ref3".to_string(),
            po_ref: "PoRef3".to_string(),
            origin: "Origin3".to_string(),
            completion: "0".to_string(),
            order_status: String::new(),
            delivery_status: "done".to_string(),
        }
    ]
}

pub fn order_fixtures() -> [Order; 3] {
    [
        Order::new(
            1,
            1,
            "Order 1".to_string(),
            chrono::NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
            "Ref1".to_string(),
            "PoRef1".to_string(),
            "Origin1".to_string(),
            30,
            Some("done".to_string()),
            Some("done".to_string()),
        )
        .unwrap(),
        Order::new(
            2,
            2,
            "Order 2".to_string(),
            chrono::NaiveDate::from_ymd_opt(2023, 8, 2).unwrap(),
            "Ref2".to_string(),
            "PoRef2".to_string(),
            "Origin2".to_string(),
            20,
            Some("failed".to_string()),
            Some("done".to_string()),
        )
        .unwrap(),
        Order::new(
            3,
            1,
            "Order 3".to_string(),
            chrono::NaiveDate::from_ymd_opt(2023, 8, 2).unwrap(),
            "Ref3".to_string(),
            "PoRef3".to_string(),
            "Origin3".to_string(),
            30,
            None,
            Some("done".to_string()),
        )
        .unwrap(),
    ]
}

pub fn order_model_fixtures() -> [OrderModel;3] {
    [
    OrderModel::new(
        1, 1, "Ref1".to_string(), NaiveDateTime::new(NaiveDate:: from_ymd_opt(2023, 8, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap()), Some("done".to_string()), Some("done".to_string())
    ),
    OrderModel::new(
        2, 2, "Ref2".to_string(), NaiveDateTime::new(NaiveDate:: from_ymd_opt(2023, 8, 2).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap()), Some("failed".to_string()), Some("done".to_string())
    ),
    OrderModel::new(
        3, 1, "Ref3".to_string(), NaiveDateTime::new(NaiveDate:: from_ymd_opt(2023, 8, 3).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap()), None, Some("done".to_string())
    ),
    ]
}

pub fn mapping_client_model_fixture() -> [MappingClientModel;2] {
    [MappingClientModel::new(1, 1), MappingClientModel::new(2, 2)]
}
