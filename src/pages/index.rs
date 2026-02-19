use crate::i18n::Translations;
use maud::{PreEscaped, html};

pub fn page_index(t: &Translations) -> String {
    html! {
        section class="hero" {
            div class="hero-container" {
                img class="hero-logo" src="assets/logo/white.svg" alt="Lince Logo";
                h1 class="hero-title" { (t.hero_title) }
                p class="hero-tagline" { (t.hero_tagline) }
                p class="hero-subtitle" { (t.hero_subtitle) }

                div class="hero-buttons" {
                    @for btn in &t.hero_buttons {
                        a class=(btn.class) href=(btn.href) { (btn.text) }
                    }
                }
            }
        }

        main class="main-content" {
            @for block in &t.index_content {
                @if let Some(ref img) = block.image {
                    div class="content-block content-block--with-image" {
                        div class="content-block__text" {
                            h2 class="content-block__title" { (block.title) }
                            p class="content-block__body" { (PreEscaped(block.text)) }
                        }
                        div class="content-block__image" {
                            img class=(img.class) src=(img.src) alt=(img.alt);
                        }
                    }
                } @else {
                    div class="content-block" {
                        h2 class="content-block__title" { (block.title) }
                        p class="content-block__body" { (PreEscaped(block.text)) }
                    }
                }
            }

            // Quick links moved to footer_sections (merged with footer)
        }
    }
    .0
}
