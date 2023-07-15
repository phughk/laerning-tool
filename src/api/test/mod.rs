




#[tokio::test]
async fn test_games_can_be_created() {
    let api = api::new(vec![]);
    let app = api.make_server().await;

    let server = TestServer::new(app).unwrap();

    let response = server.get("/game/new").await;

    response.assert_status(StatusCode::OK);

    let bytes = response.bytes();
    let body_str = std::str::from_utf8(&bytes).unwrap();

    assert_eq!(
        body_str,
        r#"{"name":"new game name 0","dataset":"new game dataset 0"}"#
    );
}
