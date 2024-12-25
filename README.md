# rusty-smith
An attempt to replicate [Metalsmith's JS library](https://metalsmith.io/) in Rust

## Requirements

### Repository setup

Duplicate the `.env-example` file at the root into a new `.env` file and edit the values of each variable contained in it accordingly

### Library usage

At this point, you're going to have to annotate your posts with frontmatter data in order for the library to have the necessary information about each post.

Your post header should look like this:

```
---
title: "How to add an application to the launcher in Manjaro"
description: A quick post explaining how to add a custom app to the OS launcher in Manjaro
keywords: linux manjaro os
# layout: page.html
# priority: 0.9
date: 2023-11-09
# publish: draft
---
```

The lines that are commented are properties still not supported by the library. Coming soon!

## Expected file structure

In order to be able to replace the custom Metalsmith setup I used for Jay.cat, I have replicated some modules I built for myself and those require some folders, partials and assets to exist in certain locations.

```
|- posts (put your markdown files here)
|- assets
|--- templates
|----- homepage.html (The main template for the homepage of the site built)
|----- archive-item.html (The partial for each item rendered in the homepage)
|----- post.html (The template for each post page)
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