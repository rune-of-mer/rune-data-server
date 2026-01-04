use diesel::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub type DBPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn create_db_pool(url: &str) -> anyhow::Result<DBPool> {
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = Pool::builder().max_size(10).build(manager)?;
    Ok(pool)
}
