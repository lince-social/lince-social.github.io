#import "components.typ": mline
#import "@preview/touying:0.6.1": *
#import themes.simple: *

#let tmil_tr(en, pt_br, zh_cn) = (en, pt_br, zh_cn)
#let tmil_langs = ("pt-BR", "zh-CN", "es")

#let tmil_month_label(year, month) = {
  str(year) + "-" + if month < 10 { "0" + str(month) } else { str(month) }
}

#let tmil_post_title(
  year,
  month,
  prefix: "This Month in Lince",
) = {
  prefix + " | " + tmil_month_label(year, month)
}

#let tmil_post_date(year, month, day) = {
  datetime(year: year, month: month, day: day)
}

#let tmil_post_publish_date(year, month) = {
  let publish_year = year + if month == 12 { 1 } else { 0 }
  let publish_month = if month == 12 { 1 } else { month + 1 }
  datetime(year: publish_year, month: publish_month, day: 1)
}

#let tmil_text(value, lang: "en") = {
  if type(value) == array and value.len() >= 3 {
    if lang == "pt-BR" {
      value.at(1)
    } else if lang == "zh-CN" {
      value.at(2)
    } else if lang == "es" {
      value.at(0)
    } else {
      value.at(0)
    }
  } else if type(value) == dictionary {
    value.at(lang, default: value.at("en", default: ""))
  } else {
    value
  }
}

#let tmil_item(heading, photo: none, body) = (
  heading: heading,
  photo: photo,
  body: body,
)

#let tmil_photo_path(photo) = {
  if type(photo) == str and not photo.starts-with("media/") {
    "media/" + photo
  } else {
    photo
  }
}

#let tmil_item_title(heading, lang: "en") = {
  let value = tmil_text(heading, lang: lang)
  if type(value) == array and value.len() >= 1 {
    value.at(0)
  } else {
    value
  }
}

#let tmil_item_subtitle(heading, lang: "en") = {
  let value = tmil_text(heading, lang: lang)
  if type(value) == array and value.len() >= 2 {
    value.at(1)
  } else {
    ""
  }
}

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

#let render_item_slide(entry) = {
  let has_heading = tmil_langs.any(lang => {
    let title = tmil_item_title(entry.heading, lang: lang)
    let subtitle = tmil_item_subtitle(entry.heading, lang: lang)
    title != "" or subtitle != ""
  })

  let item_line = lang => {
    let title = tmil_item_title(entry.heading, lang: lang)
    let subtitle = tmil_item_subtitle(entry.heading, lang: lang)
    if subtitle == "" { title } else { title + ": " + subtitle }
  }

  [
    #if entry.photo == none [
      #if has_heading [
        #align(center + horizon)[
          #for lang in tmil_langs [
            #text(size: 24pt, weight: 400)[#item_line(lang)]
            #if lang != tmil_langs.last() [
              #v(0.7em)
            ]
          ]
        ]
      ] else [
        #entry.body
      ]
    ] else [
      #grid(
        columns: (1fr, 1fr),
        gutter: 32pt,
        align(left + horizon)[
          #for lang in tmil_langs [
            #text(size: 24pt, weight: 400)[#item_line(lang)]
            #if lang != tmil_langs.last() [
              #v(0.7em)
            ]
          ]
        ],
        align(center + horizon)[
          #image(tmil_photo_path(entry.photo), width: 92%)
        ],
      )
    ]
  ]
}

#let tmil_slides(
  month_label,
  sections,
  title: tmil_tr(
    "This Month in Lince",
    "Este Mês na Lince",
    "本月在 Lince",
  ),
  closing: tmil_tr(
    "See you next month!",
    "Ate o proximo mes!",
    "下个月见！",
  ),
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

  #let title_line = (
    tmil_langs.map(lang => tmil_text(title, lang: lang)).join(" | ")
  )
  = #image("media/logo/white.svg", width: 18%) \
  #text(size: 27pt, weight: "bold")[#title_line] \
  #text(size: 32pt, weight: 700)[#month_label]


  #for block in sections [
    ==
    #align(center + horizon)[
      #text(size: 34pt, weight: "bold")[
        #for lang in tmil_langs {
          tmil_text(block.name, lang: lang)
          if lang != tmil_langs.last() { " | " }
        }
      ]
    ]

    #for entry in block.items [
      == #for lang in tmil_langs {
        tmil_text(block.name, lang: lang)
        if lang != tmil_langs.last() { " | " }
      }

      #render_item_slide(entry)
    ]
  ]

  =
  #slide[
    #align(center + horizon)[
      #text(size: 34pt, weight: "bold")[
        #for lang in tmil_langs {
          tmil_text(closing, lang: lang)
          if lang != tmil_langs.last() { linebreak() }
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
    "Este Mês na Lince",
    "本月在 Lince",
  ),
  multiline: true,
) = [
  #let join_lines(lines) = {
    if multiline {
      [#for i in range(0, lines.len()) {
        lines.at(i)
        if i != lines.len() - 1 { linebreak() }
      }]
    } else {
      lines.join(" | ")
    }
  }

  #for block in sections [
    #let area_line = (
      tmil_langs.map(lang => tmil_text(block.name, lang: lang)).join(" | ")
    )

    #align(center + horizon)[
      == #text(size: 24pt)[#area_line]
    ]
    #v(3em)


    #for entry in block.items [
      #let lines = tmil_langs.map(lang => {
        let title = tmil_item_title(entry.heading, lang: lang)
        let subtitle = tmil_item_subtitle(entry.heading, lang: lang)
        if subtitle == "" { title } else { title + ": " + subtitle }
      })
      #let has_heading = lines.any(line => line != "")
      #if has_heading [
        === #join_lines(lines)
        #linebreak()
      ]

      #if entry.photo != none [
        #align(center + horizon)[
          #image(tmil_photo_path(entry.photo), width: 72%)
        ]
      ]

      #entry.body

      #mline(2)
    ]
  ]
]
