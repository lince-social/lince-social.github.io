use crate::i18n::Translations;
use maud::{DOCTYPE, PreEscaped, html};

fn lang_suffix(lang: &str) -> &str {
    if lang == "en" {
        ""
    } else if lang == "pt-br" {
        ".pt-br"
    } else {
        ".zh"
    }
}

pub fn page(body: &str, t: &Translations, current_page: &str) -> String {
    let suffix = lang_suffix(t.lang_code);

    // Prepare language suffixes and page links so generated pages point
    // to the actual files produced by `main.rs` (e.g. `index.pt-br.html`).
    let home_href = format!("/index{}.html", suffix);
    let about_href = format!("/about{}.html", suffix);
    let blog_href = format!("/blogposts{}.html", suffix);
    let link_en = format!("/{}{}.html", current_page, "");
    let link_pt = format!("/{}{}.html", current_page, ".pt-br");
    let link_zh = format!("/{}{}.html", current_page, ".zh");

    // Determine asset path based on whether this is a blog post (in subdirectory)
    let asset_prefix = if current_page.starts_with("blog/") {
        "../"
    } else {
        ""
    };
    let assets_css = format!("{}assets/style.css", asset_prefix);
    let assets_logo = format!("{}assets/logo/preto_no_branco.png", asset_prefix);
    let assets_logo_branco = format!("{}assets/logo/branco.png", asset_prefix);
    let assets_datastar = format!("{}assets/datastar.js", asset_prefix);

    html! {
            (DOCTYPE)
            html lang=(t.lang_code) data-theme="dark" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http-equiv="X-UA-Compatible" content="ie=edge";
                meta name="description" content="Lince - Registry, Interconnection, and Automation of Needs and Contributions";
                link rel="icon" href=(assets_logo) type="image/png";
                script {
                    (PreEscaped(r#"(function(){try{const s=localStorage.getItem('theme');if(s)document.documentElement.setAttribute('data-theme',s);else document.documentElement.setAttribute('data-theme','dark');}catch(e){} })();"#))
                }
                link rel="stylesheet" href=(assets_css);
                script type="module" src=(assets_datastar) {}
                title { "Lince" }
            }
            body {
                nav class="navbar" {
                    div class="navbar-container" {
                        a class="navbar-brand" href=(home_href) {
                            img src=(assets_logo_branco) alt="Lince Logo";
                            "Lince"
                        }
                        ul class="navbar-menu" {
                            li {
                                a class=(if current_page == "index" { "navbar-item active" } else { "navbar-item" })
                                    href=(home_href.clone()) {
                                    (t.nav_home)
                                }
                            }
                            li {
                                a class=(if current_page == "about" { "navbar-item active" } else { "navbar-item" })
                                    href=(about_href.clone()) {
                                    (t.nav_about)
                                }
                            }
                            li {
                                a class=(if current_page == "blogposts" || current_page.starts_with("blog/") { "navbar-item active" } else { "navbar-item" })
                                    href=(blog_href.clone()) {
                                    (t.nav_blog)
                                }
                            }
                            li class="desktop-only" {
                                a class="navbar-item" href="https://github.com/lince-social/lince" {
                                    (t.nav_github)
                                }
                            }
                            li class="desktop-only" {
                                a class="navbar-item" href="https://github.com/lince-social/lince/releases" {
                                    (t.nav_download)
                                }
                            }
                            li class="lang-switcher" {
                                button class="lang-btn" onclick="toggleLangDropdown()" {
                                    (t.lang_name)
                                }
                                div class="lang-dropdown" id="langDropdown" {
                                    a class="lang-option" href=(link_en) { "English" }
                                    a class="lang-option" href=(link_pt) { "Português" }
                                    a class="lang-option" href=(link_zh) { "中文" }
                                }
                            }
                            li {
                                button class="theme-toggle" onclick="toggleTheme()" title=(t.nav_theme) {
                                    "◐"
                                }
                            }
                        }
                    }
                }
                (PreEscaped(body))
                footer class="footer" {
                    div class="footer-container" {
                        div class="footer-section" {
                            h4 { "Lince" }
                            ul class="footer-links" {
                                li { a href=(format!("index{}.html", suffix)) { (t.nav_home) } }
                                li { a href=(format!("about{}.html", suffix)) { (t.nav_about) } }
                                li { a href=(format!("blogposts{}.html", suffix)) { (t.nav_blog) } }
                            }
                        }
                        div class="footer-section" {
                            h4 { (t.footer_resources) }
                            ul class="footer-links" {
                                li { a href="https://github.com/lince-social/lince" { (t.footer_documentation) } }
                                li { a href="https://github.com/lince-social/lince/releases" { (t.footer_downloads) } }
                                li { a href="https://github.com/lince-social/lince/issues" { (t.footer_issues) } }
                            }
                        }
                        div class="footer-section" {
                            h4 { (t.footer_community) }
                            ul class="footer-links" {
                                li { a href="https://github.com/lince-social" { (t.footer_org) } }
                                li { a href="https://github.com/lince-social/lince/discussions" { (t.footer_contributing) } }
                            }
                        }
                        div class="footer-section" {
                            h4 { (t.footer_legal) }
                            ul class="footer-links" {
                                li { a href="https://github.com/lince-social/lince/blob/main/LICENSE" { (t.footer_license) } }
                            }
                        }
                    }
                    div class="footer-bottom" {
                        p { (t.footer_copyright) }
                    }
                }
                script {
                    (PreEscaped(r#"
                    function toggleTheme() {
                        const html = document.documentElement;
                        const current = html.getAttribute('data-theme');
                        const next = current === 'dark' ? 'light' : 'dark';
                        html.setAttribute('data-theme', next);
                        localStorage.setItem('theme', next);
                    }
                    (function() {
                        const saved = localStorage.getItem('theme');
                        if (saved) {
                            document.documentElement.setAttribute('data-theme', saved);
                        }
                    })();
                    function toggleLangDropdown() {
                        document.getElementById('langDropdown').classList.toggle('show');
                    }
                    document.addEventListener('click', function(e) {
                        if (!e.target.closest('.lang-switcher')) {
                            document.getElementById('langDropdown').classList.remove('show');
                        }
                    });
                    "#))
                }
            }
        }
    }
    .into_string()
}
