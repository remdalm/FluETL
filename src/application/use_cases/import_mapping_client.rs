use crate::infrastructure::csv_reader::{make_csv_file_reader, CsvType};

use super::UseCase;

pub struct ImportMappingClient;

// impl UseCase for ImportMappingClient {
//     fn execute(&self) -> Result<(), super::UseCaseError> {
//         let csv_reader = make_csv_file_reader(CsvType::MappingClient, b';')?;
//         let csv_data = csv_reader.read()?;

//     }
//     // pub fn execute(
//     //     csv_data: Vec<EmployeeCsvRow>,
//     //     connection: &mut DbConnection,
//     // ) -> Result<(), DieselError> {
//     //     let employees: Vec<Employee> = csv_data.into_iter().map(Employee::from_csv_row).collect();
//     //     Employee::insert_many(connection, &employees)?;
//     //     Ok(())
//     // }
// }
