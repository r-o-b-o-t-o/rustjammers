# Rustjammers

Rustjammers is a [Windjammers](https://en.wikipedia.org/wiki/Windjammers_(video_game)) clone with basic AI agents.

## Installing

Install [rust](https://www.rust-lang.org/tools/install).

Install [Unity](https://unity3d.com/get-unity/download).

Open a `cmd` window or a terminal and compile the game engine:
```sh
cd rustjammers # Go to your copy of this repository
cargo build --release
```

Copy the resulting `rustjammers_engine.dll` library file in `rustjammers/target/release` to `rustjammers/Unity/Assets`.

Open the Unity project and run the game.
