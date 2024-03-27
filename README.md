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
  -l, --lang <LANG>    Under which language the post should be created [default: de]
  -e, --edit [<EDIT>]  Open the post for editing. Optionally takes the editor to use, falls back to $EDITOR otherwise
  -h, --help           Print help

# prototool gen
Generate a new Protokoll

Usage: prototool gen [OPTIONS]

Options:
  -U <ENDPOINT_URL>      Endpoint to fetch Tops from [default: https://fscs.hhu.de/]
  -l, --lang <LANG>      Under which language the protokoll should be created [default: de]
  -e, --edit [<EDIT>]    Open the protokoll for editing. Optionally takes the editor to use, falls back to $EDITOR otherwise
  -h, --help             Print help
```
