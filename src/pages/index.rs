use crate::i18n::Translations;
use maud::html;

fn lang_suffix(lang: &str) -> &str {
    match lang {
        "en" => "",
        "pt-br" => ".pt-br",
        _ => ".zh",
    }
}

pub fn page_index(t: &Translations) -> String {
    let suffix = lang_suffix(t.lang_code);

    html! {
        section class="hero" {
            div class="hero-container" {
                img class="hero-logo" src="assets/logo/branco.png" alt="Lince Logo";
                h1 class="hero-title" { (t.hero_title) }
                p class="hero-tagline" { (t.hero_tagline) }
                p class="hero-subtitle" { (t.hero_subtitle) }
                div class="hero-buttons" {
                    a class="btn btn-primary" href="https://github.com/lince-social/lince/releases" {
                        (t.hero_download)
                    }
                    a class="btn btn-secondary" href="https://github.com/lince-social/lince" {
                        (t.hero_docs)
                    }
                    a class="btn btn-secondary" href=(format!("about{}.html", suffix)) {
                        (t.hero_learn_more)
                    }
                }
            }
        }

        main class="main-content" {
            section class="section" {
                h2 class="section-title" { (t.opensource_title) }
                p class="section-description" { (t.opensource_desc) }
                div class="features-grid" {
                    div class="feature-card" {
                        h3 class="feature-title" { (t.opensource_free_title) }
                        p class="feature-description" { (t.opensource_free_desc) }
                    }
                    div class="feature-card" {
                        h3 class="feature-title" { (t.opensource_community_title) }
                        p class="feature-description" { (t.opensource_community_desc) }
                    }
                    div class="feature-card" {
                        h3 class="feature-title" { (t.opensource_privacy_title) }
                        p class="feature-description" { (t.opensource_privacy_desc) }
                    }
                }
            }

            section class="section" {
                h2 class="section-title" { (t.goal_title) }
                p class="section-description" { (t.goal_desc) }
                div class="features-grid" {
                    div class="feature-card" {
                        h3 class="feature-title" { (t.goal_registry_title) }
                        p class="feature-description" { (t.goal_registry_desc) }
                    }
                    div class="feature-card" {
                        h3 class="feature-title" { (t.goal_needs_title) }
                        p class="feature-description" { (t.goal_needs_desc) }
                    }
                    div class="feature-card" {
                        h3 class="feature-title" { (t.goal_automation_title) }
                        p class="feature-description" { (t.goal_automation_desc) }
                    }
                }
            }

            section class="links-section" {
                h3 class="links-title" { (t.links_title) }
                div class="links-grid" {
                    a class="link-item" href="https://github.com/lince-social/lince" { (t.link_source) }
                    a class="link-item" href="https://github.com/lince-social/lince/releases" { (t.link_download) }
                    a class="link-item" href="https://github.com/lince-social/lince/blob/main/README.md" { (t.link_readme) }
                    a class="link-item" href="https://github.com/lince-social/lince/issues" { (t.link_issues) }
                    a class="link-item" href="https://github.com/lince-social/lince/discussions" { (t.link_discussions) }
                    a class="link-item" href="https://github.com/lince-social/lince/blob/main/CONTRIBUTING.md" { (t.link_contributing) }
                    a class="link-item" href="https://github.com/lince-social/website" { (t.link_website) }
                    a class="link-item" href="https://github.com/lince-social" { (t.link_org) }
                }
            }
        }
    }.0
}
