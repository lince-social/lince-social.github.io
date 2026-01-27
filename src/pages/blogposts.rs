use crate::i18n::Translations;
use maud::html;
use std::fs;

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

pub fn page_blogposts(t: &Translations) -> String {
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
