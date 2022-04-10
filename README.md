# liveboard-rs

This is a prototype of using full-stack Rust to build a collaborative web dashboard.

This uses [Yew](https://yew.rs/) for the frontend (Vue-like Rust framework), [actix](https://actix.rs/) for the backend (websocket + HTTP API).

Another interesting feature of Rust is it allows sharing types between backend and frontend, removing the need to sync validation logic or helper methods between two languages. See [shared/src/datatypes/mod.rs](shared/src/datatypes/mod.rs).

This also features spline-based interpolation of the cursor positions to have nice animation instead of "jumpy" updates.

[![Demo video](https://i.ytimg.com/vi/V75dBjBPLkI/maxresdefault.jpg)](https://www.youtube.com/watch?v=V75dBjBPLkI "Demo video")


## Developing

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

## Inspirations

- [perfect-cursors](https://github.com/steveruizok/perfect-cursors) for the spline-based cursor interpolation