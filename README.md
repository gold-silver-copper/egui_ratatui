# ratframe : egui widget + ratatui backend (WASM)

RataguiBackend is the name of the backend/widget in code

```cargo add ratframe```

This repo provides the Ratagui Backend, which is also a Widget for egui. So you can have a whole ratatui terminal inside of egui. Also since egui is so WASM compatible, this means we can use this to compile ratatui to WASM !
Look at examples, you can run the basic ones by doing



```cargo run --example hello_world_minimum```

To run a web version, enter an example directory such as demo_web or colors_web , then simply execute in the directory

```cd examples/demo_web```

```trunk serve```

or to build

```trunk build --release```


The goal is for this to be the simplest way to write a TUI app for WASM  in Rust.

You can compile your app natively or for the web, and share it using Github Pages.

Here is the simplest hello world

```rust
use eframe::egui::{self};

use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::Paragraph,
};
use ratframe::RataguiBackend;

pub fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let boop = RataguiBackend::new(100, 50);
    let mut terminal = Terminal::new(boop).unwrap();
    terminal.clear().expect("epic fail");

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        terminal
            .draw(|frame| {
                let area = frame.size();
                frame.render_widget(Paragraph::new("Hello Rataguiii").white().on_blue(), area);
            })
            .expect("epic fail");
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(terminal.backend_mut());
            if ui.input(|i| i.key_released(egui::Key::Q)) {
                panic!("HAVE A NICE WEEK");
            }
        });
    })
    .expect("epic fail");
}
```


REST IS FROM ETEMPLATE DOCS

## Getting started

Start by clicking "Use this template" at https://github.com/emilk/eframe_template/ or follow [these instructions](https://docs.github.com/en/free-pro-team@latest/github/creating-cloning-and-archiving-repositories/creating-a-repository-from-a-template).

Change the name of the crate: Chose a good name for your project, and change the name to it in:
* `Cargo.toml`
    * Change the `package.name` from `eframe_template` to `your_crate`.
    * Change the `package.authors`
* `main.rs`
    * Change `eframe_template::TemplateApp` to `your_crate::TemplateApp`
* `index.html`
    * Change the `<title>eframe template</title>` to `<title>your_crate</title>`. optional.
* `assets/sw.js`
  * Change the `'./eframe_template.js'` to `./your_crate.js` (in `filesToCache` array)
  * Change the `'./eframe_template_bg.wasm'` to `./your_crate_bg.wasm` (in `filesToCache` array)




### Learning about egui

`src/app.rs` contains a simple example app. This is just to give some inspiration - most of it can be removed if you like.

The official egui docs are at <https://docs.rs/egui>. If you prefer watching a video introduction, check out <https://www.youtube.com/watch?v=NtUkr_z7l84>. For inspiration, check out the [the egui web demo](https://emilk.github.io/egui/index.html) and follow the links in it to its source code.

### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web Locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

### Web Deploy
1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website
3. Upload the `dist` directory to any of the numerous free hosting websites including [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site).
4. we already provide a workflow that auto-deploys our app to GitHub pages if you enable it.
> To enable Github Pages, you need to go to Repository -> Settings -> Pages -> Source -> set to `gh-pages` branch and `/` (root).
>
> If `gh-pages` is not available in `Source`, just create and push a branch called `gh-pages` and it should be available.
>
> If you renamed the `main` branch to something else (say you re-initialized the repository with `master` as the initial branch), be sure to edit the github workflows `.github/workflows/pages.yml` file to reflect the change
> ```yml
> on:
>   push:
>     branches:
>       - <branch name>
> ```

You can test the template app at <https://emilk.github.io/eframe_template/>.

## Updating egui

As of 2023, egui is in active development with frequent releases with breaking changes. [eframe_template](https://github.com/emilk/eframe_template/) will be updated in lock-step to always use the latest version of egui.

When updating `egui` and `eframe` it is recommended you do so one version at the time, and read about the changes in [the egui changelog](https://github.com/emilk/egui/blob/master/CHANGELOG.md) and [eframe changelog](https://github.com/emilk/egui/blob/master/crates/eframe/CHANGELOG.md).
