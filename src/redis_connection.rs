use std::ops::{Deref, DerefMut};
use r2d2;
use r2d2::PooledConnection;
use r2d2_redis::RedisConnectionManager;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::{Outcome, State};

type Pool = r2d2::Pool<RedisConnectionManager>;
type PooledConn = PooledConnection<RedisConnectionManager>;

pub struct Conn(pub PooledConn);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Conn {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Conn, Self::Error> {
        let pool = req.guard::<State<Pool>>();

        match pool.get() {
            Some(database) => Outcome::Success(Conn(database)),
            None => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = PooledConn;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Conn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn init_pool() -> Pool {
    

    let redis_address = env::var("REDIS_ADDRESS").expect("REDIS_ADDRESS missing");
    let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT missing");
    let redis_db = env::var("REDIS_DB").expect("REDIS_DB missing");
    //let redis_password = env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD missing");
    let manager = RedisConnectionManager::new(format!("redis://{}:{}/{}", redis_address, redis_port, redis_db)).expect("connection manager");
    // Otherwise, with password:
    //let manager = RedisConnectionManager::new(format!("redis://user:{}@{}:{}/{}", redis_password redis_address, redis_port, redis_db)).expect("connection manager");
    match r2d2::Pool::builder().max_size(15).build(manager) {
        Ok(pool) => pool,
        Err(e) => panic!("Error: failed to create database pool {}", e),
    }
}