#Readme.md

Stars is a simple GFX based application that creates a type of "dynamic art", much like a screen saver.  It was written in Rust to become familiar with the use of the GFX library under Rust.

#notes

Like other SDL2 applications under Windows, SDL2.dll must be located in the application directory or a suitable library path. See Cargo.toml for the GFX dependencies required. 

The GFX library must be made available for linking. If running under Windows, the following steps will build that library:

cargo install cargo-vcpkg
This install cargo-vcpkg, a one-time task.
Then,
cargo vcpkg build
This will need to be re-run when there is any new C or C++ lib to be included, for example sdl2 ttf. This usually takes some time.
Finally,
cargo build
This will build your project as usual, with the sdl2-gfx packages installed and able to run.



