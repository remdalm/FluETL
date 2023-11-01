use serde::Deserialize;

use super::{
    csv_reader::{make_csv_file_reader, CsvDTO, CsvType},
    database::{connection::HasConnection, models::CanSelectAllModel},
    InfrastructureError,
};

pub(crate) trait CanReadCSVDataSource<T>
where
    T: CsvDTO + for<'a> Deserialize<'a>,
{
    fn read(&self, csv_type: CsvType) -> Result<Vec<T>, InfrastructureError> {
        let csv_reader = make_csv_file_reader(csv_type, b';')?;

        let csv_data: Vec<T> = csv_reader.read().map_err(InfrastructureError::CsvError)?;
        Ok(csv_data)
    }

    fn find_all(&self) -> Result<Vec<T>, InfrastructureError>;
}

pub(crate) trait CanSelectAllDataSource {
    type DbConnection: HasConnection;
    type Model: CanSelectAllModel;

    fn find_all(&self) -> Result<Vec<Self::Model>, InfrastructureError> {
        Self::Model::select_all(&mut Self::DbConnection::get_pooled_connection())
            .map_err(InfrastructureError::DatabaseError)
    }
}
