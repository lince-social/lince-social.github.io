use crate::{
    blogposts_generator::compile_blog_body,
    html::page,
    i18n::{Translations, get_translations},
    pages::{about::page_about, blogposts::page_blogposts, index::page_index},
};
use maud::{PreEscaped, html};
use std::fs;

mod blogposts_generator;
mod html;
mod i18n;
mod macros;
mod pages;

fn main() {
    let translations = get_translations();
    setup_output_dirs();

    for (lang_code, t) in &translations {
        let suffix = get_lang_suffix(lang_code);

        // 1. Generate core site pages
        generate_static_pages(t, &suffix);

        // 2. Generate blog posts from Typst files
        generate_blog_posts(t, &suffix);
    }

    println!(
        "Site generation complete for {} languages.",
        translations.len()
    );
}

// --- Helper Functions ---

fn setup_output_dirs() {
    fs::create_dir_all("output/blog").expect("Failed to create output directory");
}

fn get_lang_suffix(lang_code: &str) -> String {
    if lang_code == "en" {
        "".to_string()
    } else {
        format!(".{}", lang_code)
    }
}

fn generate_static_pages(t: &Translations, suffix: &str) {
    let pages = [
        ("index", page_index(t)),
        ("about", page_about(t)),
        ("blogposts", page_blogposts(t)),
    ];

    for (name, content) in pages {
        let html_out = page(&content, t, name);
        fs::write(format!("output/{}{}.html", name, suffix), html_out).unwrap();
    }
}

fn generate_blog_posts(t: &Translations, suffix: &str) {
    let paths = fs::read_dir("src/blog").expect("Could not read blog directory");
    let blogposts_href = format!("/blogposts{}.html", suffix);

    for entry in paths.flatten() {
        let file_path = entry.path();

        if file_path.extension().and_then(|s| s.to_str()) == Some("typ") {
            let stem = file_path.file_stem().unwrap().to_str().unwrap();

            // Compile Typst content
            let body = compile_blog_body(file_path.to_str().unwrap());

            // Wrap in Maud with breadcrumbs
            let markup = html! {
                main class="main-content" {
                    nav class="breadcrumbs" {
                        a href=(blogposts_href.clone()) { (t.blog_back_to_posts) }
                    }
                    article class="blog_post" { (PreEscaped(body)) }
                }
            };
            let blog_post_page = format!("blog/{}", stem);
            let final_html = page(&markup.0, t, &blog_post_page);

            fs::write(format!("output/blog/{}{}.html", stem, suffix), final_html).unwrap();
        }
    }
}
