#![allow(non_snake_case)]

use crate::errors::TranslateError;
use crate::languages::is_supported_language;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct TranslationResult {
    pub text: String,
    pub from: LanguageInfo,
    pub raw: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LanguageInfo {
    pub language: LanguageDetail,
    pub text: TextDetail,
}

#[derive(Serialize, Deserialize)]
pub struct LanguageDetail {
    pub didYouMean: bool,
    pub iso: String,
}

#[derive(Serialize, Deserialize)]
pub struct TextDetail {
    pub autoCorrected: bool,
    pub value: String,
    pub didYouMean: bool,
}

pub async fn translate_text(
    text: &str,
    from: Option<&str>,
    to: Option<&str>,
    raw: bool,
    api_key: &str,
) -> Result<TranslationResult, Box<dyn Error>> {
    let from_lang = from.unwrap_or("auto");
    let to_lang = to.unwrap_or("en");

    if !is_supported_language(from_lang) || !is_supported_language(to_lang) {
        return Err(Box::new(TranslateError::new(
            400,
            &format!("The language '{}' is not supported", from_lang),
        )));
    }

    let client = Client::new();
    let url = format!(
        "https://translation.googleapis.com/language/translate/v2?key={}",
        api_key
    );

    let request_body = serde_json::json!({
        "q": text,
        "source": from_lang,
        "target": to_lang,
        "format": "text"
    });

    let res = client.post(&url).json(&request_body).send().await?;

    if res.status().is_success() {
        let body: serde_json::Value = res.json().await?;
        let result = process_response(&body, raw);
        Ok(result)
    } else {
        Err(Box::new(TranslateError::new(400, "Bad request")))
    }
}

fn process_response(body: &serde_json::Value, raw: bool) -> TranslationResult {
    let mut result = TranslationResult {
        text: String::new(),
        from: LanguageInfo {
            language: LanguageDetail {
                didYouMean: false,
                iso: String::new(),
            },
            text: TextDetail {
                autoCorrected: false,
                value: String::new(),
                didYouMean: false,
            },
        },
        raw: None,
    };

    if let Some(translations) = body["data"]["translations"].as_array() {
        if let Some(translation) = translations.first() {
            result.text = translation["translatedText"]
                .as_str()
                .unwrap_or("")
                .to_string();
            result.from.language.iso = translation["detectedSourceLanguage"]
                .as_str()
                .unwrap_or("")
                .to_string();
        }
    }

    if raw {
        result.raw = Some(body.to_string());
    }

    result
}
