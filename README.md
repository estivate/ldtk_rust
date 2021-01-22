
# LDtk Rust Library

[![Crates.io](https://img.shields.io/crates/v/ldtk_rust.svg)](https://crates.io/crates/ldtk_rust)
[![Docs.rs](https://docs.rs/ldtk_rust/badge.svg)](https://docs.rs/ldtk_rust)

ldtk_rust enables access to [LDtk](https://ldtk.io) data for use in Rust.
LDtk is a 2D level editor for games that supports multiple tile layers, powerful
auto-tiling rules, entity placement and more.

## Status

This library works with LDtk version `0.7.0` and supports the optional external
level files. LDtk updates save files automatically, so there's no reason to be
on an older version, but if you are you should use 
[v0.2.0](https://github.com/estivate/ldtk_rust/releases/tag/v0.2.0) of this library.

## Getting Started

Calling the new() method on the LdtkFile struct with the path to a LDtk file will
populate a struct that closely resembles the [LDtk JSON format](https://ldtk.io/json/).

```rust
use ldtk_rust::Project;

fn main() {
    let file_path = "assets/test_game.ldtk".to_string();
    let ldtk = Project::new(file_path);
    println!("First level pixel height is {}!", ldtk.levels[0].px_hei);
}
```

Your editor's auto-complete should help you visualize your options, or you can generate
API docs with "cargo doc --open" or view them [here](https://docs.rs/ldtk_rust/).

## Run the Examples

You can run the programs in the example folder using cargo:

```bash
> cargo run --example basic
```

Example dependencies do not load when compiling the library for production.

## Using in a Real Game

An example running in [Bevy Engine](https://bevyengine.org/) is included in the [examples](examples/) directory.
There are lots of comments, and the focus of the example is on the process, not the Bevy-specific code. If you
are using another game engine the example will hopefully still be understandable and useful.

Please note if you are using Bevy and you have more than one tileset referenced in LDtk, you may have 
intermittent issues due to [issue 1056](https://github.com/bevyengine/bevy/issues/1056).


## Implementation Details

* Use `Project::new()` to load all data, including any external level data. Use
this if you want to load all your data at startup and you don't want to worry about
whether level data is in separate files.

* If you want to load one level at a time, see `examples/single_level.rs`. Essentially
you will call `Project::load_project()` followed by `Level::new()` as you load each
level.

* The JSON deserialization is handled by serde using Rust code that is auto-generated
from the LDtk JSON schema. In general this code matches the LDtk
[documentation](https://ldtk.io/json/) except CamelCase names preferred in JSON
are changed to snake_case names preferred in Rust. JSON types of String, Int and Float
become Rust types of String, i64 and f64.

* Fields that allow null values are wrapped in a Rust `Option<T>`

* If you want to "tweak" anything about the 
way the schema Rust code is generated, you can change things 
[here](https://github.com/estivate/ldtk_rust/blob/master/src/json_0_7_0.rs).

## Other Options

* [ldtk-rs](https://github.com/katharostech/LDtk-rs) auto generates the entire 
crate from the JSON schema specified.

* The LDtk project publishes [QuickType loaders](https://ldtk.io/api/) for a 
variety of languages. If you have trouble with the Rust version, you can make 
one yourself by taking the 
[JSON schema](https://github.com/deepnight/ldtk/blob/master/docs/JSON_SCHEMA.json)
and pasting it into the [quicktype web tool](https://quicktype.io/). Set the "name"
to "Project" and the "Source type" to "JSON Schema", then on the right choose the
Rust language and set field visibility to "Public". Save the resulting file to
your project and change the serde import line to "use serde::*" and Bob's your Uncle.


