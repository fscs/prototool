---
title: "Protokoll vom {{date}}"
date: "{{date_machine}}"
---

<details>
<summary>Anwesenheitsliste<summary>

##### Anwesende Räte

#### Abwesende Räte

#### Entschuldigte Räte

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

---
{{antrag.antragstext}}
---
{% endfor ~%}

_Top endet um T Uhr._
{% endfor ~%}

## Top {{tops.len() + 2}}: Verschiedenes

### Veranstaltungen

### Sonstiges

_Top endet um T Uhr._

