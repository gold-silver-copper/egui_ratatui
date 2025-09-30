# egui_ratatui : egui widget + ratatui backend (WASM)

[![Crates.io](https://img.shields.io/crates/v/egui_ratatui.svg)](https://crates.io/crates/egui_ratatui)
[![Documentation](https://docs.rs/egui_ratatui/badge.svg)](https://docs.rs/egui_ratatui/latest/egui_ratatui/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bevyengine/bevy/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/egui_ratatui.svg)](https://crates.io/crates/egui_ratatui)

---

`RataguiBackend` is a combined **ratatui Backend** and **egui Widget** that lets you render a full Ratatui terminal inside an egui UI. Because egui is WASM-friendly, this makes it easy to run terminal-style TUI apps in desktop GUIs or in the browser.

![](https://github.com/gold-silver-copper/egui_ratatui/blob/main/tyorhun.gif)
![](https://github.com/gold-silver-copper/egui_ratatui/blob/main/screen3.png)

---

## Highlights

`egui_ratatui` builds on top of the [`soft_ratatui`] library and inherits its features:

* **Software rendering backend for `ratatui`** — no GPU required.
* **Multiple font backends**: `embedded-graphics`, `embedded-ttf`, `bdf-parser`, `cosmic-text`.
* **Flexible pixel output**: raw RGB or RGBA pixmaps (with color-to-alpha support).
* **High performance**: optimized for real-time UIs (hundreds of FPS on normal workloads).
* **WASM compatible**: run your ratatui apps in the browser via egui.
* **Full Ratatui `Backend` trait implementation** — works with `ratatui::Terminal`.
* **Bevy & GUI friendly** — ship TUIs as textures to games or GUI apps.

---

## Installation

Add the crate to your project:

```bash
cargo add egui_ratatui
```

You will typically also add `soft_ratatui` (the rendering backends) and `ratatui`:

```bash
cargo add soft_ratatui
cargo add ratatui
```

Or clone and run the included examples:

```bash
git clone https://github.com/gold-silver-copper/egui_ratatui.git
cd egui_ratatui
cd bevy_example
cargo run --release
```

---

## Quick usage

### Minimal `ratatui` example (terminal style)

```rust
use eframe::egui;
use egui_ratatui::RataguiBackend;
use ratatui::Terminal;
use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use soft_ratatui::embedded_graphics_unicodefonts::{
    mono_8x13_atlas, mono_8x13_bold_atlas, mono_8x13_italic_atlas,
};
use soft_ratatui::{EmbeddedGraphics, SoftBackend};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let font_regular = mono_8x13_atlas();
    let font_italic = mono_8x13_italic_atlas();
    let font_bold = mono_8x13_bold_atlas();
    let soft_backend = SoftBackend::<EmbeddedGraphics>::new(
        100,
        50,
        font_regular,
        Some(font_bold),
        Some(font_italic),
    );
    let mut backend = RataguiBackend::new("soft_rat", soft_backend);
    //backend.set_font_size(12);
    let mut terminal = Terminal::new(backend).unwrap();

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        terminal
            .draw(|frame| {
                let area = frame.area();
                let textik = format!("Hello eframe! The window area is {}", area);
                frame.render_widget(
                    Paragraph::new(textik)
                        .block(Block::new().title("Ratatui").borders(Borders::ALL))
                        .white()
                        .on_blue()
                        .wrap(Wrap { trim: false }),
                    area,
                );
            })
            .expect("epic fail");
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(terminal.backend_mut());
        });
    })
}
```
---

## Feature Flags (inherited from `soft_ratatui`)

`soft_ratatui` (and therefore `egui_ratatui`) is modular. Enable only the backends you need to reduce binary size and dependencies.

| Feature             | Enables                            | Description                                                                                                  |
| ------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `unicodefonts`      | [`embedded_graphics_unicodefonts`] | Embedded-graphics fonts with Unicode support. Automatically enables `embedded-graphics`. Enabled by default. |
| `embedded-graphics` | [`EmbeddedGraphics`]               | Uses embedded-graphics font atlases for TUI rendering.                                                       |
| `bdf-parser`        | [`Bdf`]                            | Bitmap Distribution Format font support.                                                                     |
| `embedded-ttf`      | [`EmbeddedTTF`]                    | TrueType font rendering via RustType. Automatically enables `embedded-graphics`.                             |
| `cosmic-text`       | [`CosmicText`]                     | Advanced text shaping, layout, and Unicode support using CosmicText engine.                                  |

> Tip: Only enable the font backends you actually use in `Cargo.toml` to keep compile times and binary size down.

---

## Integration notes

* The crate is designed so `RataguiBackend<R>` is generic over any `R: RasterBackend`. That means you can use the same `RataguiBackend` code with `SoftBackend<EmbeddedGraphics>`, `SoftBackend<EmbeddedTTF>`, `SoftBackend<Bdf>`, or whichever backend you enable.
* `RataguiBackend` implements Ratatui's `Backend` trait, so it can be used anywhere a `ratatui::Terminal` is expected.
* When used as an egui widget, textures are loaded with `TextureOptions::NEAREST` and cached; `RataguiBackend` stores the `TextureHandle` so subsequent frames reuse the texture unless the content changed.
* For web/WASM deployment, remember to enable the appropriate features and follow the host GUI / wasm backend guide (see links below).

---

## Examples

* `bevy_example/` — embedding Ratagui inside Bevy (with egui).
* `wasm_example/` — run the widget in the browser via egui/eframe.
* See the `soft_ratatui` repo for font/backend-specific examples (BDF, TTF, CosmicText, embedded-graphics).

---

## Useful links

* [`soft_ratatui`]: [https://github.com/gold-silver-copper/soft_ratatui](https://github.com/gold-silver-copper/soft_ratatui) — software rendering backends used by egui_ratatui.
* [`ratatui`]: [https://github.com/ratatui/ratatui](https://github.com/ratatui/ratatui) — terminal UI crate.
* [`egui`]: [https://github.com/emilk/egui](https://github.com/emilk/egui) — immediate mode GUI used to embed the widget.
* [`bevy_ratatui`]: [https://github.com/cxreiff/bevy_ratatui](https://github.com/cxreiff/bevy_ratatui) — Bevy integration for Ratatui.
* [`mousefood`]: (https://github.com/j-g00da/mousefood)
* [`ratzilla`]: (https://github.com/orhun/ratzilla)
* [`ratatui-wgpu`]: (https://github.com/Jesterhearts/ratatui-wgpu)
* [`bevy_ratatui_camera`]: (https://github.com/cxreiff/bevy_ratatui_camera)

WASM & platform guides:

* Bevy WASM guide: [https://bevy-cheatbook.github.io/platforms/wasm.html](https://bevy-cheatbook.github.io/platforms/wasm.html)
* Macroquad WASM: [https://macroquad.rs/articles/wasm/](https://macroquad.rs/articles/wasm/)
* eframe (egui desktop + wasm): [https://github.com/emilk/eframe_template](https://github.com/emilk/eframe_template)

Font and text engine links:

* [`embedded-graphics`](https://github.com/embedded-graphics/embedded-graphics)
* [`embedded_graphics_unicodefonts`](https://github.com/j-g00da/embedded-graphics-unicodefonts)
* [`bdf-parser`](https://github.com/embedded-graphics/bdf)
* [`embedded-ttf`](https://github.com/peckpeck/embedded-ttf)
* [`cosmic-text`](https://github.com/pop-os/cosmic-text)

---

## Cool BDF fonts

* [`spleen`]: [https://github.com/fcambus/spleen](https://github.com/fcambus/spleen) — many useful sizes.
* [`cozette`]: [https://github.com/the-moonwitch/Cozette](https://github.com/the-moonwitch/Cozette) — pretty font.

---

## License

Dual-licensed under **MIT** or **Apache 2.0**. Choose the license that suits you.

---

## Status & Contribution

Active development — issues, pull requests, and suggestions are very welcome. If you have a favorite font or backend you'd like supported, open an issue or a PR. Join the discussion on the project Discord: [https://discord.gg/tDBPkdgApN](https://discord.gg/tDBPkdgApN)

---

### Short FAQ

**Q:** Can I use `RataguiBackend` with multiple different raster backends in the same binary?
**A:** Yes — `RataguiBackend<R>` is generic over `R: RasterBackend`. Build and type-annotate the specific backend you want (e.g. `RataguiBackend<soft_ratatui::EmbeddedGraphics>`).

**Q:** Is this suitable for games?
**A:** Yes — the software backend produces RGB/RGBA pixmaps that you can upload as textures in game engines (Bevy examples provided).

**Q:** How do I deploy to the web?
**A:** Use egui/eframe or your engine's WASM tooling and enable the appropriate `soft_ratatui` features. See the WASM links above for platform-specific details.

---

If anything in this README is out-of-date with your current code, open a PR or an issue and I’ll help update it.
