use anyhow::Result;
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};

pub type PgPoolSquad = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url : &String) -> Result<PgPoolSquad>{
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");
        Ok(pool)
}