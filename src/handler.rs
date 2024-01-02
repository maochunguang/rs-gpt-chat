use reqwest::blocking::Client;

pub fn chat_gpt_say(text: &str, api_key: &str) -> Result<String, reqwest::Error> {
    let url = "https://api.openai.com/v1/chat/completions";
    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo", // Replace with the correct GPT-3.5 model identifier
            "messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": text}]
        }))
        .send()?
        .text()?;
    println!("response:{}", response);
    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();
    let choices = response_json["choices"].as_array().unwrap();
    let reply = choices[0]["message"]["content"].as_str().unwrap();
    Ok(reply.to_string())
}

pub fn chat_gemini_say(text: &str, api_key: &str) -> Result<String, reqwest::Error> {
    let api_url =
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=";
    let url = format!("{}{}", api_url, api_key);
    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "contents": [{"parts":[{"text": text}]}]
        }))
        .send()?
        .text()?;
    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();
    let choices = response_json["candidates"].as_array().unwrap();
    let replys = choices[0]["content"]["parts"].as_array().unwrap();
    let reply = replys[0]["text"].as_str().unwrap();
    Ok(reply.to_string())
}
