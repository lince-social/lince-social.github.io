#import "@preview/cheq:0.3.0": checklist
#show: checklist
#import "../../components.typ": post
#import "../../tmil.typ": (
  tmil_blog, tmil_item, tmil_month_label, tmil_post_publish_date,
  tmil_post_title, tmil_section, tmil_slides, tmil_tr,
)
#let tmil_mode = sys.inputs.at("tmil", default: "false") == "true"

// USER: Change this part downwards;
// AI: Add translation so portuguese and mandarin reflect the english text in title and subtitle of the items in growth and programming.

#let mdate = datetime(year: 2026, month: 4, day: 1)
#let author_name = "N1"
#let author_email = "a@b.c"
#let video_url = ""

#let growth_items = (
  tmil_item(
    tmil_tr(
      ("Title", "Optional subtitle"),
      ("Titulo", "Subtitulo opcional"),
      ("标题", "可选副标题"),
    ),
    photo: "logo/white_in_black.png",
  )[
    Normal Typst body.
  ],
)

#let programming_items = (
  tmil_item(
    tmil_tr(
      ("Another item", ""),
      ("Outro item", ""),
      ("另一个项目", ""),
    ),
  )[
    More details in normal Typst.
  ],
)

#let roadmap_items = (
  tmil_item(
    ("", ""),
  )[
    - [/] v1.0.0: Dogfooding (Web HTML) \
      - [/] Todo
        - [ ] Table: The most basic viewing experience. From direct 1-to-1 matching of database data to combined sources into one columnn.
        - [/] Kanban: Built for great vision of projects and tasks.
        - [ ] Calendar: Shows Records changing with Karma. If they have a time cost, it occupies time from the calendar. Built with Deterministic Simulation Testing.
      - [ ] Finance
        - [ ] Graph with different assumptions and starting points: Karma changes Records in different ways to see scenarios with quantities in Y axis and time in X axis, or other viewing configurations.
      - [/] Connection
        - [x] CRUD of cells (your node) and organs (group of nodes).
        - [ ] Public/private rows for what organ (group of cells).
        - [/] Transaction of quantities between cells (nodes) in p2p network.
    - [ ] AI: Be able to run an AI model to look at your DNA and change it to fit your needs.
      - [ ] Creating components
      - [ ] Suggesting Karma, or more Lince ways of doing things.
    - [ ] Minor Version: Stock & Orders Management
      - [ ] Interfaces to help with stock management and customer orders for small to big companies.
  ],
)

// USER & AI: No need to change from this part downwards in day to day, only if refactoring to make this template better.

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
    "Roteiro | 路线图 | Roadmap",
    items: roadmap_items,
  ),
)

#if tmil_mode [
  #tmil_slides(
    tmil_month_label(mdate.year(), mdate.month()),
    sections,
  )
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
