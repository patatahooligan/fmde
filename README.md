# fmde

Forbidden Memories Data Editor (fmde) is a tool to view and modify the
data in the PlayStation game "Yu-Gi-Oh! Forbidden Memories". Only the US
version of the ROM (SLUS-01411) is supported.

Currently, fmde supports only one piece of data: cardlists, which
includes both the decks and the droplists of each duelist.

You can perform two operations:

- dump all of the data from a ROM file (in .bin format) to a collection
  of .csv files
- apply a mod, which is defined as a collection of .csv files, to the
  ROM file

These operations are symmetrical. The .csv files produced by the first
are the exact same files that are used by the second. So a simple use of
the tool would be:

Dump the data from the ROM. It is recommended to use an empty directory
because this will create several files.

```bash
fmde dump path/to/rom.bin path/to/csv/files/
```

This will fill the directory with the following structure:

```
path/to/csv/files/
├── 1.Simon Muran
│   ├── deck.csv
│   ├── drops-bcd.csv
│   ├── drops-sa-pow.csv
│   └── drops-sa-tec.csv
├── 2.Teana
│   ├── deck.csv
│   ├── drops-bcd.csv
│   ├── drops-sa-pow.csv
│   └── drops-sa-tec.csv
...
```

All the files use the same exact format. For whatever reason Forbidden
Memories represents decks as a list of card probabilities, just like the
drop rates. Each row only has two meaningful values: the card id and the
probability. fmde ignores any item beyond the first two. However, a
third column is included with the card names to make editing the files
easier.

For example, Simon Muran's SA POW drop list starts like this:

```
2,27,Mystical Elf
9,90,Shadow Specter
10,30,Blackland Fire Dragon
...
```

The first row is parsed like this:
- 2 -> card id
- 27 -> probability of Simon Muran dropping this card. The game uses a
  random number [0, 2048) to decide the result, so this card would drop
  with a probability of 27/2048, which is approximately 1.31%.
- "Mystical Elf" -> This is completely ignored, but it exists to help
  you remember that card id 2 is Mystical Elf.

The sum of all listed probabilities must sum to 2048! This is how the
game expects the lists to be defined and fmde will refuse to patch the
file otherwise.

If a card id does not appear at all in the file, fmde understands this
to mean that the probability is zero. So in your files you can define
only the non-zero probability cards. Similarly, if a file does not
exist, fmde assumes you did not want to make changes. For example, if
you delete Simon Muran's deck.csv, then your mod will have Simon Muran
use the unmodified deck from the base ROM.

After you have modified the lists as desired, apply the patch.

```bash
fmde apply \
    path/to/rom.bin \
    path/to/csv/files \
    path/to/save/mod.bin
```

## Building

This project is managed by cargo, the rust package manager. To build,
simply run

```bash
cargo build
```

For an optimized release build, run:

```bash
cargo build --release
```

## Running

To test the app while developping, you can run it via cargo. It is
recommended to separate cargo arguments from fmde arguments with `--`.
eg

```bash
cargo run --release -- apply \
    path/to/rom.bin \
    path/to/csv/files \
    path/to/save/mod.bin
```

If you want to install the app so you can use it regularly, run

```bash
cargo install
```

By default, this places the executable under `$HOME/.cargo/bin`. Add
this to your `PATH` or run the application using the absolute path
`~/.cargo/bin/fmde`.

## Documentation

Writing fmde requires experimentation and studying random umaintained
code I find online. I am not working with a reliable specification here
which means that I have to make sure to keep notes of everything
relevant I find, lest I forget it and lose progress. For this reason,
`docs/` might have incomplete information and guesses until the project
matures.

## License

This program is free software: you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the
Free Software Foundation, either version 3 of the License, or (at your
option) any later version.

## Acknowledgments

Thanks to the developers behind these projects. I have not directly used
any of their code, but they include some critical info, such as the data
layout of the YGO:FM ROM and the text encoding.

- https://github.com/forbidden-memories-coding/fmlib-cpp
- https://github.com/forbidden-memories-coding/fmscrambler
- https://github.com/forbidden-memories-coding/FMRandomizer

Thanks also to [teaonline](https://www.basededatostea.xyz/) for having
an easily browsable database of cards, decks, droplists and more.
