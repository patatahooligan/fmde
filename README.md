# fmde

Forbidden Memories Data Editor (fmde) is a tool to view and modify the
data in the PlayStation game "Yu-Gi-Oh! Forbidden Memories".

As of writing this, fmde is incomplete. It can currently find and read
some of the game data inside a .bin file. It prints this data to stdout
so we can verify that it's reading correctly. Next steps to make fmde
functional would be:

- Implement reading a configuration (deck lists, drop rates) from a
  static file
- Implement writing to a .bin file

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

To test the app while developping, you can run it via cargo.

```bash
cargo run
```

The `run` command accepts `--release` as well.

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
