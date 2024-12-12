use reqwest::Client;
use scraper::{Html, Selector};

pub async fn asciify(text: &str, font: &str) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://fastapi-text-asciify-npt1siwyq-ganmahmud.vercel.app/asciify/?text={}&font={}",
        text, font
    );

    let client = Client::new();
    let response = client.get(&url).send().await?;

    let html = response.text().await?;

    // Parsear el HTML
    let fragment = Html::parse_document(&html);
    let selector = Selector::parse("pre").unwrap();

    // Intentar encontrar el primer elemento <pre>
    if let Some(pre_element) = fragment.select(&selector).next() {
        // Si encontramos el elemento, obtenemos su contenido
        let pre = pre_element.inner_html();
        Ok(pre)
    } else {
        // Si no encontramos el elemento <pre>, manejamos el error
        Ok(":p".to_string())
    }
}
