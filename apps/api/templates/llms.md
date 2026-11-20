# {{ title }}

> {{ summary }}

Every link below is a plain markdown document. A path ending in `/` lists its contents. Every document is listed in {{ sitemap_url }}, and all of them in one file at {{ full_url }}.
{% for section in sections %}
## {{ section.heading }}

{% for entry in section.entries -%}
- [{{ entry.title }}]({{ entry.url }}): {{ entry.summary }}
{% endfor -%}
{% endfor -%}
