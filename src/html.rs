use crate::config::INCLUDE_BLOG;
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
    let blog_href = format!("/blog{}.html", suffix);
    let link_en = format!("/{}{}.html", current_page, "");
    let link_pt = format!("/{}{}.html", current_page, ".pt-br");
    let link_zh = format!("/{}{}.html", current_page, ".zh");

    html! {
            (DOCTYPE)
            html lang=(t.lang_code) data-theme="dark" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http-equiv="X-UA-Compatible" content="ie=edge";
                meta name="description" content="Lince - Registry, Interconnection, and Automation of Needs and Contributions";
                link rel="icon" href="/assets/logo/preto_no_branco.png" type="image/png";
                script {
                    (PreEscaped(r#"(function(){try{const s=localStorage.getItem('theme');if(s)document.documentElement.setAttribute('data-theme',s);else document.documentElement.setAttribute('data-theme','dark');}catch(e){} })();"#))
                }
                link rel="stylesheet" href="/assets/style.css";
                title { "Lince" }
            }
            body {
                nav class="navbar" {
                    div class="navbar-container" {
                        a class="navbar-brand" href=(home_href) {
                            img src="/assets/logo/branco.png" alt="Lince Logo";
                            "Lince"
                        }
                        ul class="navbar-menu" {
                            li {
                                a class=(if current_page == "index" { "navbar-item active" } else { "navbar-item" })
                                    href=(home_href.clone()) {
                                    (t.nav_home)
                                }
                            }
                            @if INCLUDE_BLOG {
                                li {
                                    a class=(if current_page.starts_with("blog") || current_page == "blog" { "navbar-item active" } else { "navbar-item" })
                                        href=(blog_href.clone()) {
                                        (t.nav_blog)
                                    }
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
                                    @if t.lang_code == "en" { "EN" }
                                    @else if t.lang_code == "pt-br" { "PT" }
                                    @else { "中文" }
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
                        @for section in &t.footer_sections {
                            div class="footer-section" {
                                h4 { (section.title) }
                                ul class="footer-links" {
                                    @for link in &section.links {
                                        @let href = if link.href.starts_with("http") {
                                            link.href.to_string()
                                        } else if link.href.starts_with("/") {
                                            link.href.to_string()
                                        } else {
                                            format!("/{}", link.href)
                                        };
                                        li { a href=(href) { (link.text) } }
                                    }
                                }
                            }
                        }
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
