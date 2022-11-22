use std::env;

use serde_json::Value;

pub async fn translate(body: String, from: String, to: String) -> String {
    let client = reqwest::Client::new();
    println!("{}", to);
    let params = [("from", from.as_str()), ("to", "es"), ("json", body.as_str()), ("protected_keys", "ingredients_inventory_id")];
    let res = client.post("https://nlp-translation.p.rapidapi.com/v1/jsontranslate")
        .header("X-RapidAPI-Key", env::var("TRANSLATE_KEY").unwrap())
        .header("X-RapidAPI-Host", "nlp-translation.p.rapidapi.com")
        .form(&params)
        .send()
        .await.unwrap()
        .text()
        .await.unwrap();
    let res: Value = serde_json::from_str(res.as_str()).unwrap();
    let translated = res["translated_json"].to_string();
    let translated: Value = serde_json::from_str(&translated).unwrap();
    let pre_slice = translated[to].to_string().replace("\\\"", "\"");
    pre_slice[1..pre_slice.len() - 1].to_owned()
}