use crate::{html::page, i18n::Translations};
use maud::{PreEscaped, html};
use std::{fs, path::Path, process::Command};

#[derive(Default)]
struct BlogMetadata {
    title: Option<String>,
    date: Option<String>,
}

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

fn tinymist_sidecar_path(stem: &str, _source_path: &Path) -> Option<String> {
    let sidecar_path = format!("output/assets/blog/posts/{stem}.tinymist.html");
    let sidecar = Path::new(&sidecar_path);
    if sidecar.exists() {
        return Some(sidecar_path);
    }
    None
}

fn generate_svg_sidecar(stem: &str, source_path: &Path) -> Option<String> {
    let sidecar_path = format!("output/assets/blog/posts/{stem}.tinymist.html");
    let sidecar = Path::new(&sidecar_path);
    let components = Path::new("output/assets/blog/components.typ");

    let sidecar_mtime = fs::metadata(sidecar).and_then(|m| m.modified()).ok();
    let source_mtime = fs::metadata(source_path).and_then(|m| m.modified()).ok();
    let components_mtime = fs::metadata(components).and_then(|m| m.modified()).ok();

    let needs_regen = match sidecar_mtime {
        None => true,
        Some(sidecar_time) => {
            let source_newer = source_mtime.map(|t| t > sidecar_time).unwrap_or(false);
            let components_newer = components_mtime.map(|t| t > sidecar_time).unwrap_or(false);
            source_newer || components_newer
        }
    };

    if !needs_regen {
        return Some(sidecar_path);
    }

    let output = Command::new("tinymist")
        .arg("compile")
        .arg(source_path)
        .arg(&sidecar_path)
        .arg("--root")
        .arg(".")
        .arg("--format")
        .arg("svg")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    Some(sidecar_path)
}

fn extract_first_block(input: &str, start_marker: &str, end_marker: &str) -> Option<String> {
    let start = input.find(start_marker)?;
    let end_rel = input[start..].find(end_marker)?;
    let end = start + end_rel + end_marker.len();
    Some(input[start..end].to_string())
}

fn extract_balanced_svg(input: &str, start: usize) -> Option<String> {
    let mut cursor = start;
    let mut depth = 0usize;

    while cursor < input.len() {
        let next_open = input[cursor..].find("<svg").map(|p| cursor + p);
        let next_close = input[cursor..].find("</svg>").map(|p| cursor + p);

        match (next_open, next_close) {
            (Some(open), Some(close)) if open < close => {
                depth += 1;
                cursor = open + 4;
            }
            (_, Some(close)) => {
                if depth == 0 {
                    return None;
                }
                depth -= 1;
                cursor = close + "</svg>".len();
                if depth == 0 {
                    return Some(input[start..cursor].to_string());
                }
            }
            _ => return None,
        }
    }

    None
}

fn extract_typst_doc_svg(input: &str) -> Option<String> {
    if let Some(app_start) = input.find("<div id=\"typst-app\"") {
        if let Some(svg_rel_start) = input[app_start..].find("<svg") {
            let svg_start = app_start + svg_rel_start;
            if let Some(svg) = extract_balanced_svg(input, svg_start) {
                return Some(svg);
            }
        }
    }

    // Support static Tinymist HTML exports where the document SVG is emitted directly.
    if let Some(svg_start) = input.find("<svg class=\"typst-doc\"") {
        return extract_balanced_svg(input, svg_start);
    }

    None
}

fn tinymist_native_html(sidecar_path: &str, stem: &str) -> Option<String> {
    let raw = fs::read_to_string(sidecar_path).ok()?;
    let resources = extract_first_block(&raw, "<svg id=\"typst-svg-resources\"", "</svg>")
        .or_else(|| extract_first_block(&raw, "<svg class=\"typst-svg-resources\"", "</svg>"))
        .unwrap_or_default();
    let doc_svg = extract_typst_doc_svg(&raw)?;

    Some(format!(
        r##"<div class="blog_post_embed">
  <div class="tinymist-native" id="tinymist-native-{stem}">
    {resources}
    {doc_svg}
  </div>
</div>
<script>
(function() {{
  const root = document.getElementById("tinymist-native-{stem}");
  if (!root) {{
    return;
  }}
  const svg = root.querySelector("svg.typst-doc");
  if (!svg) {{
    return;
  }}

  // Flatten pages so Tinymist output reads like one continuous post.
  const pages = Array.from(svg.querySelectorAll("g.typst-page"));
  let y = 0;
  const pageGap = 10;
  for (const page of pages) {{
    let targetY = y;
    let advance = Number(page.getAttribute("data-page-height")) || 0;
    try {{
      // Measure only page content so pages collapse without huge blank bands.
      const content = page.querySelector("g.typst-group") || page;
      const box = content.getBBox();
      if (box.height > 0) {{
        targetY = y - box.y;
        advance = box.height + pageGap;
      }}
    }} catch (_err) {{
      // Keep data-page-height fallback when geometry isn't measurable.
    }}
    page.setAttribute("transform", `translate(0, ${{targetY}})`);
    y += advance;
  }}

  const width = Number(svg.getAttribute("data-width"))
    || Number((pages[0] && pages[0].getAttribute("data-page-width")) || 596);
  if (width > 0 && y > 0) {{
    svg.setAttribute("viewBox", `0 0 ${{width}} ${{y}}`);
  }}

  svg.removeAttribute("height");
  svg.setAttribute("width", "100%");
  svg.style.height = "auto";
  svg.style.display = "block";
  svg.style.maxWidth = "100%";
  svg.style.background = "transparent";

  for (const outer of svg.querySelectorAll(".typst-page-outer")) {{
    outer.remove();
  }}
  for (const inner of svg.querySelectorAll(".typst-page-inner")) {{
    inner.remove();
  }}
  for (const ind of svg.querySelectorAll(".typst-preview-canvas-page-number, .typst-preview-svg-page-number")) {{
    ind.remove();
  }}

}})();
</script>"##
    ))
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

fn should_skip_blog_post(stem: &str) -> bool {
    stem == "0000_template" || stem.ends_with("_template")
}

fn extract_parenthesized_block(input: &str, marker: &str) -> Option<String> {
    let start = input.find(marker)? + marker.len();
    let mut depth = 1usize;
    let mut in_string = false;
    let mut escaped = false;

    for (idx, ch) in input[start..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(input[start..start + idx].to_string());
                }
            }
            _ => {}
        }
    }

    None
}

