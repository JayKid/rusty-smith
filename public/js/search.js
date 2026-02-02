// Homepage Item partial
const POST_ITEM_LINK_PLACEHOLDER = '{post_link}';
const POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER = '{post_date_timestamp}';
const POST_ITEM_DATE_READABLE_PLACEHOLDER = '{post_date_human_readable}';
const POST_ITEM_TITLE_PLACEHOLDER = '{post_title}';
const POST_ITEM_EXCERPT_PLACEHOLDER = '{post_excerpt}';

const QUERY_PARAMETER_KEY = "query";

const searchFor = needle => {
    return resources.filter(post => post.title.includes(needle.toLowerCase()) || post.url.includes(needle.toLowerCase()));
}

const reRenderResults = (resultsContainer, matches) => {
    let markup = "";
    matches.forEach(match => markup += getResultItemMarkup(match));
    resultsContainer.innerHTML = markup;
}

window.addEventListener('DOMContentLoaded', event => {
    const searchInput = document.getElementById('search');
    const resultsContainer = document.querySelector('.archive-list');
    searchInput.addEventListener("input", event => {
        if (event.target.value && event.target.value.length > 2) {
            const matches = searchFor(event.target.value);
            reRenderResults(resultsContainer, matches);
        }
        else {
            resultsContainer.innerHTML = "";
        }
    })
    const urlParameters = new URLSearchParams(window.location.search);
    const searchQuery = urlParameters.get(QUERY_PARAMETER_KEY);

    if (searchQuery) {
        const matches = searchFor(searchQuery);
        reRenderResults(resultsContainer, matches);
        searchInput.value = searchQuery;
    }
})

const getResultItemMarkup = match => {
    return ITEM_TEMPLATE
        .replace(POST_ITEM_LINK_PLACEHOLDER, match.url)
        .replace(POST_ITEM_TITLE_PLACEHOLDER, match.title)
        .replace(POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER, match.dateTimestamp)
        .replace(POST_ITEM_DATE_READABLE_PLACEHOLDER, match.dateHumanReadable)
        .replace(POST_ITEM_EXCERPT_PLACEHOLDER, match.excerpt ? match.excerpt : "");
}

const ITEM_TEMPLATE = `<li class="archive-list-item">
<a class="archive-post-item" href="{post_link}">
<div class="date-container">
    <time datetime="{post_date_timestamp}" itemprop="datePublished">{post_date_human_readable}</time>
</div>
<div class="content-container">
    <h2 class="archive-post-title">{post_title}</h2>
    <p class="post-excerpt">
        {post_excerpt}
    </p>
</div>
</a>
</li>`;