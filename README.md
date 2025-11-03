<div align="center">
  <h1><img src="assets/logo.png" alt="'RadioMare' Logo"></h1>
</div>

> [!NOTE]
> The following lists available mirrors of the repository:
> * [Codeberg](https://codeberg.org/r6915ee/radiomare) (Official repository,
> contribute by sending issues and pull requests here!)
> * [GitHub](https://github.com/r6915ee/radiomare)

> [!CAUTION]
> RadioMare is still in the early stages of development. Contributions are
> welcome under the MIT license.
>
> [Codeberg projects](https://codeberg.org/r6915ee/radiomare/projects) is used
> to detail what needs to be done for a release.

**RadioMare** is a mania-type rhythm game that primarily possesses the
following features:

* RadioMare possesses a chart system that favors reusability for faster
  development, going well with the **Entity Component System** paradigm.

todo

## Building

> [!TIP]
> [NixOS](https://nixos.org/) users have a Nix shell configuration available
> that introduces most of the dependencies.

The [Rust toolchain](https://rust-lang.org/) is necessary for compilation.
Either use Rustup, or install the compiler, Cargo, and Clippy.

Additionally, albeit not required, [`just`](https://github.com/casey/just) has
many recipes that are shorthands for a variety of tasks; for example,
documentation tests happen before the documentation actually gets generated in
the `doc` recipe. For this reason, the below examples will use `just`.

Compilation can be performed using the typical subcommand:

```sh
# Builds the workspace using the "dev" profile.
just dev
```

This will take a noticeably long amount of time initially. However, due to the
way that Cargo works with the Rust compiler, further compilations should be
faster, partly due to the way that Bevy is set up.

### Documentation

Documentation can be tested and then generated using the following recipe:

```sh
just doc
```

Additionally, a `view-docs` recipe exists that allows triggering an HTTP
server defined using `$HTTP_SERVER`. This opens the specified HTTP server in
the `target/doc` directory. The following examples assumes that
[http-server](https://github.com/http-party/http-server) is the server to use:

```sh
export HTTP_SERVER=http-server
just view-docs
```

## License

The project uses the [MIT license](LICENSE).
