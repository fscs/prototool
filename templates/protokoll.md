---
title: "{{ sitzung|protokoll_title }}"
date: "{{ sitzung.datetime.format("%Y-%m-%d") }}"
draft: true
hiddenUntil: "{{ (sitzung.datetime|hidden_until_date).format("%Y-%m-%d") }}"
---

<details>
<summary>Anwesenheitsliste</summary>

#### Anwesende Rätys
{%~ for rat in raete -%}
{%- if !rat.abgemeldet -%}
- {{ rat.name }}
{% endif -%}
{%- endfor ~%}

#### Abwesende Rätys

#### Entschuldigte Rätys
{%~ for rat in raete -%}
{%- if rat.abgemeldet -%}
- {{ rat.name }}
{% endif -%}
{%- endfor ~%}

{%~ if sitzung.sitzung_type == SitzungType::VV || sitzung.sitzung_type == SitzungType::WahlVV -%}
#### Weitere Studis
{%- else -%}
#### Gäste
{%- endif %}

</details>

## Top 0: Regularia

- Redeleitung: 
- Protokoll: 
- Startzeit: 
- Endzeit: {# this comment is a hack to sneak in a whitespace at the end of the line #}
{%~ if sitzung.sitzung_type == SitzungType::VV || sitzung.sitzung_type == SitzungType::WahlVV -%}
- Wir sind mit n Studierenden vorläufig beschlussfähig
- Wir nehmen das Protokoll der letzten VV einstimmig an
{% else -%}
- Wir sind mit n von {{ raete.len() }} Rätys beschlussfähig
- Wir nehmen das Protokoll der letzten Sitzung einstimmig an
{% endif -%}
- Wir nehmen die unten aufgelistete Topliste einstimmig an

## Top 1: Berichte, Mail und Post

### Berichte

### Mail

### Briefpost

### ToDo's

_Top endet um T Uhr._
{% for top in tops|normal_tops %}
## Top {{ loop.index0 + 2 }}: {{top.name}}

{%~ for antrag in top.anträge -%}

{%~ if top.anträge.len() > 1 ~%}
### Antrag: {{ antrag.titel }}
{%~ endif -%}
  
{{ antrag.begründung }}

```vote-success
{{antrag.antragstext}}  
Abstimmung: n Zustimmen, n Gegenstimmen, n Enthaltungen  
```
{% endfor ~%}

_Top endet um T Uhr._
{% endfor ~%}

## Top {{(tops|normal_tops).len() + 2}}: Verschiedenes

### Anstehende Veranstaltungen
{%- for event in events ~%}
- {{ event|event_format }}
{%- endfor %}

### Sonstiges
{% for top in tops|sonstige_tops -%}
{%~ for antrag in top.anträge ~%}

#### {{ antrag.titel }}
{{ antrag.begründung }}

{%- endfor ~%}
{%~ endfor ~%}

_Top endet um T Uhr._

