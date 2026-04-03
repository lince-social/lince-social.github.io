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
      ("Blog", "Este Mês na Lince + qualquer coisa que venha à mente"),
      ("博客", "本月在 Lince + 任何想到的内容"),
    ),
  )[
    Started the blog, currently only hosting the This Month In Lince Blogpost version. Speaking of which, the TMIL is now fully in Typst, no AI to create it based on an .md file, just used for translation.
  ],
  tmil_item(
    photo: "random/visual_identity_wow.png",
    tmil_tr(
      ("Visual Identity Wooow", "Amazing Wooow"),
      ("Identidade Visual Uaaau", "Incrível Uaaau"),
      ("标题", "可选副标题"),
    ),
  )[
    The visual identity turned into something leagues above what we had before. It got a huge boost, and now is a whole brand, designed with an official colorscheme, typography and text+logo visualizers.
  ],
)

#let programming_items = (
  tmil_item(photo: "random/ecosystem.png", tmil_tr(
    ("Ecosystem Commit", "Maintenance of three interfaces"),
    ("Compromisso do Ecossistema", "Manutenção de três interfaces"),
    ("生态系统承诺", "维护三个前端"),
  ))[
    For the ecosystem, we believe that simultaneously there will be three frontends being maintained at the same time.

    The TUI version is lightweight. A wrapper around the db.

    The web version will be carried by the versatility of HTML. If the user wants a component Lince didnt provide, they will have a marketplace of extensions and ease of access to a development kit of their own components, writing code by hand or with the help of agents.

    The final client will be go beyond what the most twised and pulled HTML can do. That will be Lince's endgame for client-side, probably a desktop GUI, currently using Zed's GPUI. Rendering should have access to the GPU and the feel should be of controlling an alien spaceship if you are an alien with a lot of flight experience.

    The backend should allow for conversation between many clients. If you are using locally one web and one GUI version, they should be able to access the data in the way they can without breaking each other's workflow. If two people using different clients trade Records they will not be stopped by the difference of their interfaces.
  ],
  tmil_item(
    photo: "random/tui_init.png",
    tmil_tr(
      ("TUI with Ratatui", "Basic CRUD done"),
      ("TUI com Ratatui", "CRUD básico concluído"),
      ("使用 Ratatui 的 TUI", "基础 CRUD 已完成"),
    ),
  )[
    The init of the TUI version, counts with basic features. Initially aimed at doing a tutorial, called First Steps. Now an interface that will be built in parallel with the current main one. It is for those that like efficiency and simplicity.
  ],
  tmil_item(
    photo: "random/web_return.png",
    tmil_tr(
      ("HTML's Return", "I am inevitable"),
      ("O Retorno do HTML", "Eu sou inevitável"),
      ("HTML 的回归", "我是不可避免的"),
    ),
  )[
    Before we had an HTML version using Datastar for frontend/backend reactivity. After the start of the development of the GPUI version we deprecated it, for it wasn't in the plan to maintain it.

    But, considering the fact that HTML is really easy to build and join together we decided to keep it to create extensions easily and fit various user's needs. We plan on creating a system of connectors that will deliver data to a Sandbox and allow for the creation of UI with custom components (Views) that receive that data. We want to make this the main interface until it can fit the overwhelmingly majority of user's needs. We want to give them freedom to create components (Sand) for their workflows.

    After we have a lot of web resources, official and community ones, we can decide the best components to be ported to other less generic and easy interfaces; like a GPU accelerated desktop version of Lince for great rendering capabilities, or the terminal for minimalism.
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
