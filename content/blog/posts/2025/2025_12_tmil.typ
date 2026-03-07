#import "@preview/cheq:0.3.0": checklist
#show: checklist
#import "../../components.typ": post
#import "../../tmil.typ": (
  tmil_blog, tmil_item, tmil_month_label, tmil_post_publish_date,
  tmil_post_title, tmil_section, tmil_slides, tmil_tr,
)

#let tmil_mode = sys.inputs.at("tmil", default: "false") == "true"
#let mdate = datetime(year: 2025, month: 12, day: 1)
#let author_name = "duds"
#let author_email = "xaviduds@gmail.com"
#let video_url = "https://youtu.be/bAHJzsV3_GU?si=E2eJPUkoJbOHmuuY"

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
    tmil_tr("This Month in Lince", "Este Mês na Lince", "本月在 Lince"),
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

#let roadmap_items = (
  tmil_item(
    "",
  )[
    - [/] v1.0.0: Todo \
      Rewrite of Frontend in GPUI
      - [/] Todo
        - [/] Table
        - [ ] Kanban
        - [ ] Calendar
          - [ ] Shows Records changing with Karma. If they have a time cost, it occupies time from the calendar.
      - [ ] Finance
        - [ ] Table
        - [ ] Graph
        - [ ] Calendar
      - [ ] Connection
        - [ ] CRUD of cells (your node) and organs (group of nodes).
        - [ ] Public/private rows for what organ (group of cells).
        - [ ] Transaction of quantities between cells (nodes) in p2p network.
    - [ ] v1.1.0: AI
      - [ ] Be able to run an AI model to look at your DNA and change it to fit your needs.
    - [ ] v1.2.0: Stock
      - [ ] Screens to help with stock management for small to big companies.
  ],
)

#let sections = (
  tmil_section("Crescimento | 成长工作 | Growth", items: growth_items),
  tmil_section("Programação | 开发 | Programming", items: programming_items),
  tmil_section("Roteiro | 路线图 | Roadmap", items: roadmap_items),
)

#if tmil_mode [
  #tmil_slides(tmil_month_label(mdate.year(), mdate.month()), sections)
] else [
  #post(
    title: tmil_post_title(mdate.year(), mdate.month()),
    name: author_name,
    email: author_email,
    date: tmil_post_publish_date(mdate.year(), mdate.month()),
  )[
    #tmil_blog(tmil_month_label(mdate.year(), mdate.month()), sections)
  ]
]
