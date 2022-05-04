## Getting Started
### Creating a Project Directory

```bash
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

### Creating a Project with Cargo

```bash
$ cargo new hello_cargo
$ cd hello_cargo
$ cargo build
$ ./target/debug/hello_cargo
$ cargo run # build and run
$ cargo check # check that code compiles
```

* We can build a project using cargo build.
* We can build and run a project in one step using cargo run.
* We can build a project without producing a binary to check for errors using cargo check.
* Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

## Programming a Guessing Game

```bash
$ cargo new guessing_game
$ cd guessing_game
```