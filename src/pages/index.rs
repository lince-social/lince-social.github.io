use crate::i18n::Translations;
use maud::{PreEscaped, html};

fn lang_suffix(lang: &str) -> &str {
    match lang {
        "en" => "",
        "pt-br" => ".pt-br",
        _ => ".zh",
    }
}

pub fn page_index(t: &Translations) -> String {
    let suffix = lang_suffix(t.lang_code);

    let about_href = format!("about{}.html", suffix);
    let hero_buttons: Vec<(String, String, &'static str)> = vec![
        (
            String::from("btn btn-primary"),
            String::from("https://github.com/lince-social/lince/releases"),
            t.hero_download,
        ),
        (
            String::from("btn btn-secondary"),
            String::from("https://github.com/lince-social/lince"),
            t.hero_docs,
        ),
        (
            String::from("btn btn-secondary"),
            about_href.clone(),
            t.hero_learn_more,
        ),
    ];
    let features: Vec<(&'static str, &'static str)> = vec![
        (t.opensource_free_title, t.opensource_free_desc),
        (t.opensource_community_title, t.opensource_community_desc),
        (t.opensource_privacy_title, t.opensource_privacy_desc),
    ];
    let goals: Vec<(&'static str, &'static str)> = vec![
        (t.goal_registry_title, t.goal_registry_desc),
        (t.goal_needs_title, t.goal_needs_desc),
        (t.goal_automation_title, t.goal_automation_desc),
    ];
    let links: Vec<(&'static str, &'static str)> = vec![
        ("https://github.com/lince-social/lince", t.link_source),
        (
            "https://github.com/lince-social/lince/releases",
            t.link_download,
        ),
        (
            "https://github.com/lince-social/lince/blob/main/README.md",
            t.link_readme,
        ),
        (
            "https://github.com/lince-social/lince/issues",
            t.link_issues,
        ),
        (
            "https://github.com/lince-social/lince/discussions",
            t.link_discussions,
        ),
        (
            "https://github.com/lince-social/lince/blob/main/CONTRIBUTING.md",
            t.link_contributing,
        ),
        ("https://github.com/lince-social/website", t.link_website),
        ("https://github.com/lince-social", t.link_org),
    ];

    html! {
        section class="hero" {
            div class="hero-container" {
                img class="hero-logo" src="assets/logo/branco.png" alt="Lince Logo";
                h1 class="hero-title" { (t.hero_title) }
                p class="hero-tagline" { (t.hero_tagline) }
                p class="hero-subtitle" { (t.hero_subtitle) }

                div class="hero-buttons" {
                    @for (cls, href, text) in hero_buttons {
                        a class=(cls) href=(href) { (text) }
                    }
                }
            }
        }

        main class="main-content" {
            section class="section" {
                h2 class="section-title" { (t.opensource_title) }
                p class="section-description" { (t.opensource_desc) }

                div class="features-grid" {
                    @for (title, desc) in features {
                        div class="feature-card" {
                            h3 class="feature-title" { (title) }
                            p class="feature-description" { (PreEscaped(desc)) }
                        }
                    }
                }
            }

            section class="section" {
                h2 class="section-title" { (t.goal_title) }
                p class="section-description" { (t.goal_desc) }

                div class="features-grid" {
                    @for (title, desc) in goals {
                        div class="feature-card" {
                            h3 class="feature-title" { (title) }
                            p class="feature-description" { (PreEscaped(desc)) }
                        }
                    }
                }
            }

            section class="links-section" {
                h3 class="links-title" { (t.links_title) }

                div class="links-grid" {
                    @for (href, text) in links {
                        a class="link-item" href=(href) { (text) }
                    }
                }
            }
        }
    }
    .0
}
