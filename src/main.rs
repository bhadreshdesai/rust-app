use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, routes()).await.unwrap();
}

fn routes() -> Router {
    Router::new().route("/plaintext", get(|| async { "Hello, World!!" }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn plaintext() {
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
}
