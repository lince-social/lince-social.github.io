# Lince Website

This is the Lince website. It hosts the code for the pages, the blog creation and This Month In Lince generation for slides and blogposts.

# Commands

You can run the application using [mise](https://mise.jdx.dev/) installed with:

```bash
curl https://mise.run | sh
```

The dependencies are installed automatically, once, when running commands. Just type `mise dev` to create the website's files and the TMIL blogposts. For the TMIL slides, use `mise tmil`. To create a new This Month In Lince post, use `mise tmil-new`.

It uses `maud` for templating html and typst for the blogposts, created automaticaly in Rust.
