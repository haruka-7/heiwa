# Changelog

## 0.1.14 - yyy/mm/dd

### Bugfix
- Sort articles in RSS feed by date desc (#9)

## 0.1.13 - 2023/11/02

- ADD RSS for tags
- FIX Enable Tera auto-escaping
- OTHER cargo update

## 0.1.12 - 2023-11-02

- Remove Tera auto-escaping

## 0.1.11 - 2023-11-02

- Remove HTML minification

## 0.1.10 - 2023-11-02

- Fix error with mastodon verif link missing on other pages than homepage
- Replace html minify librairie 

## 0.1.9 - 2023-11-02

- Add mastodon_verification_link option to config.toml #34
- Display Heiwa version at startup #30
- Remove the markdown h1 in home.md #23

## 0.1.8 - 2023-10-17

- Add robots.txt route
- Remove unused dependency sitewriter (sitemap)

## 0.1.7 - 2023-10-16

- Change sitemap format from xml to txt
- Fix favicon.ico 404 error
- Dependencies update

## 0.1.6 - 2023-10-13

- Fix sitemap missing attributes to the root url
- Fix sitemap exclude home.md

## 0.1.5 - 2023-10-12

- Fix pages url with subdirectories
- Fix systemd template

## 0.1.4 - 2023-10-06

- Add a path param to the serve command
- Fix sitemap generation for pages whithout date
- Dependencies update
- Add a systemd service example file

## 0.1.3 - 2023-09-29

- Fix pagination "previous" link is displayed even when there is no more pages #9
- Fix heiwa init and heiwa page with special characters in title and description metadatas

## 0.1.2 - 2023-09-16

- Remove pages thumbnails
- Fix sitemap.xml generation
- Crates update

## 0.1.1 - 2023-09-08

- Add command "page" to create a markdown template #2
- Calculate article read duration #5
- Handle TemplateNotFoundError to display an error log message #1

## 0.1.0 - 2023-09-01

- First beta test version
