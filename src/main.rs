extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate serde_json;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct Repository {
    name: String,
    link: String,
    description: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL do seu perfil no GitHub
    let url = "https://github.com/jhenrique04";

    // Realiza uma solicitação HTTP para obter o conteúdo da página
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?;
    let body = response.text()?;

    // Analisa o HTML da página
    let document = Html::parse_document(&body);

    // Use o seletor CSS correto para extrair os repositórios fixados
    let repo_selector = Selector::parse("li.pinned-item-list-item").unwrap();
    let mut repositories = HashMap::new();

    for element in document.select(&repo_selector) {
        let name_selector = Selector::parse("a").unwrap();
        let description_selector = Selector::parse("p.mb-0").unwrap();

        let name = element
            .select(&name_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let link = format!(
            "https://github.com{}",
            element
                .select(&name_selector)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
        );
        let description = element
            .select(&description_selector)
            .next()
            .map(|desc| desc.text().collect::<String>())
            .unwrap_or_default();

        let repository = Repository {
            name,
            link,
            description,
        };

        repositories.insert(repository.name.clone(), repository);
    }

    // Serialize os resultados em JSON e salve em um arquivo
    let json_data = serde_json::to_string_pretty(&repositories)?;

    // Salve o JSON em um arquivo chamado "repos.json"
    let mut file = File::create("repos.json")?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}
