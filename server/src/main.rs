mod api;

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Comment {
    id: u32,
    title: String,
    name: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct Comments {
    comments: Vec<Comment>
}

fn get_comments_handler() -> Result<HttpResponse> {
    let data = Comments{
        comments: vec![Comment {
            id: 1,
            title: "Hello World".to_string(),
            name: "Robert".to_string(),
            body: "This is my first Rust server".to_string(),
        }],
    };

    let response = api::ApiResponse::success(data);
    Ok(HttpResponse::Ok().json(response))
}

pub fn main() {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/api/comments", web::get().to(get_comments_handler))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
