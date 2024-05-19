use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

type Db = Arc<RwLock<HashMap<i32, Todo>>>;

async fn todos_get(State(db): State<Db>) -> impl IntoResponse {
    let todos = db.read().unwrap();
    let todos = todos.values().cloned().collect::<Vec<_>>();
    return Json(todos);
}

fn routes() -> Router {
    let db = Db::default();
    // create a ToDo
    let todo = Todo {
        id: 1,
        text: "First ToDo".to_owned(),
        completed: false,
    };
    db.write().unwrap().insert(todo.id, todo.clone());
    let todo = Todo {
        id: 2,
        text: "Second ToDo".to_owned(),
        completed: false,
    };
    db.write().unwrap().insert(todo.id, todo.clone());

    return Router::new()
        .route("/plaintext", get(|| async { "Hello, World!!" }))
        .route("/todos", get(todos_get))
        .with_state(db);
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, routes()).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use serde_json::{json, Value};
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn test_plaintext() {
        let routes = routes();
        let response = routes
            .oneshot(
                Request::builder()
                    .uri("/plaintext")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        //assert_eq!(&body[..], b"Hello, World!");
        // convert to string for comparision so if there is a difference it is displayed as string instead of bytes which is difficult to review
        assert_eq!(String::from_utf8(body.to_vec()).unwrap(), "Hello, World!!");
    }

    #[tokio::test]
    async fn test_todos_get() {
        let routes = routes();
        let response = routes
            .oneshot(
                Request::builder()
                    .uri("/todos")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            json!([{ "id": 1, "text": "First ToDo", "completed": false}, { "id": 2, "text": "Second ToDo", "completed": false}])
        );
    }
}
