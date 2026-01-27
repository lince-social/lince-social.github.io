use crate::i18n::Translations;
use maud::html;

pub fn page_about(t: &Translations) -> String {
    let sections = vec![
        (&t.about_mission_title, &t.about_mission_text),
        (&t.about_registry_title, &t.about_registry_text),
        (&t.about_needs_title, &t.about_needs_text),
        (&t.about_automation_title, &t.about_automation_text),
        (&t.about_dna_title, &t.about_dna_text),
        (&t.about_economic_title, &t.about_economic_text),
        (&t.about_tech_title, &t.about_tech_text),
        (&t.about_philosophy_title, &t.about_philosophy_text),
    ];
    html! {
        main class="main-content" {
            div class="about-header" {
                h1 class="about-title" { (t.about_title) }
                p class="about-intro" { (t.about_intro) }
            }

            @for (title, text) in &sections {
                div class="about-section" {
                    h2 class="about-section-title" { (title) }
                    p class="about-section-text" { (text) }
                }
            }

            section class="links-section" {
                h3 class="links-title" { (t.about_get_started) }
                div class="links-grid" {
                    a class="link-item" href="https://github.com/lince-social/lince/releases" { (t.about_download) }
                    a class="link-item" href="https://github.com/lince-social/lince" { (t.about_read_docs) }
                    a class="link-item" href="https://github.com/lince-social/lince/blob/main/README.md" { (t.about_quick_start) }
                    a class="link-item" href="https://github.com/lince-social/lince/discussions" { (t.about_join_community) }
                }
            }
        }
    }
    .0
}
