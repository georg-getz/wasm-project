# wasm-project

Implementation of the pathfinding algorithm

Prerequisites: Python

Once you install python just run the following command:

`python3 -m http.server 8080`

and open your browser on localhost:8080 to view the application

You can regenerate the contents of the pkg directory by running:

`wasm-pack build --target web`

You have to locally install `wasm-pack` if you want to do that

lib.rs implements the pathfinding algorithm and the vast majority of the logic is written in the internal function
so that it could be unit tested