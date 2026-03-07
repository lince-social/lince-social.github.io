# Lince Website

# Commands

You can run the application using [mise](https://mise.jdx.dev/) installed with:

```bash
curl https://mise.run | sh
```

The dependencies are installed automatically, once, when running commands. Just type `mise dev` to run and `mise clean` to clean the project.

It uses `maud` for templating html and typst for the blogposts, created automaticaly in Rust.

## TMIL Workflow

- TMIL is now Typst-only.
- TMIL and blog posts are unified in `content/blog/posts`.
- Template: `content/blog/posts/0000_tmil_template.typ`.
- Create a file like `content/blog/posts/YYYY/YYYY_MM_tmil.typ`.
- Write one Typst file for both outputs:
  - default mode: blog post (continuous, no slide page breaks)
  - `--input tmil=true`: slide mode
- Translations are optional:
  - English-only: plain strings (or `langs: ("en",)` in slides)
  - PT/ZH/EN: use `tmil_tr("EN", "PT", "ZH")` and `langs: ("pt-BR", "zh-CN", "en")`
- Run `mise tmil` to preview the latest `YYYY/YYYY_MM_tmil.typ` in slide mode.
