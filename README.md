# eframe template

This is a minimal template repo for [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui).

## Getting started

Start by clicking "Use this template" at the top right. Clone and open it in your favorite code editor.

### Learning about egui

`src/app.rs` contains a simple example app. This is just to give some inspiration - most of it can be removed if you like.

The official egui docs are at <https://docs.rs/egui>. Check out the [the egui web demo](https://emilk.github.io/egui/index.html) and follow the links in it to its source code.

### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.
Then, run the app with `cargo run --release`.

On Linux, you need to install the following dev-dependencies:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

## Updating egui

When updating `egui` and `eframe`, it is recommended to do so one version at a time and to read about the changes in the [egui changelog](https://github.com/emilk/egui/blob/master/CHANGELOG.md) and the [eframe changelog](https://github.com/emilk/egui/blob/master/crates/eframe/CHANGELOG.md).
