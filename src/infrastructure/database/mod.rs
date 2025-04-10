pub mod repositories;
pub mod schema;

use anyhow::Result;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

pub type SqlitePoolSquad = Pool<ConnectionManager<SqliteConnection>>;

pub fn conn(database_url: &str) -> Result<SqlitePoolSquad> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;
    Ok(pool)
}
