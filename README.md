# prototool

Keinen Bock mehr Tops aus dem Discord zu copy pasten? Du verlierst immer den Link zur Vorlage?
`prototool` kommt dir zur Rettung!

# Installation

## nix

```
nix run github:fscs/prototool
```

## nicht nix

Statische Binaries für Linux, MacOS und Windows gibts in den [Releases](https://github.com/fscs/prototool/releases/latest).

Oder auch einfach mit [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```
cargo install --git https://github.com/fscs/prototool
```

# Usage

```
# prototool new
Create a new post

Usage: prototool new [OPTIONS] <PATH>

Arguments:
  <PATH>  Path of the new post. e.g. posts/test.md

Options:
  -l, --lang <LANG>  Under which language the post should be created [default: de]
  -e, --edit         Open the post for editing
  -f, --force        Force creation, even if a file already exist
  -h, --help         Print help

# prototool gen
Generate a new Protokoll

Usage: prototool gen [OPTIONS]

Options:
  -U, --endpoint-url <ENDPOINT_URL>  Endpoint to fetch Tops from [default: https://fscs.hhu.de/]
  -l, --lang <LANG>                  Under which language the protokoll should be created [default: de]
  -e, --edit                         Open the protokoll for editing
  -f, --force                        Force creation, even if a file already exist
      --to-clipboard                 Generate the protokoll into the system clipboard
      --from-clipboard               Load the protokoll content from the system clipboard
      --to-pad                       Copies the protokolls content into the system clipboard and opens an appropriate pad url in the webbrowser
      --from-pad <PAD_URL>           Load the protokoll content from a hedgedoc note
      --no-ask-presence              Dont Ask for Presence
  -h, --help                         Print help
```

Man könnte es auch [protocool](https://www.youtube.com/watch?v=BxDmrzuPsYk) nennen :sunglasses:
