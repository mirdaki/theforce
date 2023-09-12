# The Force

[![Latest Crates.io Version][Crates Image]][Crates Download]
[![Latest Crates.io Version][Build Image]][Build]
[![MIT license][License Image]][License]

[Crates Image]: https://img.shields.io/crates/v/theforce?style=flat-square
[Crates Download]: https://crates.io/crates/theforce
[Build Image]: https://img.shields.io/github/actions/workflow/status/mirdaki/theforce/rust-check.yml?style=flat-square
[Build]: https://github.com/mirdaki/theforce/actions/workflows/rust-check.yml
[License Image]: https://img.shields.io/crates/l/theforce?style=flat-square
[License]: LICENSE.md

> The Force is a gateway to abilities many believe are unnatural...

The Force is a Star Wars inspired programming language. All keywords are made up of quotes from the movies and it is fully armed and operational!

```force
Do it!
    The Sacred Jedi Texts! "Hello there\n"
May The Force be with you.
```

## Getting Started

To learn about using The Force, please look at the [introduction](docs/introduction.md). We also have some [examples](examples) of full programs you can use as reference.

### Installing

If you have [cargo](https://doc.rust-lang.org/cargo/):
```bash
cargo install theforce
```

Or download directly from our [releases](https://github.com/mirdaki/theforce/releases).

### Usage

Run a `.force` file:
```bash
theforce /path/to/file
```

### Developing

[Install Rust](https://www.rust-lang.org/tools/install). We also provide a [Dev Container](https://code.visualstudio.com/docs/remote/create-dev-container) if you would prefer to run it that way.

To run the examples:
```bash
cargo run examples/hello-there.force
```

To run with LLVM support (currently a WIP):
```bash
cargo run examples/hello-there.force --features llvm
```

## Built With

Thank you to all the projects that helped make this possible!

- [Rust](https://www.rust-lang.org/) for being an awesome language to build with
- [Pest](https://pest.rs/) used for defining and parsing the grammar
- [Create Your Own Programming Language with Rust](https://createlang.rs/) provided an excellent introduction into the Rust tools needed to build this
- [ArnoldC](https://lhartikk.github.io/ArnoldC/) for providing inspiration for the design

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to the project.

## License

This project is dual-licensed under the MIT or Yoda License - see the [LICENSE.md](LICENSE.md) and [YODA-LICENSE.md](YODA-LICENSE.md) files for details.

The Force is in no way affiliated with or endorsed by Lucasfilm Limited or any of its subsidiaries, employees, or associates. All Star Wars quotes and references in this project are copyrighted to Lucasfilm Limited. This project intends to use these strictly within the terms of fair use under United States copyright laws.

<small>Disney please don't sue us.</small>
