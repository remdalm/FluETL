use std::env;

use crate::{
    domain::order_line::{OrderLine, OrderLinePrimaryFields},
    infrastructure::{
        csv_reader::CsvOrderLineDTO, database::models::order_line::OrderLineModel,
        InfrastructureError,
    },
};
use chrono::NaiveDate;

use super::{convert_string_to_option_string, parse_string_to_u32, MappingError};

impl TryFrom<CsvOrderLineDTO> for OrderLinePrimaryFields {
    type Error = MappingError;
    fn try_from(dto: CsvOrderLineDTO) -> Result<OrderLinePrimaryFields, MappingError> {
        let date_format = env::var("CSV_DATE_FORMAT")
            .map_err(|e| MappingError::InfrastructureError(InfrastructureError::EnvVarError(e)))?;

        let due_date = {
            let s_date = convert_string_to_option_string(dto.due_date);
            if s_date.is_some() {
                let date = NaiveDate::parse_from_str(
                    s_date.as_ref().unwrap().as_str(),
                    date_format.as_str(),
                )
                .map_err(|err| {
                    MappingError::ParsingError(
                        err.to_string() + format!(": date => {}", s_date.unwrap()).as_str(),
                    )
                })?;
                Some(date)
            } else {
                None
            }
        };

        Ok(OrderLinePrimaryFields {
            order_id: parse_string_to_u32("order_id", &dto.c_order_id)?,
            orderline_id: parse_string_to_u32("orderline_id", &dto.c_orderline_id)?,
            item_ref: dto.item_ref,
            item_name: convert_string_to_option_string(dto.item_name),
            qty_ordered: parse_string_to_u32("qty_ordered", &dto.qty_ordered)?,
            qty_reserved: parse_string_to_u32("qty_reserved", &dto.qty_reserved)?,
            qty_delivered: parse_string_to_u32("qty_delivered", &dto.qty_delivered)?,
            due_date: due_date,
        })
    }
}

impl From<OrderLine> for OrderLineModel {
    fn from(order_line: OrderLine) -> Self {
        Self {
            id_order: order_line.order().c_order_id(),
            id_order_line: order_line.orderline_id(),
            product_ref: order_line.item_ref().to_string(),
            product_name: order_line.item_name().map(|s| s.to_string()),
            qty_ordered: order_line.qty_ordered(),
            qty_reserved: order_line.qty_reserved(),
            qty_delivered: order_line.qty_delivered(),
            due_date: order_line.due_date(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::{order::Order, order_line::OrderLineDomainFactory},
        fixtures::{
            csv_order_line_dto_fixtures, order_line_fixtures, order_line_model_fixtures,
            order_model_fixtures,
        },
        infrastructure::database::models::order::OrderModel,
        interface_adapters::mappers::{convert_domain_entity_to_model, CSVToEntityParser},
        tests::load_unit_test_env,
    };

    use super::*;

    struct CsvParser;
    impl CSVToEntityParser<CsvOrderLineDTO, OrderLine> for CsvParser {
        fn transform_csv(&self, csv: CsvOrderLineDTO) -> Result<OrderLine, MappingError> {
            let raw_fields: Result<OrderLinePrimaryFields, MappingError> = csv.try_into();
            raw_fields.and_then(|fields| {
                let order_model = mock_fetching_order(&fields.order_id);
                let order: Order = order_model.try_into()?;
                OrderLineDomainFactory::new_from_order(order, fields)
                    .make()
                    .map_err(|e| MappingError::DomainError(e))
            })
        }
    }

    fn mock_fetching_order(order_id: &u32) -> OrderModel {
        let order_model_fixtures = order_model_fixtures();
        let order_model = order_model_fixtures
            .iter()
            .find(|om| om.id_order == *order_id)
            .unwrap();
        order_model.clone()
    }

    #[test]
    fn test_convert_csv_dtos_to_order_lines() {
        load_unit_test_env();

        let dto_fixtures = csv_order_line_dto_fixtures();

        let results = CsvParser.parse_all(dto_fixtures.to_vec());

        let order_line_fixtures = order_line_fixtures();

        for (i, result) in results.iter().enumerate() {
            assert!(
                result.is_ok(),
                "Expected successful conversion for index {}",
                i
            );
            assert_eq!(result.as_ref().unwrap(), &order_line_fixtures[i]);
        }
    }

    #[test]
    fn test_convert_csv_dtos_to_order_lines_with_invalid_due_date() {
        load_unit_test_env();

        let dto_fixture = &mut csv_order_line_dto_fixtures()[0];
        dto_fixture.due_date = "2023-13-01".to_string();

        let result = CsvParser.parse(dto_fixture.to_owned());

        assert!(result.is_err_and(|e| matches!(e, MappingError::ParsingError(_))));
    }

    #[test]
    fn test_convert_order_lines_to_models() {
        let models_fixtures = order_line_model_fixtures();
        let order_line_fixtures = order_line_fixtures();

        let results: Vec<OrderLineModel> =
            convert_domain_entity_to_model(order_line_fixtures.to_vec());

        assert_eq!(&results[0], &models_fixtures[0]);
        assert_eq!(&results[1], &models_fixtures[1]);
    }
}
