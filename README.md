# Kani-chan

Kani-chan is a Final Fantasy XIV focused Discord bot. Planned features are: event planning, fetching and storing character data, and other silly features.

## Requirements

To run Kani-chan, you need two things:

-   A working install of the Rust toolchain
-   A Discord application bot token

## Running Kani

Clone the project:

```sh
git clone https://github.com/Velkee/kani-chan
```

Place your bot token into an `.env` file in the project's root:

```sh
DISCORD_TOKEN = "SOME_TOKEN"
```

Then, use cargo to compile and run the project.

To build and run a release binary of Kani:

```sh
cargo run --release
```

For a development binary, which compiles faster but runs slower, ideal for... well... development:

```sh
cargo run
```

You can also build a release and dev binary without running it with `cargo build --release` and `cargo build` respectively.
