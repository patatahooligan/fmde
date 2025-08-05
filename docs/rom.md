# ROM

This document contains information on the format of the ROM. As of
writing this, it is a work in progress and it heavily depends on
trial-and-error as well as studying source code of whatever PS1
ROM-handling tool I can find.

## CD Format

The CD seems to use a single data track and nothing else. In bin/cue
format, this means that the cue file should be something like this

```
FILE "Yu-Gi-Oh! Forbidden Memories.bin" BINARY
  TRACK 01 MODE2/2352
    INDEX 01 00:00:00
```

Unfortunately, bin/cue files are not natively handled on Linux. If we
create a loop device of the files with `losetup` no filesystem will be
recognized. Instead, we need dedicated software like
[cdemu](https://wiki.archlinux.org/title/CDemu) to browse its contents.

Inside the data track, there appears to be an ISO-9660 filesystem which
can be mounted and explored. This is the filesystem structure:

```
/mnt/
├── data
│   ├── master.xa
│   ├── model.mrg
│   ├── movie.str
│   ├── sd_bgm.dat
│   ├── sd_se.dat
│   ├── su.mrg
│   └── wa_mrg.mrg
├── slus_014.11
└── system.cnf
```

As far as I can tell, the card data we care about are in two files:
`slus_014.11` and `wa_mrg.mrg`.

## Editing the data

Presumably, we don't have to actually handle all these formats. I'm
assuming the files are written contiguously. So if we figure out the
byte offsets of the files inside the bin file, we should be able to just
pull the binary data out without ever learning how the filesystem works
in much detail. As of writing this I don't have the first functioning
version of the code so these assumptions are not yet validated.

## Trying to locate text in the ROM

This project contains the utility binary `rom-to-text` to convert binary
data from the ROM into text. You can run it like this:

```bash
cargo run --bin rom-to-text \
    path/to/rom \
    > path/to/output.txt
```

Note that this converts exactly one byte to exactly one ASCII character,
so the files have a 1-1 correspondence between the n-th byte of the ROM
and the n-th byte of the txt file. So if we find that a word/phrase we
care about in the txt file at byte offset 100, for example, then the
binary form of the same word/phrase starts at byte offset 100 in the
ROM, or whatever input file we gave `rom-to-text`. Any character that
isn't recognized as text is converted to "_". `0` happens to represent
whitespace (" "), so expect to see lots of these in the output.

To locate a word/phrase, we can use `awk` as shown
[here](https://stackoverflow.com/a/74399870/2292621):

```bash
awk -v RS='<pattern>' '{print length($0); exit}' < dump
```

`-v RS=<pattern>` tells awk to use `<pattern>` as the delimiter. This
means that the record `$0` is all characters from the start of file to
the first occurence of `<pattern>`. Therefore `length($0)` gives us the
byte offset of the first occurence of `<pattern>`.

Here's a full example of what we can do:

```bash
cargo run \
    --bin rom-to-text \
    'Yu-Gi-Oh! Forbidden Memories.bin' \
    > ygofm.txt

awk \
    -v RS="Blue-eyes White Dragon" \
    '{print length($0); exit}' \
    ygofm.txt
```

This prints `2194441`. So if we then write Rust code that opens the ROM
file and starts reading at position `2194441`, we will read the binary
data that encodes "Blue-eyes White Dragon".
