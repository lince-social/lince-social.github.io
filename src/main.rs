use crate::{
    html::page,
    i18n::get_translations,
    pages::{about::page_about, index::page_index},
};
use std::fs;

mod html;
mod i18n;
mod pages;

fn main() {
    let translations = get_translations();

    // Generate pages for each language
    for (lang_code, t) in &translations {
        let suffix = if *lang_code == "en" {
            ""
        } else {
            &format!(".{}", lang_code)
        };

        // Index page
        let index_html = page(&page_index(t), t, "index");
        fs::write(format!("output/index{}.html", suffix), index_html).unwrap();

        // About page
        let about_html = page(&page_about(t), t, "about");
        fs::write(format!("output/about{}.html", suffix), about_html).unwrap();
    }

    println!("Generated pages for {} languages", translations.len());
}
