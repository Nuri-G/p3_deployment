use std::{env, collections::HashMap, sync::Mutex};
use actix_web::{get, Responder, web::{Path, self}, Result};
use serde_json::Value;

pub struct TranslationCache {
    pub values: Mutex<HashMap<String, String>>,
}

/// Translates text from the language 'from' to 'to'.
pub async fn translate(text: String, from: String, to: String) -> String {
    let client = reqwest::Client::new();
    let translate_key = env::var("TRANSLATE_KEY").unwrap();
    let qu = [("q", text.as_str()), ("source", from.as_str()), ("target", to.as_str()), ("key", translate_key.as_str())];
    let url = "https://translation.googleapis.com/language/translate/v2";
    let res = client.post(url)
        .query(&qu)
        .header("content-length", 0)
        .send()
        .await.unwrap().text().await.unwrap();
    
    let json_val: Value = serde_json::from_str(res.as_str()).unwrap();
    let out = json_val["data"]["translations"][0]["translatedText"].as_str().unwrap().to_owned();
    out
}

/// Returns a JSON object with regularly used words translated from English to the target language.
/// Caches the result of any translations to reduce future API calls.
#[get("api/translated_words/{language}")]
pub async fn translated_keywords(state: web::Data<TranslationCache>, language: Path<String>) -> Result<impl Responder> {
    const FROM: &str = "en";
    let language = language.into_inner();
    match state.values.lock() {
        Ok(values) => {
            if values.contains_key(&language) {
                let out = values.get(&language).unwrap().to_owned();
                return Ok(out);
            }
        },
        Err(_) => panic!("Translation mutex was poisoned."),
    }
    let all = translate("All".to_owned(), FROM.to_owned(), language.to_owned()).await;
    let order = translate("Your Order".to_owned(), FROM.to_owned(), language.to_owned()).await;
    let pay = translate("Pay".to_owned(), FROM.to_owned(), language.to_owned()).await;
    let clear = translate("Clear All".to_owned(), FROM.to_owned(), language.to_owned()).await;
    let tax = translate("Tax".to_owned(), FROM.to_owned(), language.to_owned()).await;
    let removed = translate("Removed".to_owned(), FROM.to_owned(), language.to_owned()).await;
    let created = translate("Order Created".to_owned(), FROM.to_owned(), language.to_owned()).await;
    let cleared = translate("Order Cleared".to_owned(), FROM.to_owned(), language.to_owned()).await;
    let mut json_text = format!(r#"
            "all": "{}",
            "order": "{}",
            "pay": "{}",
            "clear": "{}",
            "tax": "{}",
            "removed": "{}",
            "created": "{}",
            "cleared": "{}"
    "#, all, order, pay, clear, tax, removed, created, cleared);
    json_text = "{".to_owned() + json_text.as_str();
    json_text = json_text + "}";
    let out = serde_json::from_str::<Value>(&json_text).unwrap().to_string();
    match state.values.lock() {
        Ok(mut values) => {
            values.insert(language, out.clone());
        },
        Err(_) => panic!("Translation mutex was poisoned."),
    }
    Ok(out)
}
