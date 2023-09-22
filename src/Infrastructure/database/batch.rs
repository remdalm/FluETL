use super::connection::DbConnection;
use diesel::result::Error as DieselError;
use std::cell::RefCell;

pub(crate) struct Config {
    max_batch_size: usize,
}

impl Config {
    pub fn new(max_batch_size: usize) -> Self {
        Self { max_batch_size }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_batch_size: 100,
        }
    }
}

pub(crate) struct Batch<'a, M> {
    models: &'a [M],
    config: Config,
    cb: fn(&[M], &mut DbConnection) -> Result<(), DieselError>,
    connection: RefCell<DbConnection>,
}

impl<'a, M> Batch<'a, M> {
    pub fn new(
        models: &'a [M],
        config: Option<Config>,
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
