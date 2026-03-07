#import "../../components.typ": post
#import "../../tmil.typ": (
  tmil_blog, tmil_item, tmil_month_label, tmil_post_date, tmil_post_title,
  tmil_section, tmil_slides, tmil_tr,
)

#let tmil_mode = sys.inputs.at("tmil", default: "false") == "true"

#let growth_items = (
  tmil_item(
    tmil_tr(
      "Blog",
      "Titulo",
      "标题",
    ),
    subtitle: tmil_tr(
      "This Month In Lince + anything that comes to mind",
      "Subtitulo opcional",
      "可选副标题",
    ),
    photo: "media/random/blog_init.png",
  )[
    Started the blog, currently only hosting the This Month In Lince Blogpost version. Speaking of which, the TMIL is now fully in Typst, no AI to create it based on an .md file, just used for translation.
  ],
)

#let programming_items = (
  tmil_item(
    tmil_tr("Another item", "Outro item", "另一个项目"),
  )[
    More details in normal Typst.
  ],
)

#let birth_items = (
  tmil_item(
    tmil_tr("Birth item", "Item de nascimento", "诞生项目"),
  )[
  ],
)

#let aging_items = (
  tmil_item(
    tmil_tr("Aging item", "Item de envelhecimento", "衰老项目"),
  )[
  ],
)

#let sickness_items = (
  tmil_item(
    tmil_tr("Sickness item", "Item de doenca", "疾病项目"),
  )[
  ],
)

#let death_items = (
  tmil_item(
    tmil_tr("Death item", "Item de morte", "死亡项目"),
  )[
  ],
)

#let sections = (
  tmil_section(
    "Crescimento | 成长工作 | Growth",
    items: growth_items,
  ),
  tmil_section(
    "Programação | 开发 | Programming",
    items: programming_items,
  ),
  tmil_section(
    "Nascimento | 诞生 | Birth",
    items: birth_items,
  ),
  tmil_section(
    "Envelhecimento | 衰老 | Aging",
    items: aging_items,
  ),
  tmil_section(
    "Doença | 疾病 | Sickness",
    items: sickness_items,
  ),
  tmil_section(
    "Morte | 死亡 | Death",
    items: death_items,
  ),
)

#if tmil_mode [
  #tmil_slides(
    tmil_month_label(2026, 3),
    sections,
  )
] else [
  #post(
    title: tmil_post_title(2026, 3),
    name: "duds",
    email: "xaviduds@gmail.com",
    date: tmil_post_date(2026, 3, 1),
  )[
    #tmil_blog(tmil_month_label(2026, 3), sections)
  ]
]
