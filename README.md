# prototool

Keinen Bock mehr Tops aus dem Discord zu copy pasten? Du verlierst immer den Link zur Vorlage?
`prototool` kommt dir zur Rettung!

# Installation

## nix

Ganz normal als flake installieren oder einfach

```
nix run github:fscs/prototool
```

## nicht nix

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
      --to-pad                       Copies the protokolls content into the system clipboard and 
                                     opens an appropriate pad url in the webbrowser
      --from-pad <PAD_URL>           Load the protokoll content from a hedgedoc note
  -h, --help                         Print help
```
