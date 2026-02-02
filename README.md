# rusty-smith
An attempt to replicate [Metalsmith's JS library](https://metalsmith.io/) in Rust

## Usage

### Build the site

```bash
cargo run
```

This generates the static site in the `build/` directory.

### Create a new post

```bash
cargo run -- new
```

This creates a new post file in `posts/` with today's date and a template structure:
- Filename: `YYYY-MM-DD-post-title.md`
- Pre-filled frontmatter with date, title, description, keywords
- Basic section structure to get started

## Requirements

### Repository setup

Duplicate the `.env-example` file at the root into a new `.env` file and edit the values of each variable contained in it accordingly

### Library usage

At this point, you're going to have to annotate your posts with frontmatter data in order for the library to have the necessary information about each post.

Your post header should look like this:

```yaml
---
title: "How to add an application to the launcher in Manjaro"
description: A quick post explaining how to add a custom app to the OS launcher in Manjaro
keywords: linux manjaro os
date: 2023-11-09
permalink: custom-url-slug  # optional, overrides auto-generated URL
publish: draft              # optional, set to "draft" to exclude from build
lightTheme: true            # optional, adds "light-theme" CSS class to body
---
```

### Frontmatter fields

| Field | Required | Description |
|-------|----------|-------------|
| `title` | Yes | Post title |
| `date` | Yes | Publication date (YYYY-MM-DD) |
| `description` | No | Short description for meta tags and archive |
| `keywords` | No | Comma-separated keywords for meta tags |
| `permalink` | No | Custom URL slug (defaults to lowercase title with spaces as dashes) |
| `publish` | No | Set to `draft` to exclude from build |
| `lightTheme` | No | Set to `true` to add `light-theme` CSS class |

## Expected file structure

In order to be able to replace the custom Metalsmith setup I used for Jay.cat, I have replicated some modules I built for myself and those require some folders, partials and assets to exist in certain locations.

```
|- posts (put your markdown files here)
|- assets
|--- templates
|----- homepage.html (The main template for the homepage of the site built)
|----- archive-item.html (The partial for each item rendered in the homepage)
|----- post.html (The template for each post page)
|----- feed.xml (The template for the Atom RSS feed)
|----- feed-entry.xml (The partial for each item in the Atom RSS feed)
|- public
|--- css
|----- styles.css (The main stylesheet containing ALL styles)
```

### Homepage template

This template has a `{post_items}` placeholder that will be used to hold all the items rendered in the homepage. Replace this with the markup of the list of post links.

### Archive item partial

This partial has the following placeholders:

+ POST_ITEM_LINK_PLACEHOLDER (`{post_link}`):
+ POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER (`{post_date_timestamp}`):
+ POST_ITEM_DATE_READABLE_PLACEHOLDER (`{post_date_human_readable}`):
+ POST_ITEM_TITLE_PLACEHOLDER (`{post_title}`):
+ POST_ITEM_EXCERPT_PLACEHOLDER (`{post_excerpt}`):

### Post page template

This template has the following placeholders:

+ HOST_PLACEHOLDER (`{host}`)
+ POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER (`{post_date_timestamp}`)
+ POST_ITEM_DATE_READABLE_PLACEHOLDER (`{post_date_human_readable}`)
+ POST_ITEM_TITLE_PLACEHOLDER (`{post_title}`)
+ POST_ITEM_DESCRIPTION_PLACEHOLDER (`{post_description}`)
+ POST_ITEM_CONTENT_PLACEHOLDER (`{post_content}`)
+ POST_ITEM_URL_PLACEHOLDER (`{post_url}`)
+ POST_ITEM_KEYWORDS_PLACEHOLDER (`{post_keywords}`)
+ POST_ITEM_IMAGE_URL_PLACEHOLDER (`{post_image_url}`)