fn extract_named_string(args: &str, key: &str) -> Option<String> {
    let marker = format!("{key}:");
    let start = args.find(&marker)? + marker.len();
    let value = args[start..].trim_start();
    if !value.starts_with('"') {
        return None;
    }

    let mut out = String::new();
    let mut escaped = false;
    for ch in value[1..].chars() {
        if escaped {
            out.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '"' {
            return Some(out);
        }
        out.push(ch);
    }

    None
}

fn extract_named_date(args: &str, key: &str) -> Option<String> {
    let marker = format!("{key}:");
    let start = args.find(&marker)? + marker.len();
    let tail = args[start..].trim_start();
    let datetime_block = extract_parenthesized_block(tail, "datetime(")?;

    let mut year: Option<u32> = None;
    let mut month: Option<u32> = None;
    let mut day: Option<u32> = None;

    for part in datetime_block.split(',') {
        let trimmed = part.trim();
        if let Some(value) = trimmed.strip_prefix("year:") {
            year = value.trim().parse::<u32>().ok();
        } else if let Some(value) = trimmed.strip_prefix("month:") {
            month = value.trim().parse::<u32>().ok();
        } else if let Some(value) = trimmed.strip_prefix("day:") {
            day = value.trim().parse::<u32>().ok();
        }
    }

    match (year, month, day) {
        (Some(y), Some(m), Some(d)) => Some(format!("{y:04}-{m:02}-{d:02}")),
        _ => None,
    }
}

fn extract_post_metadata(file_path: &str) -> BlogMetadata {
    let Ok(content) = fs::read_to_string(file_path) else {
        return BlogMetadata::default();
    };
    let Some(post_args) = extract_parenthesized_block(&content, "#post(") else {
        return BlogMetadata::default();
    };

    BlogMetadata {
        title: extract_named_string(&post_args, "title"),
        date: extract_named_date(&post_args, "date"),
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
pub fn get_blog_posts() -> Vec<(String, String, String)> {
    let mut posts = Vec::new();

    if let Ok(entries) = fs::read_dir("output/assets/blog/posts") {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("typ") {
                let stem = file_path.file_stem().unwrap().to_str().unwrap();
                if should_skip_blog_post(stem) {
                    continue;
                }
                let path_str = file_path.to_str().unwrap();
                let meta = extract_post_metadata(path_str);
                let title = meta
                    .title
                    .unwrap_or_else(|| extract_title_from_typst(path_str));
                let date = meta.date.unwrap_or_default();
                posts.push((stem.to_string(), title, date));
            }
        }
    }

    // Sort posts in reverse order (newest first)
    posts.sort_by(|a, b| b.0.cmp(&a.0));
    posts
}
pub fn generate_blog_posts(t: &Translations, suffix: &str, show_home: bool) {
    let paths = fs::read_dir("output/assets/blog/posts").expect("Could not read blog directory");
    let blog_href = format!("/blog{}.html", suffix);

    for entry in paths.flatten() {
        let file_path = entry.path();

        if file_path.extension().and_then(|s| s.to_str()) == Some("typ") {
            let stem = file_path.file_stem().unwrap().to_str().unwrap();
            if should_skip_blog_post(stem) {
                continue;
            }

            // Prefer Tinymist-rendered sidecar HTML when available.
            // Fallback to Typst CLI HTML if sidecar parsing fails.
            let sidecar_path = generate_svg_sidecar(stem, &file_path)
                .or_else(|| tinymist_sidecar_path(stem, &file_path));

            let body = if let Some(sidecar_path) = sidecar_path {
                tinymist_native_html(&sidecar_path, stem)
                    .unwrap_or_else(|| compile_blog_body(file_path.to_str().unwrap()))
            } else {
                compile_blog_body(file_path.to_str().unwrap())
            };

            // Wrap in Maud with breadcrumbs
            let markup = html! {
                main class="main-content blog-post-content" {
                    nav class="breadcrumbs blog-breadcrumbs" {
                        a class="blog-back-link" href=(blog_href.clone()) { (t.blog_back_to_posts) }
                    }
                    article class="blog_post" { (PreEscaped(body)) }
                }
            };
            let blog_post_page = format!("blog/{}", stem);
            let final_html = page(&markup.0, t, &blog_post_page, show_home);

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
                        @for (slug, title, date) in posts {
                            li class="blog-post-item" {
                                a class="blog-post-link" href=(format!("/blog/{}{}.html", slug, suffix)) {
                                    h3 class="blog-post-title" { (title) }
                                    p class="blog-post-date" { (date) }
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
