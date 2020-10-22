## Pro-tips


### Cargo workspaces

This repo is organized in a "workspace" which means there's a single cargo
target directory shared between the sub-crates listed as members.

This means any common dependencies can be compiled once and reused between each
of the sub-crates that need them. It *also means* your own app's compile times
can be cut down since your own lib crates can be reused if unchanged.

### cargo-watch

If you install [cargo-watch], you can run the web service and automatically
recompile it when your sources change.

Since we're in a workspace, we can do this from the repo root like so:


```
$ cargo install cargo-watch
$ cargo watch -p omn-server -w omn-server -w omn-core -x run
```

> NB: the special sauce here is `-p` to select the *package* the `-x` (which
> sets the cargo command to invoke) flag will target.

If you just want to `cargo check` all your code instead of actually running
anything, this simplifies down as:

```
$ cargo watch -w omn-server -w omn-core -c
```

> In this case since we're focused on compiler feedback, we're specifying `-c`
> to clear the screen between runs.

[cargo-watch]: https://crates.io/crates/cargo-watch
