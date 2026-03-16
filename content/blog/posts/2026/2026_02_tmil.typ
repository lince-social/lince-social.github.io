#import "@preview/cheq:0.3.0": checklist
#show: checklist
#import "../../components.typ": post
#import "../../tmil.typ": (
  tmil_blog, tmil_item, tmil_month_label, tmil_post_publish_date,
  tmil_post_title, tmil_section, tmil_slides, tmil_tr,
)

#let tmil_mode = sys.inputs.at("tmil", default: "false") == "true"
#let mdate = datetime(year: 2026, month: 2, day: 1)
#let author_name = "duds"
#let author_email = "xaviduds@gmail.com"
#let video_url = "https://youtu.be/iH4L37GrPBE?si=yeZowC8tK3VeoS-L"

#let growth_items = (
  tmil_item(
    tmil_tr(
      ("New Logo", "Made by Nica"),
      ("Novo Logo", "Feito pela Nica"),
      ("新标志", "由 Nica 设计"),
    ),
    photo: "logo/white_in_black.png",
  )[
    Look at it!
  ],
  tmil_item(
    tmil_tr(
      ("Website", "lince.social"),
      ("Website", "lince.social"),
      ("网站", "lince.social"),
    ),
    photo: "random/website1.jpg",
  )[
    We started this website. Simple, central point for buncha links with some description.
  ],
  tmil_item(
    tmil_tr(
      ("Prototyping", "Wireframes"),
      ("Prototipagem", "Wireframes"),
      ("原型设计", "线框图"),
    ),
    photo: "planning/wireframing.png",
  )[
    Spent some time wireframing, we externalized some ideas before coding. Mainly around Tables, some for the P2P connection between nodes.
  ],
)

#let programming_items = (
  tmil_item(
    tmil_tr(
      ("Views", "Creation Modal with Autocomplete"),
      ("Views", "Modal de criação com autocomplete"),
      ("视图", "带自动补全的创建模态框"),
    ),
  )[
    When I'm creating data in any table, that component has autocomplete. It can be a modal or a View in my content area.
  ],
  tmil_item(
    tmil_tr(
      ("Table", "Editable Cell: Modal or not"),
      ("Table", "Célula editável: Modal ou não"),
      ("表格", "可编辑单元格：使用或不使用模态框"),
    ),
  )[
    Tables can have editable cells that can be edited either in a modal throgh an icon, or directly in the table.
  ],
  tmil_item(
    tmil_tr(
      ("Vim mode", "Vim mode"),
      ("Modo Vim", "Modo Vim"),
      ("Vim 模式", "Vim 模式"),
    ),
  )[
    For the Operation input and the Editable Cells the user can set Vim mode through the Configuration.
  ],
  tmil_item(
    tmil_tr(
      ("Views", "Pin/Unpin"),
      ("Views", "Fixar/Desafixar"),
      ("视图", "固定/取消固定"),
    ),
  )[
    The user is able to pin or unpin views, keeping them fixed at a position on the screen on top of other components; independently of the active Collection.
  ],
  tmil_item(
    tmil_tr(
      ("Views", "Command Buffer"),
      ("Views", "Command Buffer"),
      ("视图", "命令缓冲区"),
    ),
  )[
    Then running Commands, one can see their output/errors and be able to interact with them (mpv).
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
