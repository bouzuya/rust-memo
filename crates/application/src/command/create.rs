use std::collections::HashMap;

use use_case::HasNewPageUseCase;

#[derive(serde::Serialize)]
struct Body {
    content: String,
}

pub fn create<App: HasNewPageUseCase>(_app: App, title: Option<&str>) -> anyhow::Result<()> {
    let content = title.map(|s| format!("# {}", s)).unwrap_or_default();
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let mut form = HashMap::new();
    form.insert("content", content);
    let response = client
        .post("http://localhost:3000/pages")
        .form(&form)
        .send()?;
    let created = if response.status().is_redirection() {
        if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
            match location.to_str() {
                Err(_) => Err(anyhow::anyhow!(
                    "location header to_str failed : {:?}",
                    location
                )),
                Ok(s) => match s.strip_prefix("/pages/") {
                    Some(s) => Ok(format!("{}.md", s)),
                    None => Err(anyhow::anyhow!(
                        "location header does not start with /pages/ : {}",
                        s
                    )),
                },
            }
        } else {
            Err(anyhow::anyhow!(
                "location header is not found : {:?}",
                response.headers()
            ))
        }
    } else {
        Err(anyhow::anyhow!(
            "status is not redirection : {}",
            response.status()
        ))
    };
    if let Ok(created) = created {
        println!("{}", created);
    }
    Ok(())
}
