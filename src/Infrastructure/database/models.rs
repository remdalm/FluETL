use diesel::associations::HasTable;
use diesel::insertable::CanInsertInSingleQuery;
use diesel::internal::derives::multiconnection::DieselReserveSpecialization;
use diesel::query_builder::{AsQuery, IntoUpdateTarget, QueryFragment, QueryId, UpdateStatement};
use diesel::query_dsl::methods::ExecuteDsl;
use diesel::result::Error as DieselError;
use diesel::{AsChangeset, Connection, Insertable, RunQueryDsl, Table};

use crate::domain::DomainEntity;

pub(crate) mod mapping_client;
pub(crate) mod order;

pub trait Model {}
pub trait SingleRowInsertable<T, Conn>
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
        diesel::insert_into(self.target_client_table())
            .values(self)
            .execute(connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    fn target_client_table(&self) -> T;
}

pub trait SingleRowUpdatable<T, Conn>
where
    T: Table + HasTable + IntoUpdateTarget + 'static,
    T::FromClause: QueryFragment<Conn::Backend>,
    Conn: Connection,
{
    fn update(&self, connection: &mut Conn) -> Result<(), DieselError>
    where
        for<'b> &'b Self: AsChangeset<Target = <T as HasTable>::Table>,
        for<'b> UpdateStatement<
            <T as HasTable>::Table,
            <T as IntoUpdateTarget>::WhereClause,
            <&'b Self as AsChangeset>::Changeset,
        >: AsQuery + QueryFragment<Conn::Backend>,
    {
        diesel::update(self.target_client_table())
            .set(self)
            .execute(connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    fn target_client_table(&self) -> T;
}
