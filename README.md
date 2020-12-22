# LDtk Rust Library

This library enables access to [LDtk](https://ldtk.io) data for use in Rust.
LDtk is a 2D level editor for games that supports multiple tile layers, powerful
auto-tiling rules, entity placement and more.

ldtk_rust parses the JSON format created by LDtk into a typed Rust object.
You should be able to use this to generate game levels in any Rust game framework.

## Status

Currently all sample .ldtk files included in the LDtk 0.6.1 release load without
any errors. You can use the [basic example](examples/basic.rs) to check your own
files.

Most fields from the "Levels" section of the JSON are supported, but there are still
a number of fields from the "Defs" section that have not been implemented.

## Getting Started

Calling the new() method on the LdtkFile struct with the path to a LDtk file will
populate a struct that closely resembles the [LDtk JSON format](https://ldtk.io/json/).

```rust
use ldtk_rust::LdtkFile;

fn main() {
    let file_path = "assets/assets/AutoLayers_1_basic.ldtk".to_string();
    let ldtk = LdtkFile::new(file_path);
    println!("First level pxHei is {}!", ldtk.levels[0].px_hei);
}
```

CamelCase field naming used in JSON is converted to the snake_case style used in Rust.
A few other field names are altered as needed, for instance the field "type" cannot be
used in Rust since that is a reserved word.

Your editor's autocomplete should help you visualize your options, or you can generate
API docs with "cargo doc --open".

## Run the Examples

You can build and/or run the programs in the example folder using cargo:

```bash
> cargo run --example basic
```

Example dependencies do not load when compliling the library for production.

## Using with [Bevy Engine](https://bevyengine.org/)

An example running in Bevy 0.4 is included in the [examples](examples/) directory.
In a startup system you can read in the LDtk JSON and load all the tile assets. Then
you can spawn the tiles by iterating through the levels, layer instances and finally
tiles.

The Bevy example displays AutoTile layers correctly for most of the sample files, but
is currently not displaying Entities. It also currently errors on some of the more
advanced samples.

