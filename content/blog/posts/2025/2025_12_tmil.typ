#import "../../components.typ": post
#import "../../tmil.typ": tmil_blog, tmil_item, tmil_section, tmil_slides

#let tmil_mode = sys.inputs.at("tmil", default: "false") == "true"

#let growth_items = (
  tmil_item(
    "Planning",
    subtitle: "Roadmap for the v1.0.0",
    photo: "media/planning/2025_12.jpg",
  )[
    We decided on the roadmap for the v1.0.0.
  ],
  tmil_item(
    "LinceCon",
    subtitle: "2025 Edition. online",
    photo: "media/lincecon/2025/full.jpg",
  )[
    The first LinceCon, we talked about ideas, goals and plans for 2026.
  ],
  tmil_item(
    "This Month in Lince",
    subtitle: "Starting with month 2025-12",
    photo: "media/random/tmil.jpg",
  )[
    Started doing the This Month In Lince monthly videos.
  ],
)

#let programming_items = (
  tmil_item("Everything", subtitle: "Demo Time")[
  ],
)

#let sections = (
  tmil_section("Crescimento | 成长工作 | Growth", items: growth_items),
  tmil_section("Programação | 开发 | Programming", items: programming_items),
)

#if tmil_mode [
  #tmil_slides("2025-12", sections)
] else [
  #post(
    title: "This Month in Lince | 2025-12",
    name: "lince",
    email: "lincesocialnetwork@gmail.com",
    date: datetime(
      year: 2025,
      month: 12,
      day: 1,
    ),
  )[
    #tmil_blog("2025-12", sections)
  ]
]
