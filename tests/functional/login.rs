
use awesome_rust_web_app::app;
use actix_web::{test};
use serde_json::json;

#[actix_rt::test]
async fn test_login() {
    app::setup(app::AppEnv::Test);

    let mut service = test::init_service(
        app::boot()
        ).await;

    let message = json!({
            "username": "sam.smith",
            "password": "1234Qwer",
        });

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&message)
        .to_request();

    let resp = test::call_service(&mut service, req).await;

    assert!(resp.status().is_success());
}