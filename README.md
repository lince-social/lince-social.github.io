# Lince Website

This is the Lince website, meant to not have a build step, when the code is deployed, it is just HTML being hosted. The evolution of the website happens locally at development, when it is ready, the commit will be made and the push with GitHub Actions should refresh the github.io website.

This repo hosts the code for the pages, the blog creation and TMIL (This Month In Lince) generation for slides and blogposts. It uses the `maud` crate for templating HTML and Typst for the blogposts, created automaticaly in Rust.

# Commands

You can best run the application using [mise](https://mise.jdx.dev/) installed with:

```bash
curl https://mise.run | sh
```

The dependencies are installed automatically, once, when running commands. Just type `mise dev` to do the following:

- Create the blogposts from Typst including TMIL blogpost version. If there are missing months of TMIL from this month backwards (up to one year) they will be generated; the Typst for them, their equivalent html blogpost (svg based) and maintain the two most recent months of TMIL with touying (slides) to not fill up with .gitignored generated HTML.
- Create the website's HTML.
- Runs the Rust's tests that assure the website works correctly.
- Serves the website locally at `http://localhost:46785`

This way whenever a new month passes, the TMIL blogpost will be automatically generated and the website will be updated. All one needs to do is run `mise dev`, edit the Typst (the website will update in real time), and commit/push the changes to update the website. If one wants to present the TMIL they can open the HTML (.gitignored) next to the real TMIL Typst in the browser and it is good to go.
