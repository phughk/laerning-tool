#[cfg(test)]
mod test {
    use crate::api;

    use crate::api::game::game_library::{Game, GameStats};
    use axum::http::StatusCode;
    use axum_test::http::header::CONTENT_TYPE;
    use axum_test::http::HeaderValue;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_games_can_be_created() {
        let db = crate::start_db().await;
        let repo = crate::repository::new(db);
        let api = api::new(repo);
        let app = api.make_server().await;

        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/game/new")
            .add_header(
                CONTENT_TYPE,
                HeaderValue::from_str("application/json").unwrap(),
            )
            .text(
                r#"{
                   "name": "any name",
                   "dataset": "any dataset"
                }"#,
            )
            .await;

        assert_eq!(StatusCode::OK, response.status_code(), "{:?}", response);

        let bytes = response.as_bytes();
        let body_str = std::str::from_utf8(bytes).unwrap();
        let game: Game = serde_json::from_str(body_str).unwrap();
        assert_eq!(
            game,
            Game {
                name: "⟨any name⟩".to_string(),
                dataset: "⟨any dataset⟩".to_string(),
                current_question: None,
                stats: GameStats {
                    current_question: 1,
                    total_questions: 2,
                    current_try: 3,
                    max_tries: 4,
                    duration: 5,
                    average_question_duration: 6.0,
                },
            }
        );
    }
}
