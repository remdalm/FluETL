use super::connection::DbConnection;
use core::ops::Deref;
use diesel::associations::HasTable;
use diesel::expression::ValidGrouping;
use diesel::insertable::CanInsertInSingleQuery;
use diesel::internal::derives::multiconnection::DieselReserveSpecialization;
use diesel::mysql::Mysql;
use diesel::query_builder::{
    AsQuery, InsertStatement, IntoUpdateTarget, QueryFragment, QueryId, UpdateStatement,
};
use diesel::query_dsl::methods::{ExecuteDsl, FilterDsl, FindDsl};
use diesel::result::Error as DieselError;
use diesel::{
    AsChangeset, Connection, EqAll, Identifiable, Insertable, QuerySource, RunQueryDsl, Table,
};

use diesel::QueryDsl;

mod mapping_client;
pub mod order;

pub trait ModelInsertOps<T, Conn>: Insertable<T> + Sized
where
    T: Table + QueryFragment<Conn::Backend> + QueryId + HasTable + 'static,
    T::FromClause: QueryFragment<Conn::Backend>,
    Conn: Connection,
    Conn::Backend: DieselReserveSpecialization,
{
    fn insert(&self, connection: &mut Conn) -> Result<(), DieselError>
    where
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

pub trait ModelUpdateOps<T, Conn, PK>: AsChangeset<Target = T> + Sized
where
    T: Table + FindDsl<PK> + QueryFragment<Conn::Backend> + QueryId + HasTable + 'static,
    T::FromClause: QueryFragment<Conn::Backend>,
    Conn: Connection,
    Conn::Backend: DieselReserveSpecialization,
{
    fn update(&self, connection: &mut Conn) -> Result<(), DieselError>
    where
        for<'b> &'b Self: AsChangeset<Target = <<T as FindDsl<PK>>::Output as HasTable>::Table>,
        for<'b> <&'b Self as AsChangeset>::Changeset: QueryFragment<Conn::Backend>,
        <T as Table>::PrimaryKey: EqAll<T>,
        <T as FindDsl<T>>::Output: HasTable + IntoUpdateTarget,
        <T as AsQuery>::Query: FilterDsl<<<T as Table>::PrimaryKey as EqAll<T>>::Output>,
        <T as AsQuery>::Query: FilterDsl<T>,
        <<T as AsQuery>::Query as FilterDsl<T>>::Output: HasTable + IntoUpdateTarget,
        <T as FindDsl<PK>>::Output: HasTable + IntoUpdateTarget,
        <<T as FindDsl<PK>>::Output as IntoUpdateTarget>::WhereClause:
            QueryFragment<<Conn as Connection>::Backend>,
        for<'b> UpdateStatement<
            <<T as FindDsl<PK>>::Output as HasTable>::Table,
            <<T as FindDsl<PK>>::Output as IntoUpdateTarget>::WhereClause,
            <&'b Self as AsChangeset>::Changeset,
        >: AsQuery,
        <<<T as FindDsl<PK>>::Output as HasTable>::Table as QuerySource>::FromClause:
            QueryFragment<Conn::Backend>,
    {
        let update_statement = diesel::update(self.target_client_table().find(self.primary_key()));

        let set = update_statement.set(self);
        set.execute(connection).map(|_| ()).map_err(|e| e.into())
    }

    fn target_client_table(&self) -> T;
    fn primary_key(&self) -> PK;
}

// fn primary_key(&self) -> PK;

// <T as AsQuery>::Query: FilterDsl<T>,
// <T as FindDsl<PK>>::Output: HasTable + IntoUpdateTarget,
// <<T as AsQuery>::Query as FilterDsl<T>>::Output: HasTable + IntoUpdateTarget,
// <<<T as FindDsl<PK>>::Output as HasTable>::Table as AsQuery>::Query: FilterDsl<PK>,
// <<T as FindDsl<PK>>::Output as HasTable>::Table: FindDsl<PK>,
// <<<T as AsQuery>::Query as FilterDsl<T>>::Output as HasTable>::Table: FindDsl<PK>,
// <<<<T as AsQuery>::Query as FilterDsl<T>>::Output as HasTable>::Table as FindDsl<PK>>::Output:
//     Identifiable,
// UpdateStatement<
//     <<T as FindDsl<PK>>::Output as HasTable>::Table,
//     <<T as FindDsl<PK>>::Output as IntoUpdateTarget>::WhereClause,
// >: AsQuery,
// for<'b> &'b Self: AsChangeset,
// for<'b> <&'b Self as diesel::AsChangeset>::Target: Table,
// for<'b> <&'b Self as diesel::AsChangeset>::Changeset: AsQuery,
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
