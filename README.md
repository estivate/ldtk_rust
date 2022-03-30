
# LDtk Rust Library

[![Crates.io](https://img.shields.io/crates/v/ldtk_rust.svg)](https://crates.io/crates/ldtk_rust)
[![Docs.rs](https://docs.rs/ldtk_rust/badge.svg)](https://docs.rs/ldtk_rust)

ldtk_rust enables access to [LDtk](https://ldtk.io) data for use in Rust.
LDtk is a 2D level editor for games that supports multiple tile layers, powerful
auto-tiling rules, entity placement and more.

## Status

This library works with LDtk version `1.0.0` and supports the optional external
level files. LDtk updates save files automatically, so there's no reason to be
on an older version, but if you are (or if you get a new version before this
crate is updated) you can follow the [process below](#using-with-older-or-newer-version-of-ldtk) to
generate code against whatever LDtk version you want to use. Or if you'd like to 
not use ANY library and embed the code directly in your game, take a look at
[this process](#how-to-not-use-this-library).

## Getting Started

Calling the new() method on the LdtkFile struct with the path to a LDtk file will
populate a struct that closely resembles the [LDtk JSON format](https://ldtk.io/json/).

```rust
use ldtk_rust::Project;

fn main() {
    let ldtk = Project::new( "assets/test_game.ldtk");
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

An example running in [Bevy Engine](https://bevyengine.org/) is included in the
[examples](examples/) directory. There are lots of comments, and the focus of 
the example is on the process, not the Bevy-specific code. If you
are using another game engine the example will hopefully still be understandable 
and useful.

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

## Other Options

* [ldtk-rs](https://github.com/katharostech/LDtk-rs) auto generates the entire 
crate from the JSON schema specified.

* The LDtk project publishes [QuickType loaders](https://ldtk.io/api/) for a 
variety of languages. These are auto generated, so they might need adjusting a bit.
Last time I tested the Rust version, I needed to tweak the serde line at the top.

* Embed the JSON conversion in your game (removing any dependencies) by following
the instructions below and reviewing the `lib.rs` file.

## Using With Older or Newer Version of LDtk

> You can enable older versions with features - just remember to disable default features and enable only one schema version feature:
> ```toml
> [dependencies.ldtk_rust]
> version = "*"
> default-features = false
> features = ["json_0_9_3"]
> ```

LDtk includes a [JSON schema](https://github.com/deepnight/ldtk/blob/master/docs/JSON_SCHEMA.json)
that can be used to auto-generate RUST code to unmarshal the JSON.

To use this library with an older (or newer) version of the LDtk Schema:

1. Clone this project
2. Check the [/src](https://github.com/estivate/ldtk_rust/tree/master/src)
subdirectory to see if there is already auto-generated code for the version you
want to use. If so, skip to #7 below. If not, you're going to create a new file
for the version you want to use.
3. Copy the version of the Schema file from the [LDtk Github](https://github.com/deepnight/ldtk)
that corresponds to your version and paste it into the [quicktype web tool](https://quicktype.io/).
The Schema is in the `docs/` directory.
4. On the left, set the "name" to "Project" and the "Source type" to "JSON Schema"
5. On the right choose the Rust language and set field visibility to "Public".
6. Save the resulting file to the /src subdirectory of this project 
7. Change the serde import line near the top of the file to "use serde::*;". You 
can view the other `.rs` version files to see this.
8. Change the `mod` and `pub use` lines at the top of `lib.rs` (in the same
directory you're working in already) to include your new file instead.

You'll need to adjust your Cargo.toml file to use your project instead of this
one (or contribute your change back here).

## How To Not Use This Library

Take a look at `lib.rs` and decide if you really even want this
project wrapping the autogenerated code, or if you want to just include it
in your own project directly. As LDtk nears 1.0 the JSON Schema is getting
better, and using this process has become easier. To do this:

1. Do the QuickType stuff in the instructions above (steps #3-5).
2. Copy and save the resulting Rust code to a file in your project.
3. Change the serde import line at the top (step #7 above).
4. Include this file in your project and have fun! You can review
the `lib.rs` file in this project to see how it's included and used.