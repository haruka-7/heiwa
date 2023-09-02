# Heiwa

By Pedro CADETE aka Hidrile



> Heiwa is currently in **BETA** state.
> 
> Bugs may occurs and there is still a lot of work to do on optimizing the code.



## Description

Heiwa is a minimalist flat-file CMS using Markdown to generate HTML pages. There is no build step, as static site generator, since pages are generated on the fly when accessing them from the browser.

The Rust language, also known for is performance regarding strings and files operations, makes Heiwa very fast, to dynamically render web pages.

## Licence

The Heiwa project is under GPLv3 licence : https://www.gnu.org/licenses/gpl-3.0.en.html

## Installation

```shell
cargo install heiwa
```

## Documentation

Initialize a new Heiwa website project : 

```shell
cargo init project_name
```

Add a theme in the `project_name/themes` directory. The Heiwa default theme can be found here : [Shizen](https://git.sr.ht/~hidrile/shizen)

Edit the `project_name/config.toml` file accordingly.

Launch the builtin web server : 

```shell
cd project_name
cargo serve
```

Access your website at `localhost:3000`.

## Contribute

TODO

## Development

TODO
