---
title: "{{ sitzung|protokoll_title }}"
date: "{{ sitzung.datetime.format("%Y-%m-%d") }}"
draft: true
hiddenUntil: "{{ (sitzung.datetime|hidden_until_date).format("%Y-%m-%d") }}"
sitzung-kind: "{{ sitzung.kind}}"
---

<details>
<summary>Anwesenheitsliste</summary>

#### Anwesende Rätys
{%~ for rat in raete -%}
{%- if rat.anwesend -%}
- {{ rat }}
{% endif -%}
{%- endfor ~%}

#### Abwesende Rätys
{%~ for rat in raete -%}
{%- if !rat.anwesend && !rat.abgemeldet -%}
- {{ rat }}
{% endif -%}
{%- endfor ~%}

#### Entschuldigte Rätys
{%~ for rat in raete -%}
{%- if rat.abgemeldet && !rat.anwesend -%}
- {{ rat }}
{% endif -%}
{%- endfor ~%}

{%~ if sitzung.kind == SitzungKind::VV || sitzung.kind == SitzungKind::WahlVV -%}
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
{%~ if sitzung.kind == SitzungKind::VV || sitzung.kind == SitzungKind::WahlVV -%}
- Wir sind mit n Studierenden vorläufig beschlussfähig
- Wir nehmen das Protokoll der letzten VV einstimmig an
{% else -%}
- Wir sind mit {{ raete|anwesende_raete_label }} von {{ raete.len() }} Rätys {{ raete|beschlussfaehig_label }}
{%~ if raete|beschlussfaehig -%}
- Wir nehmen das Protokoll der letzten Sitzung einstimmig an
{% else -%}
- Es wird eine Ersatzsitzung gehalten, D um T Uhr
- Wir können aufgrund der fehlenden Beschlussfähigkeit das Protokoll nicht annehmen
{% endif -%}
{% endif -%}
{%~ if (sitzung|nicht_fristgerechte_antraege).is_empty() -%}
- Wir nehmen die unten aufgelistete Topliste einstimmig an
{% else -%}
- Die folgenden Anträge wurden nicht fristgerecht eingereicht:
{%- for antrag in sitzung|nicht_fristgerechte_antraege %}
    - {{ antrag.titel }}    
{%- endfor ~%}
- Wir nehmen die unten aufgelistete Topliste {%~ if !(sitzung|nicht_fristgerechte_antraege).is_empty() -%} mit den oben genannten Änderungen{% endif ~%} einstimmig an
{% endif ~%}

_Falls Begriffe unklar sind, verweisen wir auf unser [Abkürzungsverzeichnis](https://fscs.hhu.de/wtf)_

## Top 1: Berichte, Mail und Post

### Berichte

### Mail

### Briefpost

### ToDo's

_Top endet um T Uhr._
{% for top in sitzung.tops|normal_tops %}
## Top {{ loop.index0 + 2 }}: {{top.name}}

{{top.inhalt}}

{%~ for antrag in top.anträge ~%}

### Antrag: {{ antrag.titel }}

{{ antrag.begründung }}

```vote-success
{{antrag.antragstext}}

Abstimmung: n Zustimmen, m Gegenstimmen, k Enthaltungen  
```
{% endfor ~%}

_Top endet um T Uhr._
{% endfor ~%}

## Top {{(sitzung.tops|normal_tops).len() + 2}}: Verschiedenes

### Anstehende Veranstaltungen
{%- for event in events ~%}
- {{ event|event_format }}
{%- endfor %}

### Sonstiges

_Top endet um T Uhr._

