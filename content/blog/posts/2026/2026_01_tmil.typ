#import "@preview/cheq:0.3.0": checklist
#show: checklist
#import "../../components.typ": post
#import "../../tmil.typ": (
  tmil_blog, tmil_item, tmil_month_label, tmil_post_publish_date,
  tmil_post_title, tmil_section, tmil_slides, tmil_tr,
)

#let tmil_mode = sys.inputs.at("tmil", default: "false") == "true"
#let mdate = datetime(year: 2026, month: 1, day: 1)
#let author_name = "duds"
#let author_email = "xaviduds@gmail.com"
#let video_url = "https://youtu.be/1IZjVhAFxSU?si=9FmIe1ppoZzqsJIf"

#let growth_items = (
  tmil_item(
    tmil_tr(
      (
        "Lince Institute in Brazil",
        "Draft of Institute regiment (Estatuto Social) + In touch with help to create it",
      ),
      (
        "Instituto Lince no Brasil",
        "Rascunho do Estatuto Social + Em contato com ajuda para criar",
      ),
      (
        "巴西 Lince 研究所",
        "章程草案（Estatuto Social）+ 正在联系协助创建",
      ),
    ),
    photo: "random/institute_depenency.jpg",
  )[
    The creation of the Lince Institute, a non-profit organization in Brazil, will wait. The cost is high to justify right now. The documents are being prepared and the organization to help create it has been contacted very thoroughly.
  ],
  tmil_item(
    tmil_tr(
      ("Social Media Posts", "Draft, in Typst -> PNG"),
      ("Posts de redes sociais", "Rascunho, em Typst -> PNG"),
      ("社交媒体帖子", "草稿，Typst -> PNG"),
    ),
    photo: "random/social_media_posts.jpg",
  )[
    An experiment was concluded to test if it was possible to create social media posts using Typst. The result was a process of writting something in english, then with an ai translation into portuguese and mandarin all the images where created.

    The design was lacking though, nobody would see it.
  ],
)

#let programming_items = (
  tmil_item(
    tmil_tr(
      ("Playing around, bugging out", "Exploration with manual input"),
      ("Brincando e encontrando bugs", "Exploracao com entrada manual"),
      ("尝试中，发现 bug", "使用手动输入进行探索"),
    ),
  )[
    Created a simple input field to send operations.
  ],
  tmil_item(
    tmil_tr(
      (
        "Playing around, bugging out",
        "Small bugs (Toggle View now updating data)",
      ),
      (
        "Brincando e encontrando bugs",
        "Bugs pequenos (Toggle View agora atualiza os dados)",
      ),
      ("尝试中，发现 bug", "小问题（Toggle View 现在会更新数据）"),
    ),
  )[
    Solved some bugs, namely the View change not updating main content.
  ],
)

#let roadmap_items = (
  tmil_item(
    ("", ""),
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
