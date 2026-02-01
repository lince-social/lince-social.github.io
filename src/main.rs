use crate::{
    html::page,
    i18n::get_translations,
    pages::{
        blog::{generate_blog_posts, page_blog},
        index::page_index,
    },
};
use std::fs;

mod html;
mod i18n;
mod macros;
mod pages;

fn main() {
    let translations = get_translations();
    fs::create_dir_all("output/blog").expect("Failed to create output directory");

    for (lang_code, t) in &translations {
        let suffix = if lang_code == &"en" {
            "".to_string()
        } else {
            format!(".{}", lang_code)
        };

        generate_blog_posts(t, &suffix);

        let pages = [("index", page_index(t)), ("blog", page_blog(t))];

        for (name, content) in pages {
            let html_out = page(&content, t, name);
            fs::write(format!("output/{}{}.html", name, suffix), html_out).unwrap();
        }
    }
}
