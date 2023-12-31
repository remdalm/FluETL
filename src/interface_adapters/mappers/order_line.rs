use std::env;

use crate::{
    domain::{
        order_line::{OrderLine, OrderLineLocalizedItemFactory, OrderLinePrimaryFields},
        vo::{locale::Locale, Translation},
    },
    infrastructure::{
        csv_reader::order_line::{CsvOrderLineDTO, CsvOrderLineLocalizedItemDTO},
        database::models::order_line::{OrderLineLangModel, OrderLineModel},
        InfrastructureError,
    },
};

use super::{convert_string_to_option_date, parse_string_to_u32, MappingError};

impl TryFrom<CsvOrderLineDTO> for OrderLinePrimaryFields {
    type Error = MappingError;
    fn try_from(dto: CsvOrderLineDTO) -> Result<OrderLinePrimaryFields, MappingError> {
        let date_format = env::var("CSV_DATE_FORMAT")
            .map_err(|e| MappingError::Infrastructure(InfrastructureError::EnvVarError(e)))?;

        Ok(OrderLinePrimaryFields {
            order_id: parse_string_to_u32("order_id", &dto.c_order_id)?,
            orderline_id: parse_string_to_u32("orderline_id", &dto.c_orderline_id)?,
            item_ref: dto.item_ref,
            qty_ordered: parse_string_to_u32("qty_ordered", &dto.qty_ordered)?,
            qty_reserved: parse_string_to_u32("qty_reserved", &dto.qty_reserved)?,
            qty_delivered: parse_string_to_u32("qty_delivered", &dto.qty_delivered)?,
            due_date: convert_string_to_option_date(dto.due_date, &date_format).transpose()?,
        })
    }
}

impl TryFrom<CsvOrderLineLocalizedItemDTO> for OrderLineLocalizedItemFactory {
    type Error = MappingError;
    fn try_from(
        dto: CsvOrderLineLocalizedItemDTO,
    ) -> Result<OrderLineLocalizedItemFactory, MappingError> {
        Ok(OrderLineLocalizedItemFactory {
            orderline_id: parse_string_to_u32("orderline_id", &dto.c_orderline_id)?,
            locale: Locale::try_from(dto.ad_language.as_str())?,
            name: Translation::new(dto.item_name)?,
        })
    }
}

impl From<OrderLine> for (OrderLineModel, Vec<OrderLineLangModel>) {
    fn from(order_line: OrderLine) -> Self {
        let order_line_items: Vec<OrderLineLangModel> = order_line
            .item_names()
            .iter()
            .map(|item_name| OrderLineLangModel {
                id_order_line: order_line.orderline_id(),
                id_lang: item_name.language().id(),
                product_name: item_name.name().as_str().to_string(),
            })
            .collect();
        (
            OrderLineModel {
                id_order: order_line.order().order_id(),
                id_order_line: order_line.orderline_id(),
                product_ref: order_line.item_ref().to_string(),
                qty_ordered: order_line.qty_ordered(),
                qty_reserved: order_line.qty_reserved(),
                qty_delivered: order_line.qty_delivered(),
                due_date: order_line.due_date(),
            },
            order_line_items,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        domain::{
            order::Order,
            order_line::{tests::order_line_fixtures, OrderLineDomainFactory},
            vo::localized_item::{tests::localized_item_fixtures, LocalizedItem},
        },
        infrastructure::{
            csv_reader::order_line::tests::csv_order_line_dto_fixtures,
            database::models::{
                order::{bench::order_model_fixtures, OrderModel},
                order_line::tests::{order_line_lang_model_fixtures, order_line_model_fixtures},
            },
        },
        interface_adapters::mappers::{convert_domain_entity_to_model, CsvEntityParser},
        tests::load_unit_test_env,
    };

    use super::*;

    struct CsvParser;
    impl CsvEntityParser<CsvOrderLineDTO, OrderLine> for CsvParser {
        fn transform_csv_row_to_entity(
            &self,
            csv: CsvOrderLineDTO,
        ) -> Result<OrderLine, MappingError> {
            let raw_fields: Result<OrderLinePrimaryFields, MappingError> = csv.try_into();
            raw_fields.and_then(|fields| {
                let order_model = mock_fetching_order(&fields.order_id);
                let order: Order = order_model.try_into()?;
                let mut factory = OrderLineDomainFactory::new_from_order(order, &fields);
                order_line_items_hashmap_fixture()
                    .contains_key(&fields.orderline_id)
                    .then(|| {
                        factory.item_names = order_line_items_hashmap_fixture()
                            .get(&fields.orderline_id)
                            .unwrap()
                            .to_owned();
                    });
                factory.make().map_err(MappingError::Domain)
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

    fn order_line_items_hashmap_fixture() -> HashMap<u32, Vec<LocalizedItem>> {
        let mut order_line_items = HashMap::new();
        order_line_items.insert(
            1,
            vec![
                localized_item_fixtures()[0].clone(),
                localized_item_fixtures()[1].clone(),
            ],
        );
        order_line_items.insert(2, vec![localized_item_fixtures()[2].clone()]);
        order_line_items.insert(3, Vec::new());
        order_line_items
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

        assert!(result.is_err_and(|e| matches!(e, MappingError::Parsing(_))));
    }

    #[test]
    fn test_convert_order_lines_to_models() {
        let models_fixtures = order_line_model_fixtures();
        let model_lang_fixtures = order_line_lang_model_fixtures();
        let order_line_fixtures = order_line_fixtures();

        let results: Vec<(OrderLineModel, Vec<OrderLineLangModel>)> =
            convert_domain_entity_to_model(order_line_fixtures.to_vec());

        assert_eq!(&results[0].0, &models_fixtures[0]);
        assert_eq!(&results[1].0, &models_fixtures[1]);
        assert_eq!(&results[0].1, &model_lang_fixtures[0]);
        assert_eq!(&results[1].1, &model_lang_fixtures[1]);
    }
}
