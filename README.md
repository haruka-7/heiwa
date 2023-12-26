# Heiwa
A minimalist flat-file CMS, by Pedro CADETE aka Hidrile


> Heiwa is currently in **BETA** state.
> 
> Bugs may occurs and there is still a lot of work to do on optimizing the code.

![Crates.io Version](https://img.shields.io/crates/v/heiwa?color=green&link=https%3A%2F%2Fcrates.io%2Fcrates%2Fheiwa)
![Crates.io License](https://img.shields.io/crates/l/heiwa?color=purple&link=https%3A%2F%2Fcodeberg.org%2Fharuka%2Fheiwa%2Fsrc%2Fbranch%2Fmain%2FLICENCE)

## Description

Heiwa is a minimalist flat-file CMS using Markdown to generate HTML pages. There is no build step, as static site generator, since pages are generated on the fly when accessing them from the browser.

The Rust language, also known for is performance regarding strings and files operations, makes Heiwa very fast, to dynamically render web pages.

## Licence

The Heiwa project is under GPLv3 licence : https://www.gnu.org/licenses/gpl-3.0.en.html

## Installation

Using Rust package manager :

```shell
cargo install heiwa
```

## Documentation

Initialize a new Heiwa website project : 

```shell
heiwa init project_name
```

Add a theme in the `project_name/themes/theme_name` directory. The Heiwa default theme can be found here : [Shizen](https://codeberg.org/haruka/shizen)

Edit the `project_name/config.toml` file accordingly.

Launch the builtin web server : 

```shell
cd project_name
heiwa serve
```

Access your website at `localhost:3000`.

## Contribute

TODO

## Development

TODO
