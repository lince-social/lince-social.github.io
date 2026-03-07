use crate::i18n::Translations;
use maud::{PreEscaped, html};

pub fn page_index(t: &Translations) -> String {
    html! {
        section.hero {
            .hero-container {
                img.hero-logo src="assets/logo/white.svg" alt="Lince Logo";
                h1.hero-title { (t.hero_title) }
                p.hero-tagline { (t.hero_tagline) }
                p.hero-subtitle { (t.hero_subtitle) }

                .hero-buttons {
                    @for btn in &t.hero_buttons {
                        a class=(btn.class) href=(btn.href) { (btn.text) }
                    }
                }
            }
        }

        main.main-content {
            @for block in &t.index_content {
                @if let Some(ref img) = block.image {
                    .content-block.content-block--with-image {
                        .content-block__text {
                            h2.content-block__title { (block.title) }
                            p.content-block__body { (PreEscaped(block.text)) }
                        }
                        .content-block__image {
                            img class=(img.class) src=(img.src) alt=(img.alt);
                        }
                    }
                } @else {
                    .content-block {
                        h2.content-block__title { (block.title) }
                        p.content-block__body { (PreEscaped(block.text)) }
                    }
                }
            }
        }
    }
    .0
}
