use crate::api::{build_router, make_server};
use axum::http::{Request, StatusCode};
use axum_test::TestServer;
use hyper::Body;

#[tokio::test]
async fn test_games_can_be_created() {
    let app = make_server().await;

    let server = TestServer::new(app).unwrap();

    let response = server.get("/game/new").await;

    response.assert_status(StatusCode::OK);

    let bytes = response.bytes();
    let body_str = std::str::from_utf8(&bytes).unwrap();

    assert_eq!(
        body_str,
        r#"{"name":"new game name","dataset":"new game dataset"}"#
    );
}
