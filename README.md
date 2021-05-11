Framework for supporting different programming languages in TMC.

TMC-langs provides an interface that encapsulates everything needed to support a new language in TMC. The framework provides CLI wrappers so that it's fairly convenient to call from other languages like Ruby.

## Documentation

Documentation for the latest release is available at https://rage.github.io/tmc-langs-rust

The specifications for various configuration files are included in the spec directory.

The student file policy of each plugin is explained in a `README.md` file in the plugin's subdirectory.

## Building

Install Rust according to https://www.rust-lang.org/tools/install

Install [zstd](https://github.com/facebook/zstd). For example, on Ubuntu you need the package `libzstd1`. For Windows, download the appropriate archive from the [releases](https://github.com/facebook/zstd/releases), extract it and add the extracted directory to your PATH.

```bash
git clone git@github.com:rage/tmc-langs-rust.git
cd tmc-langs-rust
cargo build
```

If you have any troubles building the project, please do make an issue!

## Testing

```bash
cargo test
```

## Running the CLI

```bash
cargo run -p tmc-langs-cli help
```

## Development

Format using `cargo fmt`, use `cargo clippy` for linting. All crates should have the clippy lints `print_stdout` and `print_stderr` set to deny. The CLI has one function where writing to stdout is allowed.

If using vscode and rust-analyzer, be sure to turn on the setting `loadOutDirsFromCheck` to avoid a spurious unresolved import error from `isolang`.

## Updating dependencies

In addition to the dependencies listed in each crate's `Cargo.toml`, the project bundles a few external dependencies such as `tmc-checkstyle-runner`, `tmc-junit-runner` and so on. When updating dependencies, you may want to check whether these projects have been updated.

## Versioning

tmc-langs-rust follows Rust-style semantic versioning, but only for the `tmc-langs-cli` and `tmc-langs` crates. Other crates may go through breaking changes in any release as long as the CLI and langs are unaffected.

## CLI binary deployment and downloads

Documentation and binaries for the supported targets are built and the binaries deployed to Google Cloud when creating a GitHub release. The binaries are available at https://download.mooc.fi/tmc-langs-rust/, with each binary following the file name format `tmc-langs-cli-{target}-{version}(.exe)`, with the `.exe` suffix added for the Windows binaries. For a list of targets see below. For example, The 64-bit Linux binary for version 0.5.0 is available at https://download.mooc.fi/tmc-langs-rust/tmc-langs-cli-x86_64-unknown-linux-gnu-0.5.0.

### Supported targets

- Linux 64-bit (x86_64-unknown-linux-gnu)
- Linux 32-bit (i686-unknown-linux-gnu)
- Windows MSVC 64-bit (x86_64-pc-windows-msvc)
- Windows MSVC 32-bit (i686-pc-windows-msvc)
- MacOS 64-bit (x86_64-apple-darwin)
- ARM64 (aarch64-unknown-linux-gnu)
- Armv7 (armv7-unknown-linux-gnueabihf)

## Included projects

### tmc-langs-cli

A "frontend" for tmc-langs. A binary CLI client for TMC-langs for IDEs. Intended to be used programmatically, for a CLI meant for manual use see [tmc-cli-rust](https://github.com/rage/tmc-cli-rust).

### tmc-langs

The "backend". A library that provides a convenient API for implementing different frontends. A frontend (such as a CLI) should only depend on this.

### tmc-client

A library for communicating with the TMC server.

### tmc-langs-framework

A library for creating language plugins.

### tmc-langs-plugins

A library that provides a convenient API for using all the different plugins.

### tmc-langs-util

A utility library that contains various kinds of useful functionality for other projects.

### plugins/csharp

A TMC plugin for C#.

### plugins/java

TMC plugins for Maven and Ant projects.

### plugins/make

A TMC plugin for Make.

### plugins/notests

A TMC plugin for projects with no tests.

### plugins/python3

A TMC plugin for Python 3.

### plugins/r

A TMC plugin for R.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
