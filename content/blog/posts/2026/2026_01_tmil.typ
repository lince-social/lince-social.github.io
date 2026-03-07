#import "../../components.typ": post
#import "../../tmil.typ": tmil_blog, tmil_item, tmil_section, tmil_slides

#let tmil_mode = sys.inputs.at("tmil", default: "false") == "true"

#let growth_items = (
  tmil_item(
    "Lince Institute in Brazil",
    subtitle: "Draft of Institute regiment (Estatuto Social) + In touch with help to create it",
    photo: "media/random/institute_depenency.jpg",
  )[
  ],
  tmil_item(
    "Social Media Posts",
    subtitle: "Draft, in Typst -> PNG",
    photo: "media/random/social_media_posts.jpg",
  )[
  ],
)

#let programming_items = (
  tmil_item(
    "Playing around, bugging out",
    subtitle: "Exploration with manual input",
  )[
  ],
  tmil_item(
    "Playing around, bugging out",
    subtitle: "Small bugs (Toggle View now updating data)",
  )[
  ],
)

#let sections = (
  tmil_section("Crescimento | 成长工作 | Growth", items: growth_items),
  tmil_section("Programação | 开发 | Programming", items: programming_items),
)

#if tmil_mode [
  #tmil_slides("2026-01", sections)
] else [
  #post(
    title: "This Month in Lince | 2026-01",
    name: "N1",
    email: "a@b.c",
    date: datetime(
      year: 2026,
      month: 1,
      day: 1,
    ),
  )[
    #tmil_blog("2026-01", sections)
  ]
]
