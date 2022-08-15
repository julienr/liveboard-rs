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
        InternalError(String),
    }
    impl std::error::Error for MyError {}
    impl From<std::fmt::Error> for MyError {
        fn from(e: std::fmt::Error) -> MyError {
            MyError::InternalError(e.to_string())
        }
    }

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

pub mod models {
    use chrono::{NaiveDateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_json;
    use shared::datatypes as data;
    use tokio_pg_mapper_derive::PostgresMapper;

    pub trait Insertable {
        // Transform to (column name, value) tuples for INSERT statement
        fn to_insert_tuples(&self) -> Vec<[String; 2]>;
    }

    #[derive(Deserialize, PostgresMapper, Serialize, Debug)]
    #[pg_mapper(table = "shapes")]
    pub struct Shape {
        pub id: i32,
        pub board_id: i32,
        pub created_at: NaiveDateTime,
        pub shape: String,
    }

    impl From<data::Shape> for Shape {
        fn from(shape: data::Shape) -> Self {
            Shape {
                id: 0,
                board_id: 0,
                created_at: Utc::now().naive_utc(),
                shape: serde_json::to_string(&shape).unwrap(),
            }
        }
    }

    impl From<Shape> for data::Shape {
        fn from(shape: Shape) -> Self {
            serde_json::from_str::<data::Shape>(&shape.shape).unwrap()
        }
    }

    impl Insertable for Shape {
        fn to_insert_tuples(&self) -> Vec<[String; 2]> {
            return vec![["shape".to_owned(), self.shape.clone()]];
        }
    }

    #[derive(Deserialize, PostgresMapper, Serialize, Debug)]
    #[pg_mapper(table = "boards")]
    pub struct Board {
        pub id: i32,
        pub created_at: NaiveDateTime,
        pub name: String,
    }

    impl Board {
        pub fn new(name: String) -> Board {
            Board {
                id: 0,
                created_at: Utc::now().naive_utc(),
                name,
            }
        }
    }

    impl From<Board> for data::Board {
        fn from(board: Board) -> Self {
            data::Board {
                name: board.name,
                id: board.id,
            }
        }
    }

    impl Insertable for Board {
        fn to_insert_tuples(&self) -> Vec<[String; 2]> {
            return vec![["name".to_owned(), self.name.clone()]];
        }
    }
}

use deadpool_postgres::Client;
use deadpool_postgres::{Config, Pool};
use errors::MyError;
use models::{Board, Shape};
use shared::datatypes as data;
use std::env;
use std::fmt::Write as _;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::NoTls;

use self::models::Insertable;

pub struct State {
    pub pool: Pool,
}

pub fn make_state() -> State {
    let mut cfg = Config::new();
    cfg.user = Some("postgres".to_string());
    cfg.password = Some(match env::var("DB_PASSWORD") {
        Ok(val) => val,
        Err(_) => "postgres".to_string(),
    });
    cfg.host = Some("localhost".to_string());
    cfg.dbname = Some("liveboard".to_string());
    let pool = cfg.create_pool(None, NoTls).unwrap();

    State { pool }
}

async fn get_by_id<T: FromTokioPostgresRow>(client: &Client, id: i32) -> Result<T, MyError> {
    let stmt = format!(
        "SELECT {} FROM {} WHERE id={} LIMIT 1;",
        &T::sql_table_fields(),
        &T::sql_table(),
        id
    );
    let stmt = client.prepare(&stmt).await.unwrap();
    let row = client.query_one(&stmt, &[]).await?;
    let t = T::from_row_ref(&row).unwrap();
    Ok(t)
}

async fn list<T: FromTokioPostgresRow>(
    client: &Client,
    where_statement: Option<String>,
) -> Result<Vec<T>, MyError> {
    let mut stmt = format!("SELECT {} FROM {}", &T::sql_table_fields(), &T::sql_table());
    if where_statement.is_some() {
        write!(stmt, " WHERE {}", where_statement.unwrap())?;
    }
    write!(stmt, ";")?;
    let stmt = client.prepare(&stmt).await.unwrap();
    let r = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| T::from_row_ref(row))
        .collect::<Result<Vec<T>, tokio_pg_mapper::Error>>();
    match r {
        Ok(v) => Ok(v),
        Err(_v) => Err(MyError::NotFound),
    }
}

async fn insert<T: Insertable + FromTokioPostgresRow>(
    client: &Client,
    t: &T,
) -> Result<T, MyError> {
    let mut columns: Vec<String> = vec![];
    let mut values: Vec<String> = vec![];
    for [column_name, value] in t.to_insert_tuples() {
        columns.push(column_name);
        values.push(value);
    }
    let stmt = format!(
        "INSERT INTO {} ({}) VALUES ({}) RETURNING id;",
        &T::sql_table(),
        columns.join(","),
        values
            .into_iter()
            .map(|v| format!("'{}'", v))
            .collect::<Vec<String>>()
            .join(",")
    );
    println!("stms: {}", stmt);
    let stmt = client.prepare(&stmt).await.unwrap();
    let row = client.query_one(&stmt, &[]).await?;
    let id: i32 = row.get(0);
    let t: T = get_by_id(client, id).await?;

    Ok(t)
}

pub async fn get_boards(client: &Client) -> Result<Vec<Board>, MyError> {
    list::<Board>(client, None).await
}

pub async fn create_board(client: &Client, name: String) -> Result<Board, MyError> {
    let b = Board::new(name);
    let board = insert(client, &b).await?;
    Ok(board)
}

pub async fn get_shapes(client: &Client, board_id: u32) -> Result<Vec<Shape>, MyError> {
    let shapes = list::<Shape>(client, Some(format!("board_id={}", board_id))).await?;
    Ok(shapes)
}

pub async fn create_shape(client: &Client, shape: data::Shape) -> Result<Shape, MyError> {
    let s: Shape = insert::<Shape>(client, &shape.into()).await?;
    Ok(s)
}
