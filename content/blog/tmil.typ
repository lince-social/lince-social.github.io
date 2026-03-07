#import "@preview/touying:0.6.1": *
#import themes.simple: *

#let tmil_tr(en, pt_br, zh_cn) = (en, pt_br, zh_cn)

#let tmil_text(value, lang: "en") = {
  if type(value) == array and value.len() >= 3 {
    if lang == "pt-BR" {
      value.at(1)
    } else if lang == "zh-CN" {
      value.at(2)
    } else {
      value.at(0)
    }
  } else if type(value) == dictionary {
    value.at(lang, default: value.at("en", default: ""))
  } else {
    value
  }
}

#let tmil_item(title, subtitle: "", photo: none, body) = (
  title: title,
  subtitle: subtitle,
  photo: photo,
  body: body,
)

#let tmil_section_name(name) = {
  if type(name) != str {
    panic(
      "tmil_section(name, ...): name must be a string in 'pt-BR | zh-CN | en' format",
    )
  }

  let parts = name.split("|")
  if parts.len() != 3 {
    panic("tmil_section(name, ...): use exactly 'pt-BR | zh-CN | en'")
  }

  tmil_tr(parts.at(2).trim(), parts.at(0).trim(), parts.at(1).trim())
}

#let tmil_section(name, items: ()) = (
  name: tmil_section_name(name),
  items: items,
)

#let render_item_slide(entry, langs: ("en",)) = {
  let item_line = lang => {
    let title = tmil_text(entry.title, lang: lang)
    let subtitle = tmil_text(entry.subtitle, lang: lang)
    if subtitle == "" { title } else { title + ": " + subtitle }
  }

  [
    #align(center + horizon)[
      #if entry.photo == none [
        #for lang in langs [
          #text(size: 24pt, weight: 400)[#item_line(lang)]
          #if lang != langs.last() [
            #v(0.7em)
          ]
        ]
        #v(0.7em)
        #entry.body
      ] else [
        #grid(
          columns: (1fr, 1fr),
          gutter: 32pt,
          align(left + horizon)[
            #for lang in langs [
              #text(size: 24pt, weight: 400)[#item_line(lang)]
              #if lang != langs.last() [
                #v(0.7em)
              ]
            ]
            #v(0.7em)
            #entry.body
          ],
          align(center + horizon)[
            #image(entry.photo, width: 92%)
          ],
        )
      ]
    ]
  ]
}

#let tmil_slides(
  month_label,
  sections,
  title: tmil_tr(
    "This Month in Lince",
    "Este Mes na Lince",
    "本月在林斯",
  ),
  closing: tmil_tr(
    "See you next month!",
    "Ate o proximo mes!",
    "下个月见！",
  ),
  langs: ("en",),
) = [
  #show: simple-theme.with(aspect-ratio: "16-9")

  #let dark = true
  #let bg = if dark { rgb(20, 20, 20) } else { white }
  #let fg = if dark { white } else { black }

  #set page(fill: bg)
  #set text(fill: fg)
  #set text(
    lang: "en",
    font: "New Computer Modern Math",
    size: 20pt,
  )
  #set heading(numbering: none)

  = #for lang in langs {
    tmil_text(title, lang: lang)
    if lang != langs.last() { " | " }
  } | #month_label

  #for block in sections [
    ==
    #align(center + horizon)[
      #text(size: 34pt, weight: "bold")[
        #for lang in langs {
          tmil_text(block.name, lang: lang)
          if lang != langs.last() { " | " }
        }
      ]
    ]

    #for entry in block.items [
      == #for lang in langs {
        tmil_text(block.name, lang: lang)
        if lang != langs.last() { " | " }
      }

      #render_item_slide(entry, langs: langs)
    ]
  ]

  =
  #slide[
    #align(center + horizon)[
      #text(size: 34pt, weight: "bold")[
        #for lang in langs {
          tmil_text(closing, lang: lang)
          if lang != langs.last() { linebreak() }
        }
      ]
    ]
  ]
]

#let tmil_blog(
  month_label,
  sections,
  title: tmil_tr(
    "This Month in Lince",
    "Este Mes na Lince",
    "本月在林斯",
  ),
  lang: "en",
) = [
  #for block in sections [
    == #tmil_text(block.name, lang: lang)

    #for entry in block.items [
      #let title = tmil_text(entry.title, lang: lang)
      #let subtitle = tmil_text(entry.subtitle, lang: lang)
      === #if subtitle == "" { title } else { title + ": " + subtitle }

      #if entry.photo != none [
        #image(entry.photo, width: 100%)
      ]

      #entry.body
      #v(0.8em)
    ]
  ]
]
