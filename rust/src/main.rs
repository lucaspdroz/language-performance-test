use actix_web::{post, get, web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::{PgPool, Row};
use dotenvy::dotenv;
use std::env;
use chrono::{NaiveDate, NaiveTime, DateTime, Utc}; // Added DateTime and Utc

use uuid::Uuid;

// Struct for handling incoming request data
#[derive(Deserialize)]
struct EncodeRequest {
    date: NaiveDate,
    time: NaiveTime,
    batch: String,
}

// Struct for handling outgoing response data
#[derive(Serialize)]
struct EncodeResponse {
    id: String,
    date: NaiveDate,
    time: NaiveTime,
    batch: String,
}

// Temporary struct for reliable SQLX decoding.
// - 'date' is DateTime<Utc> to handle the driver's TIMESTAMPTZ inference.
// - 'time' is String to handle the driver's TEXT inference.
struct DecodedTokenHelper {
    id: String,
    date: DateTime<Utc>, 
    time: String, // CHANGED: Decode as String (TEXT) to bypass `TIME` mismatch error
    batch: String,
}

#[post("/encode")]
async fn encode(
    pool: web::Data<PgPool>,
    req: web::Json<EncodeRequest>,
) -> impl Responder {
    let id = Uuid::new_v4().to_string();

    // Insertion is confirmed to work correctly using `sqlx::query` and `.bind()`.
    let result = sqlx::query(
        r#"
        INSERT INTO tokens (id, date, time, batch)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(&id)
    .bind(req.date)
    .bind(req.time)
    .bind(&req.batch)
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
            // Use eprintln! for error logging
            eprintln!("DB Error during encode: {:?}", e);
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

    // Fetch the row using simple `sqlx::query`.
    let row_result = sqlx::query(
        r#"
        SELECT id, date, time, batch FROM tokens
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await;

    match row_result {
        Ok(row) => {
            // Encapsulate the struct creation in a block that returns Result.
            let helper_result: Result<DecodedTokenHelper, sqlx::Error> = (|| {
                Ok(DecodedTokenHelper {
                    id: row.try_get("id")?,
                    date: row.try_get("date")?, 
                    time: row.try_get("time")?, // Retrieves time as String
                    batch: row.try_get("batch")?,
                })
            })(); // immediately executed closure

            match helper_result {
                Ok(helper) => {
                    // Manual conversion: Extract NaiveDate and parse NaiveTime from String
                    let time_naive = match NaiveTime::parse_from_str(&helper.time, "%H:%M:%S") {
                        Ok(t) => t,
                        Err(e) => {
                            eprintln!("Time parsing error for '{}': {:?}", helper.time, e);
                            return HttpResponse::InternalServerError().finish();
                        }
                    };

                    HttpResponse::Ok().json(EncodeResponse {
                        id: helper.id,
                        date: helper.date.date_naive(), // Use .date_naive() to get NaiveDate
                        time: time_naive, // Use the manually parsed time
                        batch: helper.batch,
                    })
                },
                Err(e) => {
                    // If any try_get failed
                    eprintln!("DB Decode Error (try_get failed with manual time type): {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        // Handle RowNotFound specifically for a 404 response
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(e) => {
            eprintln!("DB Error during decode (fetch_one failed): {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
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
