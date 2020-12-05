dmxparser
==============

Reading the Valve Data Model eXchange (DMX) format in Rust

## Features

This library is split into 3 parts:

- A low-level reader that returns a `File` structure following closely the in-memory layout of the data. This API exists in two versions, the streaming `dmxparser::from_reader(impl BufRead) -> File` returning an owned version of the data, and the non-streaming `dmxparser::from_slice(&[u8]) -> File` returning a borrowed version of the data from the original buffer
- A high-level `serde::Deserializer` implementation in `dmxparser::serde::from_file<T: Deserialize>(File) -> T` that reads any deserializable data structure from a `File` returned by the low-level reader
- A library of predefined data structures for deserializing a VMAP file, accessible through `dmxparser::formats::vmap::read_vmap(File) -> CMapRootElement` 

## Limitations

At the moment this library:

- Can only read the binary DMX format, specifically in version 9
- Cannot write back a file to binary form
- Has missing parts in its `Deserializer` implementation, it's been tested on the VMAP structure but other formats might not work properly
- Has missing fields or invalid typings in the VMAP implementation, since the format isn't documented the type definitions were written to work on the vmap files found in the HL:Alyx SDK but other maps found in the wild may use structures with different shapes

All of this may eventually get fixed in a future (valve) time, but feel free to open a pull request if you want to help

## Tests

The crate has a dynamic test suite that generates test cases dynamically based on a `fixtures` directory containing map files to be loaded. It defaults to using the `tests/fixtures` directory but can be pointed at the map directory of a game whose maps can obviously not be included here for copyright reasons to test the library on real data with `cargo test --test reader -- --fixtures "D:/SteamLibrary/steamapps/common/Half-Life Alyx/content"`. Note that map files from retail games can be huge, and it can be necessary to disable the parallel execution of tests with `--test-threads 1` to prevent too many maps being loaded at the same time and running out of memory.
