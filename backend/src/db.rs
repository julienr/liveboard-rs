mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use chrono::{NaiveDateTime};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "liveboard.shapes")]
    pub struct Shape {
        pub id: i32,
        pub timestamp: NaiveDateTime,
        pub shape: String,
    }
}

use std::env;
use deadpool_postgres::{Pool, Config};
use deadpool_postgres::Client;
use tokio_postgres::NoTls;
use models::Shape;
use errors::MyError;
use tokio_pg_mapper::FromTokioPostgresRow;

pub struct State {
    pub pool: Pool
}

pub fn make_state() -> State {
    let mut cfg = Config::new();
    cfg.user = Some("postgres".to_string());
    cfg.password = Some(env::var("DB_PASSWORD").unwrap());
    cfg.host = Some("localhost".to_string());
    cfg.dbname = Some("liveboard".to_string());
    let pool = cfg.create_pool(None, NoTls).unwrap();

    return State {
        pool: pool.clone()
    };
}

pub async fn get_shapes(client: &Client) -> Result<Vec<Shape>, MyError> {
    let stmt = include_str!("../sql/get_shapes.sql");
    let stmt = stmt.replace("$table_fields", &Shape::sql_table_fields());
    let stmt = stmt.replace("$table", &Shape::sql_table());
    let stmt = client.prepare(&stmt).await.unwrap();
    let r = client
        .query(
            &stmt, &[]
        )
        .await?
        .iter()
        .map(|row| Shape::from_row_ref(row))
        .collect::<Result<Vec<Shape>, tokio_pg_mapper::Error>>();
    return match r {
        Ok(v) => Ok(v),
        Err(_v) => Err(MyError::NotFound)
    };
}
