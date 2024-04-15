---
title: "Protokoll vom {{datetime.format("%d.%m.%Y")}}"
date: "{{ datetime.format("%Y-%m-%dT%H:%M:%S") }}"
---

<details>
<summary>Anwesenheitsliste</summary>

#### Anwesende Räte
{%~ for rat in raete -%}
{%- if !rat.abgemeldet -%}
- {{ rat.name }}
{% endif -%}
{%- endfor ~%}

#### Abwesende Räte

#### Entschuldigte Räte
{%~ for rat in raete -%}
{%- if rat.abgemeldet -%}
- {{ rat.name }}
{% endif -%}
{%- endfor ~%}

#### Gäste

</details>

## Top 0 : Regularia

Protokollant: 
Redeleitung: 
Startzeit: 
Endzeit: 
Wir sind mit n von nmax Räten beschlussfähig
Wir nehmen die unten aufgelistete Topliste einstimmig an
Wir nehmen das Protokoll der letzten Sitzung einstimmig an

## Top 1: Berichte, Mail und Post

## Berichte

## Mail

## Briefpost

## ToDo's

_Top endet um T Uhr._
{% for top in tops %}
## Top {{ loop.index0 + 2 }}: {{top.name}}

{%~ for antrag in top.anträge -%}

{%~ if top.anträge.len() > 1 ~%}
### Antrag: {{ antrag.titel }}
{%~ endif -%}
  
{{ antrag.begründung }}

:::info
{{antrag.antragstext}}
**Abstimmung:** n Zustimmen, n Gegenstimmen, n Enthaltungen
:::
{% endfor ~%}

_Top endet um T Uhr._
{% endfor ~%}

## Top {{tops.len() + 2}}: Verschiedenes

### Anstehende Veranstaltungen
{%- for event in events ~%}
- {{ event.start.format("%d.%m.") }} {{ event.title }} {{ event.start.format("%H:%M")}} Uhr {{ event.location }}
{%- endfor %}

### Sonstiges

_Top endet um T Uhr._

