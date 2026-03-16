#import "@preview/cheq:0.3.0": checklist
#show: checklist
#import "../../components.typ": post
#import "../../tmil.typ": (
  tmil_blog, tmil_item, tmil_month_label, tmil_post_publish_date,
  tmil_post_title, tmil_section, tmil_slides, tmil_tr,
)
#let tmil_mode = sys.inputs.at("tmil", default: "false") == "true"

#let mdate = datetime(year: 2026, month: 3, day: 1)
#let author_name = "duds"
#let author_email = "xaviduds@gmail.com"
#let video_url = ""

#let growth_items = (
  tmil_item(
    photo: "random/blog_init.png",
    tmil_tr(
      ("Blog", "This Month In Lince + anything that comes to mind"),
      ("Titulo", "Subtitulo opcional"),
      ("标题", "可选副标题"),
    ),
  )[
    Started the blog, currently only hosting the This Month In Lince Blogpost version. Speaking of which, the TMIL is now fully in Typst, no AI to create it based on an .md file, just used for translation.
  ],
)

#let programming_items = (
  tmil_item(
    photo: "random/tui_init.png",
    tmil_tr(
      ("TUI with Ratatui", "Basic CRUD done"),
      ("Outro item", ""),
      ("另一个项目", ""),
    ),
  )[
  ],
  tmil_item(
    photo: "random/web_return.png",
    tmil_tr(
      ("Web's Return", "I am inevitable"),
      ("Outro item", ""),
      ("另一个项目", ""),
    ),
  )[
    Before we had an HTML version using Datastar for frontend/backend reactivity. After the start of the development of the GPUI version we deprecated it, for it wasn't in the plan to maintain it.

    But, considering the fact that HTML is really easy to build and join together we decided to keep it to create extensions easily and fit various user's needs. We plan on creating a system of connectors that will deliver data and allow for the creation of UI with user or AI built Views that receive that data.
  ],
)

#let roadmap_items = (
  tmil_item(
    ("", ""),
  )[
    - [/] v1.0.0: Dogfooding \
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
