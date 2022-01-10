pub mod dictcc {
    use std::error::Error;

    use scraper::{Html, Selector};

    pub fn translate(
        translate_from: &str,
        translate_to: &str,
        word_to_translate: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let url = generate_url(translate_from, translate_to, word_to_translate);
        let html = get_html(&url)?;
        Ok(get_definitions(html))
    }

    fn generate_url(translate_from: &str, translate_to: &str, word_to_translate: &str) -> String {
        if translate_from.is_empty() && translate_to.is_empty() {
            format!("https://dict.cc/?s={}", word_to_translate)
        } else {
            format!(
                "https://{}{}.dict.cc/?s={}",
                translate_from, translate_to, word_to_translate
            )
        }
    }

    fn get_definitions(html: Html) -> Vec<String> {
        let td_selector =
            Selector::parse("td.td7nl").expect("Website layout has changed! Raise an issue");
        let a_selector = Selector::parse("a").expect("Website layout has changed! Raise an issue");

        let mut definition = vec![];

        // Iterating over selector td.td7nl (which is this part of the page: https://imgur.com/jgMfZgn)
        // Then getting all the <a> tags (the contents of the table are links)
        html.select(&td_selector).into_iter().for_each(|td| {
            let html = Html::parse_document(&td.inner_html());
            html.select(&a_selector).into_iter().for_each(|a| {
                let definition_inner_html = a.inner_html();
                definition.push(definition_inner_html);
            })
        });

        // Filtering out anything which is in bold (see the above image. Notice that all english counterparts are in bold?)
        // `kbd` `abbr` and `Unverified` were other pesky mosquitoes
        let mut definitions = definition
            .into_iter()
            .filter(|x| {
                !x.starts_with("<kbd")
                    && !x.starts_with("<abbr")
                    && !x.starts_with("<b>")
                    && !x.contains("Unverified")
            })
            .collect::<Vec<String>>();

        definitions.drain(5..); // getting the first 5 definitions

        definitions
    }

    fn get_html(url: &str) -> Result<Html, Box<dyn Error>> {
        let response = reqwest::blocking::get(url)?.text()?;
        let html = Html::parse_document(&response);
        Ok(html)
    }
}