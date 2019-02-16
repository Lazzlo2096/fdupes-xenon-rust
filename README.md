# fdupes-xenon
Duplicates finder on Windows and Linux. 
<!---
[![NPM Version][npm-image]][npm-url]
[![Build Status][travis-image]][travis-url]
[![Downloads Stats][npm-downloads]][npm-url]
-->
<!---
Crposs-platform and 
-->
Rust version of [fdupes](https://github.com/adrianlopezroche/fdupes) written from scratch. 

[//]:![](header.png)

## Building

```sh
cargo run "./for tests/" -r
#or
cargo build
./target/debug/fdupes-xenon <PATH> -r
```

## Testing

```sh
cargo test
```

## Release History

* 0.1.2
    * Feature 224: Pass multiple paths in arguments.
    * Add CLI flag for recursive search.
    * More acceptable out interface.
* 0.1.1
    * MAIN FUNCTION released: Now it findes Duplicates.
* 0.1.0
    * not working prototype.
	
<!---
## Usage example

A few motivating and useful examples of how your product can be used. Spice this up with code blocks and potentially more screenshots.

## Development setup

Describe how to install all development dependencies and how to run an automated test-suite of some kind. Potentially do this for multiple platforms.

```sh
make install
npm test
```

## Release History

* 0.2.1
    * CHANGE: Update docs (module code remains unchanged)
* 0.2.0
    * CHANGE: Remove `setDefaultXYZ()`
    * ADD: Add `init()`
* 0.1.1
    * FIX: Crash when calling `baz()` (Thanks @GenerousContributorName!)
* 0.1.0
    * The first proper release
    * CHANGE: Rename `foo()` to `bar()`
* 0.0.1
    * Work in progress
-->
