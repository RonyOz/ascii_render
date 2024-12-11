// This module is focused in convert strings to big ascii art. Use an API
// https://fastapi-text-asciify-npt1siwyq-ganmahmud.vercel.app/asciify/?text={text}&font={broadway}

use reqwest::Client;

pub async fn asciify(text: &str, font: &str) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://fastapi-text-asciify-npt1siwyq-ganmahmud.vercel.app/asciify/?text={}&font={}",
        text, font
    );

    let client = Client::new();
    let response = client.get(&url).send().await?;

    response.text().await
}


