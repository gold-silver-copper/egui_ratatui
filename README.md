# egui_ratatui : egui widget + ratatui backend (WASM)

[![Crates.io](https://img.shields.io/crates/v/egui_ratatui.svg)](https://crates.io/crates/egui_ratatui)
[![Documentation](https://docs.rs/egui_ratatui/badge.svg)](https://docs.rs/egui_ratatui/latest/egui_ratatui/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bevyengine/bevy/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/egui_ratatui.svg)](https://crates.io/crates/egui_ratatui)

Discord: [https://discord.gg/tDBPkdgApN](https://discord.gg/tDBPkdgApN)

Web demo: [https://gold-silver-copper.github.io/](https://gold-silver-copper.github.io/) (works best on Firefox)

See also:

* [https://ratatui.rs/](https://ratatui.rs/)
* [https://github.com/emilk/egui](https://github.com/emilk/egui)

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
use egui_ratatui::RataguiBackend;
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

// include a font (BDF, TTF atlas, or whichever backend you enable)
let font_bytes: &[u8] = include_bytes!("../assets/mono_8x13.bdf");

// annotate the concrete backend type if necessary:
let mut backend: RataguiBackend<soft_ratatui::EmbeddedGraphics> =
    RataguiBackend::new("my_ratatui", 16, font_bytes);

// Use it as any ratatui Backend:
let mut terminal = Terminal::new(&mut backend).unwrap();
terminal.clear().unwrap();

terminal.draw(|frame| {
    let area = frame.area();
    frame.render_widget(
        Paragraph::new("Hello egui_ratatui!")
            .block(Block::new().title("Ratatui").borders(Borders::ALL))
            .wrap(Wrap { trim: false }),
        area,
    );
}).unwrap();
```

### Embedding inside an `egui` UI

`RataguiBackend` implements `egui::Widget` for `&mut RataguiBackend<R>`, so you can insert it into any egui layout:

```rust
use egui::{CentralPanel, Context};
use egui_ratatui::RataguiBackend;

// create RataguiBackend as above
let font_bytes: &[u8] = include_bytes!("../assets/mono_8x13.bdf");
let mut rat_backend: RataguiBackend<soft_ratatui::EmbeddedGraphics> =
    RataguiBackend::new("my_ratatui", 16, font_bytes);

// inside your egui frame code:
fn ui_example(ctx: &egui::Context, rat_backend: &mut RataguiBackend<soft_ratatui::EmbeddedGraphics>) {
    CentralPanel::default().show(ctx, |ui| {
        // add the ratatui widget; it will render the backend's current pixmap as an egui image
        ui.add(rat_backend);
    });
}
```

When the panel changes size, `RataguiBackend` will resize its internal SoftBackend to match the available characters (based on `char_width` / `char_height`). The backend exposes functions to fetch the generated pixmap if you want to use it directly.

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
* [`mousefood`]: https://github.com/j-g00da/mousefood
* [`ratzilla`]: https://github.com/orhun/ratzilla
* [`ratatui-wgpu`]: https://github.com/Jesterhearts/ratatui-wgpu
* [`bevy_ratatui_camera`]: https://github.com/cxreiff/bevy_ratatui_camera

WASM & platform guides:

* Bevy WASM guide: [https://bevy-cheatbook.github.io/platforms/wasm.html](https://bevy-cheatbook.github.io/platforms/wasm.html)
* Macroquad WASM: [https://macroquad.rs/articles/wasm/](https://macroquad.rs/articles/wasm/)
* eframe (egui desktop + wasm): [https://github.com/emilk/eframe_template](https://github.com/emilk/eframe_template)

Font and text engine links:

* [`embedded-graphics`]: https://github.com/embedded-graphics/embedded-graphics
* [`embedded_graphics_unicodefonts`]: https://github.com/j-g00da/embedded-graphics-unicodefonts
* [`bdf-parser`]: https://github.com/embedded-graphics/bdf
* [`embedded-ttf`]: https://github.com/peckpeck/embedded-ttf
* [`cosmic-text`]: https://github.com/pop-os/cosmic-text

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
