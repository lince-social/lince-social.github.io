#import "../../components.typ": post
#import "../../tmil.typ": (
  tmil_blog, tmil_item, tmil_section, tmil_slides, tmil_tr,
)

#let tmil_mode = sys.inputs.at("tmil", default: "false") == "true"

#let growth_items = (
  tmil_item(
    tmil_tr("New Logo", "Novo Logo", "新标志"),
    subtitle: tmil_tr("Made by Nica", "Feito pela Nica", "由 Nica 设计"),
    photo: "media/logo/white_in_black.png",
  )[
  ],
  tmil_item(
    tmil_tr("Website", "Website", "网站"),
    subtitle: tmil_tr("lince.social", "lince.social", "lince.social"),
    photo: "media/random/website1.jpg",
  )[
  ],
  tmil_item(
    tmil_tr("Prototyping", "Prototipagem", "原型设计"),
    subtitle: tmil_tr("Wireframes", "Wireframes", "线框图"),
    photo: "media/planning/wireframing.png",
  )[
  ],
)

#let programming_items = (
  tmil_item(
    tmil_tr("Views", "Views", "视图"),
    subtitle: tmil_tr(
      "Creation Modal with Autocomplete",
      "Modal de criação com autocomplete",
      "带自动补全的创建模态框",
    ),
  )[
  ],
  tmil_item(
    tmil_tr("Table", "Table", "表格"),
    subtitle: tmil_tr(
      "Editable Cell: Modal or not",
      "Célula editável: Modal ou não",
      "可编辑单元格：使用或不使用模态框",
    ),
  )[
  ],
  tmil_item(
    tmil_tr("Vim mode", "Modo Vim", "Vim 模式"),
    subtitle: tmil_tr("Vim mode", "Modo Vim", "Vim 模式"),
  )[
  ],
  tmil_item(
    tmil_tr("Views", "Views", "视图"),
    subtitle: tmil_tr("Pin/Unpin", "Fixar/Desafixar", "固定/取消固定"),
  )[
  ],
  tmil_item(
    tmil_tr("Views", "Views", "视图"),
    subtitle: tmil_tr("Command Buffer", "Command Buffer", "命令缓冲区"),
  )[
  ],
)

#let sections = (
  tmil_section("Crescimento | 成长工作 | Growth", items: growth_items),
  tmil_section("Programação | 开发 | Programming", items: programming_items),
)

#if tmil_mode [
  #tmil_slides("2026-02", sections, langs: ("pt-BR", "zh-CN", "en"))
] else [
  #post(
    title: "This Month in Lince | 2026-02",
    name: "N1",
    email: "a@b.c",
    date: datetime(
      year: 2026,
      month: 2,
      day: 1,
    ),
  )[
    #tmil_blog("2026-02", sections)
  ]
]
