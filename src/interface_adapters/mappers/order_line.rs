use std::env;

use crate::{
    domain::{
        convert_string_to_option_string,
        order_line::{OrderLine, OrderLinePrimaryFields},
    },
    infrastructure::{
        csv_reader::CsvOrderLineDTO, database::models::order_line::OrderLineModel,
        InfrastructureError,
    },
};
use chrono::NaiveDate;

use super::MappingError;

impl TryFrom<CsvOrderLineDTO> for OrderLinePrimaryFields {
    type Error = MappingError;
    fn try_from(dto: CsvOrderLineDTO) -> Result<OrderLinePrimaryFields, MappingError> {
        let date_format = env::var("CSV_DATE_FORMAT")
            .map_err(|e| MappingError::InfrastructureError(InfrastructureError::EnvVarError(e)))?;

        Ok(OrderLinePrimaryFields {
            order_id: dto
                .c_order_id
                .parse::<u32>()
                .map_err(|e| MappingError::ParsingError(e.to_string()))?,
            orderline_id: dto
                .c_orderline_id
                .parse::<u32>()
                .map_err(|e| MappingError::ParsingError(e.to_string()))?,
            item_ref: dto.item_ref,
            item_name: convert_string_to_option_string(dto.item_name),
            qty_ordered: dto
                .qty_ordered
                .parse::<u32>()
                .map_err(|e| MappingError::ParsingError(e.to_string()))?,
            qty_reserved: dto
                .qty_reserved
                .parse::<u32>()
                .map_err(|e| MappingError::ParsingError(e.to_string()))?,
            qty_delivered: dto
                .qty_delivered
                .parse::<u32>()
                .map_err(|e| MappingError::ParsingError(e.to_string()))?,
            due_date: NaiveDate::parse_from_str(&dto.due_date.as_str(), date_format.as_str())
                .map_err(|err| {
                    MappingError::ParsingError(
                        err.to_string() + format!(": date => {}", dto.due_date).as_str(),
                    )
                })?,
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
    fn test_convert_order_lines_to_models() {
        let models_fixtures = order_line_model_fixtures();
        let order_line_fixtures = order_line_fixtures();

        let results: Vec<OrderLineModel> =
            convert_domain_entity_to_model(order_line_fixtures.to_vec());

        assert_eq!(&results[0], &models_fixtures[0]);
        assert_eq!(&results[1], &models_fixtures[1]);
    }
}
