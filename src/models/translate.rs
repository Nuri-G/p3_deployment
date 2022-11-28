use std::env;
use serde_json::Value;

pub async fn translate(text: String, from: &str, to: &str) -> String {
    let client = reqwest::Client::new();
    let translate_key = env::var("TRANSLATE_KEY").unwrap();
    let qu = [("q", text.as_str()), ("source", from), ("target", to), ("key", translate_key.as_str())];
    let url = "https://translation.googleapis.com/language/translate/v2";
    let res = client.post(url)
        .query(&qu)
        .header("content-length", 0)
        .send()
        .await.unwrap().text().await.unwrap();
    
    let json_val: Value = serde_json::from_str(res.as_str()).unwrap();
    json_val["data"]["translations"][0]["translatedText"].as_str().unwrap().to_owned()
}
