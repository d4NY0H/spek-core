 # spek-core

**spek-core** is a **headless, platform-agnostic Rust core** for generating
**deterministic, high-quality audio spectrograms with a full legend**.

It is designed as a **pure computation and rendering engine**:
no UI, no windows, no event loop, no platform assumptions.

---

## What spek-core is

- A **pure spectrogram computation core**
- **Headless-first** and **deterministic**
- Produces **pixel-perfect, reproducible images**
- Always renders a **complete legend** (time, frequency, dBFS)
- Designed for **Linux, Android, and iPadOS**

---

## What spek-core is NOT

- ❌ No GUI application
- ❌ No desktop UI framework (egui, Qt, GTK, etc.)
- ❌ No audio player
- ❌ No live visualizer
- ❌ No DAW plugin
- ❌ No ffmpeg replacement

`spek-core` is a **library**, not an app.

---

## Design principles

- **Audio in → data → image out**
- **Legend is mandatory**
- **No ffmpeg legend**
- **No UI logic**
- **No platform-specific code**
- **Same input + same settings = identical output**

---

## Architecture overview

`spek-core` is split into **small, clearly separated modules**:

- `audio` – audio access & metadata
- `analysis` – signal processing (FFT, scaling, dBFS)
- `color` – intensity → color mapping
- `render` – pixel-level image construction
- `legend` – axes, labels, scales, text rendering
- `api` – stable public interface (CLI, JNI, C ABI)

Each module has **one responsibility** and no hidden dependencies.

---

## Output model

`spek-core` produces:

- A **numerical spectrogram grid** (internal)
- A **final RGBA image buffer**
- Optional **PNG output**

The legend (time, frequency, dBFS) is **always part of the output**.

---

## Platform strategy

### Phase 1 – Linux / CLI (primary target)

- Headless CLI
- Google Colab compatible
- CI-friendly
- Deterministic PNG output

### Phase 2 – Android

- Rust core via **C ABI**
- Thin JNI wrapper
- Bitmap output
- No Android-specific logic in core

### Phase 3 – iPadOS (optional)

- Rust → C ABI
- Swift / Metal frontend possible
- Core remains unchanged

---

## Relationship to spek-rs

`spek-core` is **not a fork** of `spek-rs`.

- `spek-rs` is UI-centric and desktop-focused
- `spek-core` is UI-less and platform-neutral

Some concepts are reused **selectively**, but the architecture is independent.

---

## Project documents

The full design is specified in six core documents:

- `docs/01_core_vision.md`
- `docs/02_core_architecture.md`
- `docs/03_signal_pipeline.md`
- `docs/04_rendering_model.md`
- `docs/05_legend_system.md`
- `docs/06_platform_strategy_and_roadmap.md`

These documents define the project completely.

---

## Status

- Design: **Final**
- Architecture: **Frozen**
- Implementation: **Pending**
- First target: **Linux CLI**

---

## License

(To be defined)

---

