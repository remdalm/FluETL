use super::connection::DbConnection;
use core::ops::Deref;
use diesel::associations::HasTable;
use diesel::expression::ValidGrouping;
use diesel::insertable::CanInsertInSingleQuery;
use diesel::internal::derives::multiconnection::DieselReserveSpecialization;
use diesel::mysql::Mysql;
use diesel::query_builder::{AsQuery, InsertStatement, QueryFragment, QueryId};
use diesel::query_dsl::methods::ExecuteDsl;
use diesel::result::Error as DieselError;
use diesel::{Connection, Insertable, RunQueryDsl, Table};

mod mapping_client;
pub mod order;

pub trait ModelOps<T, Conn>: Insertable<T> + Sized
where
    T: Table + QueryFragment<Conn::Backend> + QueryId + HasTable + 'static,
    T::FromClause: QueryFragment<Conn::Backend>,
    // InsertStatement<T, <Self as Insertable<T>>::Values>:
    //     QueryFragment<Conn::Backend> + QueryId + ExecuteDsl<Conn>,
    // Self: 'a,
    Conn: Connection,
    Conn::Backend: DieselReserveSpecialization,
{
    // type TargetTable: Table;
    // Define a default implementation for insert
    fn insert(&self, connection: &mut Conn) -> Result<(), DieselError>
    where
        // Self: AsQuery<SqlType = T::SqlType>,
        for<'b> &'b Self: diesel::Insertable<T>,
        for<'b> <&'b Self as Insertable<T>>::Values: QueryFragment<<Conn>::Backend>
            + CanInsertInSingleQuery<<Conn>::Backend>
            + QueryId
            + ExecuteDsl<Conn, <Conn>::Backend>,
    {
        let target = self.target_client_table();
        let values = diesel::insert_into(target).values(self);
        values.execute(connection).map(|_| ()).map_err(|e| e.into())
    }

    fn target_client_table(&self) -> T;
}

// type TargetTable: diesel::prelude::Table;

// // Define a default implementation for insert
// fn insert(
//     &self,
//     connection: &mut DbConnection,
//     target: Self::TargetTable,
// ) -> Result<(), DieselError>;

// fn update(
//     &self,
//     connection: &mut DbConnection,
//     table: Self::TargetTable,
//     // new: &Self,
// ) -> Result<(), DieselError>;

// fn insert<T: diesel::prelude::Table>(
//     &self,
//     connection: &mut DbConnection,
//     target: T,
// ) -> Result<(), DieselError>;

// Define an associated type for the target table
// type TargetTable: diesel::prelude::Table;
