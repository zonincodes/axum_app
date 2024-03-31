use axum::{
    body::Body,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::{PgPoolOptions, PgRow}, PgPool, Row};
use std::env;
use colored::Colorize;

// user
#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

struct Roww {
    id: i64,
    user: Json<User>,
}

// A struct for the JSON body
#[derive(Deserialize)]
struct Page {
    number: u32,
}
// A handler to demonstrate path and query extractors

async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item {} on page {}", id, page.number)
}
// handler for /create-user
async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created Successfully"))
        .unwrap()
}

async fn create_user_db(Extension(pool): Extension<PgPool>, user: Json<User>) -> impl IntoResponse {
    let id:i64 = user.id as i64;
    let rec = sqlx::query(
        r#"INSERT INTO users ( id, name, email)
        VALUES ( $1, $2, $3 )
        RETURNING id"#,
    )
    .bind(&id)
    .bind(&user.name)
    .bind(&user.email)
    .fetch_one(&pool)
    .await;

    match  rec {
        Ok(user) => {
            let js = json!({
                "id": user.try_get::<i32, _>("id").unwrap_or_default(),
                "name": user.try_get::<String, _>("name").unwrap_or_default(),
                "email": user.try_get::<String, _>("email").unwrap_or_default(),
            });
            return  (axum::http::StatusCode::OK, Json(js)).into_response();
        }
    
    Err(e) => return  (axum::http::StatusCode::NOT_IMPLEMENTED).into_response(),
    }

    
}

// a handler to demonstrate the JSON body extractor
async fn add_user(Json(user): Json<User>) -> Json<User> {
    Json(user)
}

// get users
async fn get_users(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let rows = match sqlx::query(r#"SELECT * FROM users"#).fetch_all(&pool).await {
        Ok(rows) => rows,
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error {}", e),
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(users)).into_response()
}

// Handler for /users
async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Jane".to_string(),
            email: "jane@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Wambui".to_string(),
            email: "wambui@murindu.com".to_string(),
        },
    ];
    Json(users)
}

// define a handler that performs an operation and may return an error
async fn delete_user(Path(user_id): Path<u64>) -> Result<Json<User>, impl IntoResponse> {
    match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(User {
            id: user_id,
            name: "Deleted User".into(),
            email: "deleted@example.com".into(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user: {}", e),
        )),
    }
}

// Hypothetical async function to delete a user by ID
async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    // Simulate an error for demo
    if user_id == 1 {
        Err("User cannot be deleted.".to_string())
    } else {
        Ok(())
    }
}
#[tokio::main]
async fn main() {
    // address thet server eill bind to
    let db_url = env::var("DB_URL").expect("NOT FOUND");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("Failed to connnect");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = Router::new()
        .route("/", get(index))
        .route("/user", post(add_user))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users))
        .route("/item/:id", get(show_item))
        .route("/user/:id", delete(delete_user))
        .route("/user-get", get(get_users))
        .route("/db-user", post(create_user_db))
        .layer(Extension(pool));
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    println!("Listening on {}", "0.0.0.0:3000".purple());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Hello, Axum"
}
