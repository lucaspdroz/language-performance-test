use actix_web::{post, get, web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;
use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

#[derive(Deserialize)]
struct EncodeRequest {
    date: NaiveDate,
    time: NaiveTime,
    batch: String,
}

#[derive(Deserialize)]
struct DecodeRequest {
    id: String,
}

#[derive(Serialize)]
struct EncodeResponse {
    id: String,
    date: NaiveDate,
    time: NaiveTime,
    batch: String,
}

#[post("/encode")]
async fn encode(
    pool: web::Data<PgPool>,
    req: web::Json<EncodeRequest>,
) -> impl Responder {
    let id = Uuid::new_v4().to_string();

    let result = sqlx::query!(
        r#"
        INSERT INTO tokens (id, date, time, batch)
        VALUES ($1, $2, $3, $4)
        "#,
        id,
        req.date,
        req.time,
        req.batch
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(EncodeResponse {
            id,
            date: req.date,
            time: req.time,
            batch: req.batch.clone(),
        }),
        Err(e) => {
            println!("DB Error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/decode/{id}")]
async fn decode(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();

    let row = sqlx::query!(
        r#"
        SELECT id, date, time, batch FROM tokens
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool.get_ref())
    .await;

    match row {
        Ok(row) => HttpResponse::Ok().json(EncodeResponse {
            id: row.id,
            date: row.date, // agora sem .to_string()
            time: row.time, // agora sem .to_string()
            batch: row.batch,
        }),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to DB");

    println!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(encode)
            .service(decode)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
