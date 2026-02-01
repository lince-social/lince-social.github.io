use crate::{html::page, i18n::Translations};
use maud::{PreEscaped, html};
use std::{fs, process::Command};

pub fn compile_blog_body(source_path: &str) -> String {
    // 1. Run the CLI: typst compile <path> --format html -
    // The "-" at the end tells typst to output to stdout instead of a file
    let output = Command::new("typst")
        .arg("compile")
        .arg(source_path)
        .arg("--root")
        .arg(".")
        .arg("--format")
        .arg("html")
        .arg("--features")
        .arg("html")
        .arg("-")
        .output()
        .expect("Typst CLI not found. Install it with 'cargo install typst-cli'");

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        panic!("Typst Error in {}: {}", source_path, err);
    }

    let full_html = String::from_utf8_lossy(&output.stdout).to_string();

    // 2. Extract the inner body
    extract_body(full_html)
}

fn extract_body(full_html: String) -> String {
    if let (Some(start), Some(end)) = (full_html.find("<body>"), full_html.rfind("</body>")) {
        full_html[start + 6..end].trim().to_string()
    } else {
        full_html
    }
}

fn lang_suffix(lang: &str) -> &str {
    match lang {
        "en" => "",
        "pt-br" => ".pt-br",
        _ => ".zh",
    }
}

/// Extract the title from a Typst file by looking for the first heading
fn extract_title_from_typst(file_path: &str) -> String {
    if let Ok(content) = fs::read_to_string(file_path) {
        // Look for lines starting with `=` (Typst heading syntax)
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('=') {
                // Remove leading `=` and trim whitespace
                let title = trimmed.trim_start_matches('=').trim();
                return title.to_string();
            }
        }
    }
    // Fallback: use the filename without extension
    std::path::Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string()
}

/// Get all blog posts with their metadata
pub fn get_blog_posts() -> Vec<(String, String)> {
    let mut posts = Vec::new();

    if let Ok(entries) = fs::read_dir("src/blog") {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("typ") {
                let stem = file_path.file_stem().unwrap().to_str().unwrap();
                let title = extract_title_from_typst(file_path.to_str().unwrap());
                posts.push((stem.to_string(), title));
            }
        }
    }

    // Sort posts in reverse order (newest first)
    posts.sort_by(|a, b| b.0.cmp(&a.0));
    posts
}
pub fn generate_blog_posts(t: &Translations, suffix: &str) {
    let paths = fs::read_dir("src/blog").expect("Could not read blog directory");
    let blog_href = format!("/blog{}.html", suffix);

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
                        a href=(blog_href.clone()) { (t.blog_back_to_posts) }
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

pub fn page_blog(t: &Translations) -> String {
    let suffix = lang_suffix(t.lang_code);
    let posts = get_blog_posts();

    html! {
        main class="main-content" {
            section class="blog-header" {
                h1 class="section-title" { (t.blog_title) }
                p class="section-description" { (t.blog_posts) }
            }

            @if posts.is_empty() {
                section class="blog-posts-container" {
                    p class="no-posts" { "No blog posts yet." }
                }
            } @else {
                section class="blog-posts-container" {
                    ul class="blog-posts-list" {
                        @for (slug, title) in posts {
                            li class="blog-post-item" {
                                a class="blog-post-link" href=(format!("/blog/{}{}.html", slug, suffix)) {
                                    h3 class="blog-post-title" { (title) }
                                    span class="blog-post-slug" { (slug) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    .0
}
