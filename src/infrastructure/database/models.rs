use diesel::associations::HasTable;
use diesel::insertable::CanInsertInSingleQuery;
use diesel::internal::derives::multiconnection::DieselReserveSpecialization;
use diesel::query_builder::{AsQuery, IntoUpdateTarget, QueryFragment, QueryId, UpdateStatement};
use diesel::query_dsl::methods::ExecuteDsl;
use diesel::result::Error as DieselError;
use diesel::{AsChangeset, Connection, Insertable, RunQueryDsl, Table};

use super::connection::DbConnection;

pub(crate) mod delivery_slip;
pub(crate) mod invoice;
pub(crate) mod language;
pub(crate) mod mapping_client;
pub(crate) mod order;
pub(crate) mod order_line;
pub(crate) mod product;
pub(crate) mod product_substitute;

pub use order::OrderModel;

pub trait Model {}

pub trait CanUpsertModel: Model {
    fn upsert(&self, connection: &mut DbConnection) -> Result<(), DieselError>;
}

pub trait CanSelectAllModel: Model
where
    Self: Sized,
{
    fn select_all(connection: &mut DbConnection) -> Result<Vec<Self>, DieselError>;
}

pub trait CanDeleteModel: Model
where
    Self: Sized,
{
    fn delete_list(
        connection: &mut DbConnection,
        associations_to_delete: &[Self],
    ) -> Option<Vec<DieselError>>;
}

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
    }

    fn target_client_table(&self) -> T;
}

macro_rules! upsert {
    ($table:path, $model:expr, $connection:expr) => {
        diesel::replace_into($table)
            .values($model)
            .execute($connection)
            .map(|_| ())
    };
}
use upsert;
