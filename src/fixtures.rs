use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::{
    domain::{order::{Order, Origin}, mapping_client::MappingClient},
    infrastructure::{csv_reader::CsvOrderDTO, database::models::{order::OrderModel, mapping_client::{MappingClientModel, MappingClientSource}}},
};

pub const ORDER_FLAWLESS_CSV: &str= 
    "c_order_id;c_bpartner_id;client_name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Client 1;2023-08-01;Ref1;PoRef1;Web;30;done;done\n2;2;Client 2;2023-08-02;Ref2;PoRef2;EDI;20;failed;done\n";
pub const ORDER_WITH_EMPTY_FIELD_CSV: &str =
    "c_order_id;c_bpartner_id;client_name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Client 1;2023-08-01;Ref1;PoRef1;Web;30;;\n3;1;;2023-08-03;Ref3;PoRef3;Origin3;0;;done\n";

pub const ORDER_WITH_MISSING_DATA_CSV: &str =
    "c_order_id;c_bpartner_id;client_name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Client 1;2023-08-01;Ref1;PoRef1;Web;30\n2;2;Client 2;2023-08-02;Ref2;PoRef2;EDI;20\n";

pub fn csv_order_dto_fixtures() -> [CsvOrderDTO; 3] {
    [
        CsvOrderDTO {
            c_order_id: 1.to_string(),
            c_bpartner_id: 1.to_string(),
            client_name: "Client 1".to_string(),
            date: "2023-08-01".to_string(),
            order_ref: "Ref1".to_string(),
            po_ref: "PoRef1".to_string(),
            origin: "Web".to_string(),
            completion: "30".to_string(),
            order_status: "done".to_string(),
            delivery_status: "done".to_string(),
        },
        CsvOrderDTO {
            c_order_id: 2.to_string(),
            c_bpartner_id: 2.to_string(),
            client_name: "Client 2".to_string(),
            date: "2023-08-02".to_string(),
            order_ref: "Ref2".to_string(),
            po_ref: "PoRef2".to_string(),
            origin: "EDI".to_string(),
            completion: "20".to_string(),
            order_status: "failed".to_string(),
            delivery_status: "done".to_string(),
        },
        CsvOrderDTO {
            c_order_id: 3.to_string(),
            c_bpartner_id: 1.to_string(),
            client_name: String::new(),
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
            Some("Client 1".to_string()),
            chrono::NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
            "Ref1".to_string(),
            Some("PoRef1".to_string()),
            Origin::Web,
            Some(30),
            Some("done".to_string()),
            Some("done".to_string()),
        )
        .unwrap(),
        Order::new(
            2,
            2,
            Some("Client 2".to_string()),
            chrono::NaiveDate::from_ymd_opt(2023, 8, 2).unwrap(),
            "Ref2".to_string(),
            Some("PoRef2".to_string()),
            Origin::EDI,
            Some(20),
            Some("failed".to_string()),
            Some("done".to_string()),
        )
        .unwrap(),
        Order::new(
            3,
            1,
            None,
            chrono::NaiveDate::from_ymd_opt(2023, 8,3).unwrap(),
            "Ref3".to_string(),
            None,
            Origin::Unknown,
            None,
            None,
            Some("done".to_string()),
        )
        .unwrap(),
    ]
}

pub fn order_model_fixtures() -> [OrderModel;3] {
    [
        OrderModel{
            id_order: 1,
            id_client: 1,
            client_name: Some("Client 1".to_string()),
            order_ref: "Ref1".to_string(),
            po_ref: Some("PoRef1".to_string()),
            origin: Some("Web".to_string()),
            completion: Some(30),
            date: NaiveDateTime::new(NaiveDate:: from_ymd_opt(2023, 8, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap()),
            order_status: Some("done".to_string()),
            delivery_status: Some("done".to_string()),
        },
        OrderModel{
            id_order: 2,
            id_client: 2,
            client_name: Some("Client 2".to_string()),
            order_ref: "Ref2".to_string(),
            po_ref: Some("PoRef2".to_string()),
            origin: Some("EDI".to_string()),
            completion: Some(20),
            date: NaiveDateTime::new(NaiveDate:: from_ymd_opt(2023, 8, 2).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap()),
            order_status: Some("failed".to_string()),
            delivery_status: Some("done".to_string())
        },
        OrderModel{
            id_order: 3,
            id_client: 1,
            client_name: None,
            order_ref: "Ref3".to_string(),
            po_ref: None,
            origin: None,
            completion: None,
            date: NaiveDateTime::new(NaiveDate:: from_ymd_opt(2023, 8, 3).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap()),
            order_status: None,
            delivery_status: Some("done".to_string())
        }
    ]
}

pub fn mapping_client_fixture() -> [MappingClient;2] {
    [MappingClient::new(1, 1).unwrap(), MappingClient::new(2, 2).unwrap()]
}

pub fn mapping_client_model_fixture() -> [MappingClientModel;2] {
    [MappingClientModel::new(1, 1), MappingClientModel::new(2, 2)]
}

pub fn mapping_client_source_model_fixture() -> [MappingClientSource;2] {
    [MappingClientSource{ id_source_client: 1, id_source_contact: 1, id: Some(1) }, MappingClientSource{ id_source_client: 2, id_source_contact: 2, id: Some(2) }]
}
