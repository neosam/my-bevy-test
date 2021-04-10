# My Bevy Test

This is me messing with the bevy engine (https://bevyengine.org/) which is pretty
awesome!
This example lets you move the camera with the arrow keys.  It will look down 
by 45 degrees to the ground.  But there is more:  a cube!  You can press space 
and the game will spawn another cube on the center of the screen.   If 
you click on the cube, you can move it with ASDW-keys.

This project is dual licensed under Apache License 2.0 and the MIT License.

## Build and run

I tested this project on MacOS and it worked out of the box.  I don't know which
packages are required on any Linux distributions, sorry.  I guess, it should work
on Windows.

```
# Build
cargo build
# Run
cargo run
```

## Web Build

It's possible to run it in the browser.  You need wasm-unknown-unkown target
build and wasm-bindgen-cli to generate the JavaScript bindings.

```
# Build
cargo build --target wasm32-unknown-unknown --no-default-features --features web
# Generate JS Bindings
wasm-bindgen --out-dir target --out-name wasm --target web --no-typescript target/wasm32-unknown-unknown/debug/my-bevy-game.wasm
# Share the content of the web directory over http (e.g. with python)
cd web
python2 -m SimpleHTTPServer
```

Now you can run the game over http://localhost:8000
