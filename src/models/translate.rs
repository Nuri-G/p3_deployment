use std::env;
use actix_web::{get, Responder, web::Path, Result};
use serde_json::Value;

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
    json_val["data"]["translations"][0]["translatedText"].as_str().unwrap().to_owned()
}

#[get("api/translated_words/{language}")]
pub async fn translated_keywords(language: Path<String>) -> Result<impl Responder> {
    const FROM: &str = "en";
    let language = language.into_inner();
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
    let out: Value = serde_json::from_str(&json_text).unwrap();
    Ok(out.to_string())
}
