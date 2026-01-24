use maud::html;
use crate::i18n::Translations;

pub fn page_about(t: &Translations) -> String {
    html! {
        main class="main-content" {
            div class="about-header" {
                h1 class="about-title" { (t.about_title) }
                p class="about-intro" { (t.about_intro) }
            }
            
            div class="about-section" {
                h2 class="about-section-title" { (t.about_mission_title) }
                p class="about-section-text" { (t.about_mission_text) }
            }
            
            div class="about-section" {
                h2 class="about-section-title" { (t.about_registry_title) }
                p class="about-section-text" { (t.about_registry_text) }
            }
            
            div class="about-section" {
                h2 class="about-section-title" { (t.about_needs_title) }
                p class="about-section-text" { (t.about_needs_text) }
            }
            
            div class="about-section" {
                h2 class="about-section-title" { (t.about_automation_title) }
                p class="about-section-text" { (t.about_automation_text) }
            }
            
            div class="about-section" {
                h2 class="about-section-title" { (t.about_dna_title) }
                p class="about-section-text" { (t.about_dna_text) }
            }
            
            div class="about-section" {
                h2 class="about-section-title" { (t.about_economic_title) }
                p class="about-section-text" { (t.about_economic_text) }
            }
            
            div class="about-section" {
                h2 class="about-section-title" { (t.about_tech_title) }
                p class="about-section-text" { (t.about_tech_text) }
            }
            
            div class="about-section" {
                h2 class="about-section-title" { (t.about_philosophy_title) }
                p class="about-section-text" { (t.about_philosophy_text) }
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
