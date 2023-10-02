use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::{
    domain::{
        delivery_slip::{DeliverySlip, DeliverySlipDomainFactory},
        dto::date_dto::DateDTO,
        mapping_client::MappingClient,
        order::{Order, OrderDomainFactory, Origin},
        order_line::{OrderLine, OrderLineDomainFactory},
    },
    infrastructure::{
        csv_reader::{
            delivery_slip::CsvDeliverySlipDTO, order::CsvOrderDTO, order_line::CsvOrderLineDTO,
        },
        database::models::{
            delivery_slip::DeliverySlipModel,
            mapping_client::{MappingClientModel, MappingClientSource},
            order::OrderModel,
            order_line::OrderLineModel,
        },
    },
};

// ORDER FIXTURES
pub const ORDER_FLAWLESS_CSV: &str= "c_order_id;c_bpartner_id;client_name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Client 1;2023-08-01;Ref1;PoRef1;Web;30;done;done\n2;2;Client 2;2023-08-02;Ref2;PoRef2;EDI;20;failed;done\n";
pub const ORDER_WITH_EMPTY_FIELD_CSV: &str = "c_order_id;c_bpartner_id;client_name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Client 1;2023-08-01;Ref1;PoRef1;Web;30;;\n3;1;;2023-08-03;Ref3;PoRef3;Origin3;0;;done\n";
pub const ORDER_WITH_MISSING_DATA_CSV: &str = "c_order_id;c_bpartner_id;client_name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Client 1;2023-08-01;Ref1;PoRef1;Web;30\n2;2;Client 2;2023-08-02;Ref2;PoRef2;EDI;20\n";

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
        },
    ]
}

pub fn order_model_fixtures() -> [OrderModel; 3] {
    [
        OrderModel {
            id_order: 1,
            id_client: 1,
            client_name: Some("Client 1".to_string()),
            order_ref: "Ref1".to_string(),
            po_ref: Some("PoRef1".to_string()),
            origin: Some("Web".to_string()),
            completion: Some(30),
            date: NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            ),
            order_status: Some("done".to_string()),
            delivery_status: Some("done".to_string()),
        },
        OrderModel {
            id_order: 2,
            id_client: 2,
            client_name: Some("Client 2".to_string()),
            order_ref: "Ref2".to_string(),
            po_ref: Some("PoRef2".to_string()),
            origin: Some("EDI".to_string()),
            completion: Some(20),
            date: NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2023, 8, 2).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            ),
            order_status: Some("failed".to_string()),
            delivery_status: Some("done".to_string()),
        },
        OrderModel {
            id_order: 3,
            id_client: 1,
            client_name: None,
            order_ref: "Ref3".to_string(),
            po_ref: None,
            origin: None,
            completion: None,
            date: NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2023, 8, 3).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            ),
            order_status: None,
            delivery_status: Some("done".to_string()),
        },
    ]
}

// MAPPING CLIENT FIXTURES

pub fn mapping_client_fixture() -> [MappingClient; 2] {
    [
        MappingClient::new(1, 1).unwrap(),
        MappingClient::new(2, 2).unwrap(),
    ]
}

pub fn mapping_client_model_fixture() -> [MappingClientModel; 2] {
    [MappingClientModel::new(1, 1), MappingClientModel::new(2, 2)]
}

pub fn mapping_client_source_model_fixture() -> [MappingClientSource; 2] {
    [
        MappingClientSource {
            id_source_client: 1,
            id_source_contact: 1,
            id: Some(1),
        },
        MappingClientSource {
            id_source_client: 2,
            id_source_contact: 2,
            id: Some(2),
        },
    ]
}

// ORDER LINE FIXTURES

pub fn csv_order_line_dto_fixtures() -> [CsvOrderLineDTO; 3] {
    [
        CsvOrderLineDTO {
            c_orderline_id: 1.to_string(),
            c_order_id: 1.to_string(),
            item_ref: "ItemRef1".to_string(),
            item_name: "ItemName1".to_string(),
            qty_ordered: "10".to_string(),
            qty_reserved: "5".to_string(),
            qty_delivered: "5".to_string(),
            due_date: "2023-08-01".to_string(),
        },
        CsvOrderLineDTO {
            c_orderline_id: 2.to_string(),
            c_order_id: 1.to_string(),
            item_ref: "ItemRef2".to_string(),
            item_name: "ItemName2".to_string(),
            qty_ordered: "20".to_string(),
            qty_reserved: "10".to_string(),
            qty_delivered: "10".to_string(),
            due_date: "2023-08-02".to_string(),
        },
        CsvOrderLineDTO {
            c_orderline_id: 3.to_string(),
            c_order_id: 2.to_string(),
            item_ref: "ItemRef3".to_string(),
            item_name: String::new(),
            qty_ordered: "30".to_string(),
            qty_reserved: "15".to_string(),
            qty_delivered: "15".to_string(),
            due_date: String::new(),
        },
    ]
}

