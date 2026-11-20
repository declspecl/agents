# Index of {{ url_path }}
{% for line in lines %}{% if loop.first %}
{% endif %}- [{{ line.label }}]({{ line.url_path }}){% if let Some(summary) = line.summary %}: {{ summary }}{% endif %}
{% endfor -%}
