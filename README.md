# Developing

Install needed tools:

    $ cargo install --locked trunk
    $ cargo install cargo-watch
    $ rustup target add wasm32-unknown-unknown

You also need to setup wasm tools for rust:

https://rustwasm.github.io/docs/book/game-of-life/setup.html


And then start dev servers:

    $ cd backend
    $ ./watch.sh

    $ cd frontend
    $ ./watch.sh