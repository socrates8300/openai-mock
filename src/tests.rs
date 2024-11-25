#[cfg(test)]
mod tests {
use actix_web::{test, App};
use crate::handlers::completions_handler;
use crate::models::completion::CompletionRequest;
use serde_json::json;
#[actix_web::test]

async fn test_completions_handler() {
    // Initialize the mock service
    let app = test::init_service(
        App::new()
            .service(
                actix_web::web::resource("/v1/completions")
                    .route(actix_web::web::post().to(completions_handler)),
            )
    ).await;

    // Create a sample CompletionRequest
    let req_payload = CompletionRequest {
        model: "gpt-3.5-turbo".to_string(),
        prompt: Some(json!("Hello, world!")),
        ..Default::default()
    };

    // Create POST request
    let req = test::TestRequest::post()
        .uri("/v1/completions")
        .set_json(&req_payload)
        .to_request();

    // Send request and get the response
    let resp = test::call_service(&app, req).await;

    // Assert the response status is 200 OK
    assert!(resp.status().is_success());

    // Parse the response body
    let response_body: serde_json::Value = test::read_body_json(resp).await;

    // Assert the response contains expected fields
    assert_eq!(response_body["model"], "gpt-3.5-turbo");
    assert!(response_body["choices"].is_array());
    // Add more assertions as needed
}
}