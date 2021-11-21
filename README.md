# The Force

> The Force is a gateway to abilities many believe are unnatural...

```force
Do it!
    The Sacred Jedi Texts! "Hello there\n"
May The Force be with you.
```

The Force is a Star Wars inspired programming language. All keywords are made up of quotes from the movies and it is fully armed and operational!

## Getting Started

To learn about using The Force, please look at the [introduction](docs/introduction.md). We also have some [examples](examples) of full programs you can use as reference.

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

This project is dual-licensed under the MIT and Yoda License - see the [LICENSE.md](LICENSE.md) and [YODA-LICENSE.md](YODA-LICENSE.md) files for details.

The Force is in no way affiliated with or endorsed by Lucasfilm Limited or any of its subsidiaries, employees, or associates. All Star Wars quotes and references in this project are copyrighted to Lucasfilm Limited. This project intends to use these strictly within the terms of fair use under United States copyright laws.

<small>Disney please don't sue us.</small>
