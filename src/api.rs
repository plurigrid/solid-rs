use reqwest::Client;
use serde::Deserialize;
use uuid::Uuid;

use crate::language_parser::LanguageInput;
use crate::solidity_generator::SolidityOutput;

const API_URL: &str = "http://localhost:8000";

#[derive(Deserialize)]
struct ApiResponse {
    id: Uuid,
    result: SolidityOutput,
}

pub async fn send_language_input(input: LanguageInput) -> Result<SolidityOutput, reqwest::Error> {
    let client = Client::new();
    let res = client.post(API_URL)
        .json(&input)
        .send()
        .await?;

    let api_response: ApiResponse = res.json().await?;
    Ok(api_response.result)
}