pub fn order_line_model_fixtures() -> [OrderLineModel; 3] {
    [
        OrderLineModel {
            id_order_line: 1,
            id_order: 1,
            product_ref: "ItemRef1".to_string(),
            product_name: Some("ItemName1".to_string()),
            qty_ordered: 10,
            qty_reserved: 5,
            qty_delivered: 5,
            due_date: Some(NaiveDate::from_ymd_opt(2023, 8, 1).unwrap()),
        },
        OrderLineModel {
            id_order_line: 2,
            id_order: 1,
            product_ref: "ItemRef2".to_string(),
            product_name: Some("ItemName2".to_string()),
            qty_ordered: 20,
            qty_reserved: 10,
            qty_delivered: 10,
            due_date: Some(NaiveDate::from_ymd_opt(2023, 8, 2).unwrap()),
        },
        OrderLineModel {
            id_order_line: 3,
            id_order: 2,
            product_ref: "ItemRef3".to_string(),
            product_name: None,
            qty_ordered: 30,
            qty_reserved: 15,
            qty_delivered: 15,
            due_date: None,
        },
    ]
}

// DELIVERY SLIP FIXTURES

pub fn csv_delivery_slip_dto_fixtures() -> [CsvDeliverySlipDTO; 3] {
    [
        CsvDeliverySlipDTO {
            m_inout_id: 1.to_string(),
            c_bpartner_id: 1.to_string(),
            documentno: "Doc1".to_string(),
            shipping_date: "2023-08-01".to_string(),
            po_ref: "PoRef1".to_string(),
            carrier_name: "Carrier1".to_string(),
            trackingno: "TrackingNo1".to_string(),
            status: "1".to_string(),
            tracking_link: "https://tracking1.com/123".to_string(),
        },
        CsvDeliverySlipDTO {
            m_inout_id: 2.to_string(),
            c_bpartner_id: 2.to_string(),
            documentno: "Doc2".to_string(),
            shipping_date: "2023-08-02".to_string(),
            po_ref: "PoRef2".to_string(),
            carrier_name: "Carrier2".to_string(),
            trackingno: "TrackingNo2".to_string(),
            status: "2".to_string(),
            tracking_link: "http:://tracking2.com".to_string(),
        },
        CsvDeliverySlipDTO {
            m_inout_id: 3.to_string(),
            c_bpartner_id: 1.to_string(),
            documentno: "Doc3".to_string(),
            shipping_date: String::new(),
            po_ref: String::new(),
            carrier_name: String::new(),
            trackingno: String::new(),
            status: String::new(),
            tracking_link: String::new(),
        },
    ]
}

pub fn delivery_slip_fixtures() -> [DeliverySlip; 3] {
    [
        DeliverySlipDomainFactory {
            delivery_slip_id: 1,
            c_bpartner_id: 1,
            reference: "Doc1".to_string(),
            shipping_date: Some(NaiveDate::from_ymd_opt(2023, 8, 1).unwrap()),
            po_ref: Some("PoRef1".to_string()),
            carrier_name: Some("Carrier1".to_string()),
            trackingno: Some("TrackingNo1".to_string()),
            status: Some("1".to_string()),
            tracking_link: Some("https://tracking1.com/123".to_string()),
        }
        .make()
        .unwrap(),
        DeliverySlipDomainFactory {
            delivery_slip_id: 2,
            c_bpartner_id: 2,
            reference: "Doc2".to_string(),
            shipping_date: Some(NaiveDate::from_ymd_opt(2023, 8, 2).unwrap()),
            po_ref: Some("PoRef2".to_string()),
            carrier_name: Some("Carrier2".to_string()),
            trackingno: Some("TrackingNo2".to_string()),
            status: Some("2".to_string()),
            tracking_link: None,
        }
        .make()
        .unwrap(),
        DeliverySlipDomainFactory {
            delivery_slip_id: 3,
            c_bpartner_id: 1,
            reference: "Doc3".to_string(),
            shipping_date: None,
            po_ref: None,
            carrier_name: None,
            trackingno: None,
            status: None,
            tracking_link: None,
        }
        .make()
        .unwrap(),
    ]
}

pub fn delivery_slip_model_fixtures() -> [DeliverySlipModel; 3] {
    [
        DeliverySlipModel {
            id_delivery_slip: 1,
            id_client: 1,
            reference: "Doc1".to_string(),
            shipping_date: Some(NaiveDate::from_ymd_opt(2023, 8, 1).unwrap()),
            po_ref: Some("PoRef1".to_string()),
            carrier_name: Some("Carrier1".to_string()),
            status: Some("1".to_string()),
            tracking_number: Some("TrackingNo1".to_string()),
            tracking_link: Some("https://tracking1.com/123".to_string()),
        },
        DeliverySlipModel {
            id_delivery_slip: 2,
            id_client: 2,
            reference: "Doc2".to_string(),
            shipping_date: Some(NaiveDate::from_ymd_opt(2023, 8, 2).unwrap()),
            po_ref: Some("PoRef2".to_string()),
            carrier_name: Some("Carrier2".to_string()),
            status: Some("2".to_string()),
            tracking_number: Some("TrackingNo2".to_string()),
            tracking_link: None,
        },
        DeliverySlipModel {
            id_delivery_slip: 3,
            id_client: 1,
            reference: "Doc3".to_string(),
            shipping_date: None,
            po_ref: None,
            carrier_name: None,
            status: None,
            tracking_number: None,
            tracking_link: None,
        },
    ]
}
