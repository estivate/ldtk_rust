
# LDtk Rust Library

[![Crates.io](https://img.shields.io/crates/v/ldtk_rust.svg)](https://crates.io/crates/ldtk_rust)
[![Docs.rs](https://docs.rs/ldtk_rust/badge.svg)](https://docs.rs/ldtk_rust)

ldtk_rust enables access to [LDtk](https://ldtk.io) data for use in Rust.
LDtk is a 2D level editor for games that supports multiple tile layers, powerful
auto-tiling rules, entity placement and more.

## Status

This library works with LDtk version `0.7.0` and supports the optional external
level files. If you are using an earlier version of LDtk you should use the 
[v0.2.0](https://github.com/estivate/ldtk_rust/releases/tag/v0.2.0) version
of this library.

The Bevy example is still being updated to work with 0.7.0... once that's working
I'll generate another version for crates.io. In the meantime you can pull 
from `master`.

## Getting Started

Calling the new() method on the LdtkFile struct with the path to a LDtk file will
populate a struct that closely resembles the [LDtk JSON format](https://ldtk.io/json/).

```rust
use ldtk_rust::Project;

fn main() {
    let file_path = "assets/SeparateLevelFiles.ldtk".to_string();
    let ldtk = Project::new(file_path);
    println!("First level pixel height is {}!", ldtk.levels[0].px_hei);
}
```

Your editor's autocomplete should help you visualize your options, or you can generate
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

## Loading Projects and Levels Separately

LDtk saves all data in one (large) JSON file by default, but there is an option for saving each level in
it's own external file. ldtk_rust adopts this same approach. Calling the `ldtk_rust::Project::new()` method
with a path to an LDtk project file will load ALL the data available, regardless of whether it is in one
file or multiple files.

But this `new()` method is just a convenience wrapper for running `load_project()` followed
by `load_external_levels()` as necessary. If you need to load projects and levels separately you 
can call these two methods directly. If you want to load one level at a time, you can call 
the `load_project()` method followd by `Level::new()` to populate an individual. 


## Implementation Details

TK
