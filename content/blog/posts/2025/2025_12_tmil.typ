#import "../../components.typ": post
#import "../../tmil.typ": (
  tmil_blog, tmil_item, tmil_month_label, tmil_post_date, tmil_post_title,
  tmil_section, tmil_slides, tmil_tr,
)

#let tmil_mode = sys.inputs.at("tmil", default: "false") == "true"
#let mdate = datetime(year: 2025, month: 12, day: 1)
#let author_name = "duds"
#let author_email = "xaviduds@gmail.com"

#let growth_items = (
  tmil_item(
    tmil_tr("Planning", "Planejamento", "规划"),
    subtitle: tmil_tr(
      "Roadmap for the v1.0.0",
      "Roteiro para a v1.0.0",
      "v1.0.0 路线图",
    ),
    photo: "media/planning/2025_12.jpg",
  )[
    We decided on the roadmap for the v1.0.0.
  ],
  tmil_item(
    tmil_tr("LinceCon", "LinceCon", "LinceCon"),
    subtitle: tmil_tr(
      "2025 Edition. online",
      "Edicao 2025. online",
      "2025 版，线上",
    ),
    photo: "media/lincecon/2025/full.jpg",
  )[
    The first LinceCon, we talked about ideas, goals and plans for 2026.
  ],
  tmil_item(
    tmil_tr("This Month in Lince", "Este Mes na Lince", "本月在林斯"),
    subtitle: tmil_tr(
      "Starting with month 2025-12",
      "Comecando no mes 2025-12",
      "从 2025-12 月开始",
    ),
    photo: "media/random/tmil.jpg",
  )[
    Started doing the This Month In Lince monthly videos.
  ],
)

#let programming_items = (
  tmil_item(
    tmil_tr("Everything", "Tudo", "全部内容"),
    subtitle: tmil_tr("Demo Time", "Hora da Demo", "演示时间"),
  )[
    Shortest demo, the application appears in video. Showcased the HTML (Datastar) deprecated version and the GPUI next version.
  ],
)

#let sections = (
  tmil_section("Crescimento | 成长工作 | Growth", items: growth_items),
  tmil_section("Programação | 开发 | Programming", items: programming_items),
)

#if tmil_mode [
  #tmil_slides(tmil_month_label(mdate.year(), mdate.month()), sections)
] else [
  #post(
    title: tmil_post_title(mdate.year(), mdate.month()),
    name: author_name,
    email: author_email,
    date: tmil_post_date(mdate.year(), mdate.month(), mdate.day()),
  )[
    #tmil_blog(tmil_month_label(mdate.year(), mdate.month()), sections)
  ]
]
