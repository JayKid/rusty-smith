# rusty-smith

A static site generator written in Rust, inspired by [Metalsmith](https://metalsmith.io/).

## Features

- **Markdown to HTML** conversion with YAML frontmatter support
- **Blog posts** with automatic permalink generation
- **Static pages** (about, contact, etc.) from markdown files
- **Client-side search** with embedded JSON index
- **RSS/Atom feed** generation
- **Sitemap** generation
- **Draft posts** support (excluded from build)
- **Light/dark theme** support via CSS classes
- **Custom permalinks** override auto-generated URLs

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

## Setup

### 1. Environment configuration

Copy `.env.example` to `.env` and configure your site settings:

```bash
cp .env.example .env
```

Required environment variables:

| Variable | Description | Example |
|----------|-------------|---------|
| `HOST` | Site URL (no trailing slash) | `https://example.com` |
| `WEBSITE_NAME` | Site name for navigation/titles | `My Site` |
| `AUTHOR_NAME` | Author name for meta tags | `Your Name` |
| `WEBSITE_LOGO_URL` | Logo URL for Open Graph/Twitter cards | `https://example.com/img/logo.png` |
| `WEBSITE_DESCRIPTION` | Site description for meta tags | `A description of your site` |
| `TWITTER_HANDLE` | Twitter handle (include @) | `@yourusername` |

### 2. Directory structure

```
├── posts/                    # Blog posts (markdown files)
├── pages/                    # Static pages (markdown files)
├── assets/
│   └── templates/
│       ├── homepage.html     # Homepage template
│       ├── archive-item.html # Post list item partial
│       ├── post.html         # Individual post template
│       ├── page.html         # Static page template
│       ├── search.html       # Search page template
│       ├── feed.xml          # RSS feed template
│       └── feed-entry.xml    # RSS entry partial
├── public/                   # Static assets (copied to build/)
│   ├── css/
│   ├── img/
│   └── js/
└── build/                    # Generated output (git-ignored)
```

## Content

### Blog posts

Create markdown files in `posts/` with YAML frontmatter:

```yaml
---
title: "My Post Title"
description: A short description for meta tags and archive
keywords: comma, separated, keywords
date: 2024-01-15
permalink: custom-url-slug  # optional
publish: draft              # optional, excludes from build
lightTheme: true            # optional, adds CSS class
---

Your markdown content here...
```

#### Frontmatter fields

| Field | Required | Description |
|-------|----------|-------------|
| `title` | Yes | Post title |
| `date` | Yes | Publication date (YYYY-MM-DD) |
| `description` | No | Short description for meta tags and archive listing |
| `keywords` | No | Comma-separated keywords for meta tags |
| `permalink` | No | Custom URL slug (defaults to lowercase title with dashes) |
| `publish` | No | Set to `draft` to exclude from build |
| `lightTheme` | No | Set to `true` to add `light-theme` CSS class to body |

### Static pages

Create markdown files in `pages/` for standalone pages like "About" or "Contact":

```yaml
---
title: About me
description: A short description for meta tags
---

# About Me

Your page content here...
```

The filename (without `.md`) becomes the URL slug. For example:
- `pages/about-me.md` → `/about-me/`
- `pages/contact.md` → `/contact/`

A sample page is provided in `pages.example/`. Copy it to get started:

```bash
cp -r pages.example pages
```

#### Page frontmatter fields

| Field | Required | Description |
|-------|----------|-------------|
| `title` | Yes | Page title |
| `description` | No | Short description for meta tags |

### Search page

The search functionality is automatically generated at `/search/`. It embeds a JSON index of all posts that can be used by client-side JavaScript for instant search.

Your `search.html` template should include a `{resources}` placeholder where the JSON array will be injected:

```html
<script>
const searchIndex = {resources};
// Your search implementation here
</script>
```

The JSON structure for each post:
```json
{
  "title": "Post Title",
  "url": "https://example.com/post-slug/",
  "dateTimestamp": "2024-01-15",
  "dateHumanReadable": "2024/01/15",
  "excerpt": "Post description"
}
```

## Templates

### Template placeholders

Templates use `{placeholder}` syntax for dynamic content.

#### Homepage (`homepage.html`)

| Placeholder | Description |
|-------------|-------------|
| `{post_items}` | Rendered list of archive items |
| `{host}` | Site URL |
| `{website_name}` | Site name |
| `{website_description}` | Site description |
| `{author_name}` | Author name |

#### Archive item (`archive-item.html`)

| Placeholder | Description |
|-------------|-------------|
| `{post_link}` | Post URL path (permalink) |
| `{post_date_timestamp}` | Date in YYYY-MM-DD format |
| `{post_date_human_readable}` | Date in YYYY/MM/DD format |
| `{post_title}` | Post title |
| `{post_excerpt}` | Post description/excerpt |

#### Post page (`post.html`)

| Placeholder | Description |
|-------------|-------------|
| `{host}` | Site URL |
| `{website_name}` | Site name |
| `{author_name}` | Author name |
| `{post_date_timestamp}` | Date in YYYY-MM-DD format |
| `{post_date_human_readable}` | Date in YYYY/MM/DD format |
| `{post_title}` | Post title |
| `{post_description}` | Post description |
| `{post_content}` | Post HTML content |
| `{post_url}` | Full post URL |
| `{post_keywords}` | Post keywords |
| `{post_image_url}` | Open Graph image URL |
| `{theme_class}` | CSS class (`light-theme` or empty) |

#### Static page (`page.html`)

| Placeholder | Description |
|-------------|-------------|
| `{page_title}` | Page title |
| `{page_description}` | Page description |
| `{page_content}` | Page HTML content |
| `{page_url}` | Full page URL |
| `{page_slug}` | Page slug |
| `{host}` | Site URL |
| `{website_name}` | Site name |
| `{website_logo_url}` | Logo URL |
| `{author_name}` | Author name |

#### Search page (`search.html`)

| Placeholder | Description |
|-------------|-------------|
| `{host}` | Site URL |
| `{website_name}` | Site name |
| `{website_description}` | Site description |
| `{website_logo_url}` | Logo URL |
| `{author_name}` | Author name |
| `{twitter_handle}` | Twitter handle |
| `{resources}` | JSON array of posts for search index |

#### RSS feed (`feed.xml` and `feed-entry.xml`)

Standard Atom feed placeholders for syndication.

## Generated output

The build process creates:

```
build/
├── index.html              # Homepage
├── search/index.html       # Search page
├── feed.xml                # RSS/Atom feed
├── sitemap.xml             # XML sitemap
├── css/                    # Copied from public/
├── img/                    # Copied from public/
├── js/                     # Copied from public/
├── post-slug/index.html    # Individual posts
└── page-slug/index.html    # Static pages
```

## Plugin architecture

rusty-smith uses a plugin-based architecture. The build pipeline runs these plugins in order:

1. **BuildPlugin** - Creates build directory, copies static assets
2. **PostsPlugin** - Parses posts, filters drafts, sorts by date
3. **PostPlugin** - Generates individual post pages
4. **HomepagePlugin** - Generates the homepage with post archive
5. **PagesPlugin** - Generates static pages from `pages/`
6. **SearchPlugin** - Generates search page with JSON index
7. **FeedPlugin** - Generates RSS/Atom feed
8. **SitemapPlugin** - Generates XML sitemap

## License

MIT
