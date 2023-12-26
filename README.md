# Heiwa
A minimalist dynamic flat-file CMS.

> Heiwa is currently in **BETA** state.
> 
> Bugs may occurs and there is still a lot of work to do on optimizing the code.

[![Crates.io Version](https://img.shields.io/crates/v/heiwa?style=for-the-badge&color=green&logo=rust)](https://crates.io/crates/heiwa)
[![Crates.io License](https://img.shields.io/crates/l/heiwa?style=for-the-badge&color=purple)](https://codeberg.org/haruka/heiwa/src/branch/main/LICENCE)

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

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/M4M3R2Z7O)
