use crate::AppResponse::{Accepted, Unauthorized};
use crate::sender::DynMtbFileSender;
use crate::{CONFIG, auth, shutdown_signal};
use axum::body::Body;
use axum::http::Request;
use axum::http::header::AUTHORIZATION;
use axum::middleware::{Next, from_fn};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Extension, Router};
use tower_http::trace::TraceLayer;

pub async fn start_server(sender: DynMtbFileSender) -> Result<(), String> {
    match tokio::net::TcpListener::bind(&CONFIG.listen).await {
        Ok(listener) => {
            log::info!("Starting application listening on '{}'", CONFIG.listen);
            if let Err(err) = axum::serve(listener, routes(sender))
                .with_graceful_shutdown(shutdown_signal())
                .await
            {
                return Err(err.to_string());
            }
        }
        Err(err) => return Err(format!("Cannot listening on '{}': {}", CONFIG.listen, err)),
    }

    Ok(())
}

pub async fn handle_post(
    Extension(sender): Extension<DynMtbFileSender>,
    payload: String,
) -> Response {
    let Err(e) = sender.send(&payload).await else {
        return Accepted.into_response();
    };
    e.into_response()
}

pub fn routes(sender: DynMtbFileSender) -> Router {
    Router::new()
        .route("/", post(handle_post))
        .layer(Extension(sender))
        .layer(from_fn(check_basic_auth))
        .layer(TraceLayer::new_for_http())
}

async fn check_basic_auth(request: Request<Body>, next: Next) -> Response {
    if let Some(Ok(auth_header)) = request.headers().get(AUTHORIZATION).map(|x| x.to_str())
        && auth::check_basic_auth(auth_header, &CONFIG.token)
    {
        return next.run(request).await;
    }
    log::warn!("Invalid authentication used");
    Unauthorized.into_response()
}

#[cfg(test)]
mod tests {
    use crate::AppResponse::{Accepted, InternalServerError, Unauthorized};
    use crate::sender::{DynMtbFileSender, MockMtbFileSender};
    use crate::server::routes;
    use axum::body::Body;
    use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, WWW_AUTHENTICATE};
    use axum::http::{Method, Request, StatusCode};
    use axum::response::IntoResponse;
    use std::sync::Arc;
    use tower::ServiceExt;

    #[test]
    fn should_return_success_response() {
        let response = Accepted.into_response();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
    }

    #[test]
    fn should_return_error_response() {
        let response = InternalServerError.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn should_return_unauthorized_response() {
        let response = Unauthorized.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert!(response.headers().contains_key(WWW_AUTHENTICATE));
    }

    #[tokio::test]
    #[allow(clippy::expect_used)]
    async fn should_handle_post_request() {
        let mut sender_mock = MockMtbFileSender::new();

        sender_mock
            .expect_send()
            .once()
            .return_once(move |_| Ok(()));

        let router = routes(Arc::new(sender_mock) as DynMtbFileSender);
        let body = Body::from(include_str!("../test-files/example.json"));

        let response = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/")
                    .header(AUTHORIZATION, "Basic dG9rZW46dmVyeS1zZWNyZXQ=")
                    .header(CONTENT_TYPE, "application/json")
                    .body(body)
                    .expect("request built"),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::ACCEPTED);
    }
}
