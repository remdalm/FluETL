use super::connection::{DbConnection, HasConnection};
use diesel::result::Error as DieselError;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BatchConfig {
    max_batch_size: usize,
}

impl BatchConfig {
    pub fn new(max_batch_size: usize) -> Self {
        Self { max_batch_size }
    }
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 100,
        }
    }
}

pub(crate) struct Batch<'a, M> {
    models: &'a [M],
    config: BatchConfig,
    cb: fn(&[M], &mut DbConnection) -> Result<(), DieselError>,
    connection: RefCell<DbConnection>,
}

impl<'a, M> Batch<'a, M> {
    pub fn new(
        models: &'a [M],
        config: Option<BatchConfig>,
        cb: fn(&[M], &mut DbConnection) -> Result<(), DieselError>,
        connection: DbConnection,
    ) -> Self {
        Self {
            models,
            config: config.unwrap_or_default(),
            cb,
            connection: RefCell::new(connection),
        }
    }

    pub fn run(&self) -> Option<Vec<DieselError>> {
        let mut errors: Vec<DieselError> = Vec::new();
        let iter = self.models.chunks(self.config.max_batch_size);

        for chunk in iter {
            let result = (self.cb)(chunk, &mut self.connection.borrow_mut());
            if let Err(e) = result {
                errors.push(e);
            }
        }

        Option::from(errors).filter(|e| !e.is_empty())
    }
}

pub(crate) trait CanMakeBatchTransaction<M> {
    type DbConnection: HasConnection;
    fn make_batch<'a>(
        &self,
        models: &'a [M],
        config: Option<BatchConfig>,
        f: fn(models: &[M], connection: &mut DbConnection) -> Result<(), DieselError>,
    ) -> Batch<'a, M> {
        Batch::new(
            models,
            config,
            f,
            Self::DbConnection::get_pooled_connection(),
        )
    }
}
