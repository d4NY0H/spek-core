# spek-core – Project Vision

## 1. Ziel des Projekts

`spek-core` ist eine **plattformunabhängige, UI-lose Kernbibliothek**
zur **deterministischen Erzeugung hochwertiger Audio-Spektrogramme
inklusive vollständiger Legende**.

Das Projekt ist **kein Fork von spek-rs**, sondern ein **eigenständiger Core**,
der Teile aus spek-rs **konzeptionell und selektiv** übernimmt.

---

## 2. Warum kein spek-rs-Fork?

`spek-rs` ist:
- stark an **eframe / egui** gekoppelt
- UI-zentriert
- historisch gewachsen
- nicht für Mobile oder Headless-Betrieb gedacht

`spek-core` hingegen ist:
- **headless first**
- **deterministisch**
- **plattformneutral**
- **UI-agnostisch**
- **mobilfähig**

👉 Die UI von spek-rs wird **nicht benötigt**, **nicht portiert**
und ist **kein Bestandteil** von spek-core.

---

## 3. Grundprinzipien von spek-core

### 3.1 Headless by Design

- Kein GUI-Code
- Keine Fenster
- Kein Event-Loop
- Keine UI-Abhängigkeiten

**Eingabe:**
- Audiodatei oder Audio-Stream

**Ausgabe:**
- RGBA-Pixelbuffer **oder**
- PNG (optional)

---

### 3.2 Die Legende ist immer Teil des Outputs

- **Keine ffmpeg-Legende**
- **Keine optionale Legende**
- **Kein Spektrogramm ohne Skalen**

Zeit-, Frequenz- und dB-Achsen sind **untrennbarer Bestandteil**
des finalen Bildes.

Ein Output **ohne vollständige Legende gilt als ungültig**.

---

### 3.3 Deterministischer Output

Gleiche Eingabe + gleiche Parameter ⇒ **bit-identischer Output**

Kein:
- Frame-Jitter
- Live-Re-Rasterizing
- UI-abhängiges Rendering
- nicht-deterministisches Verhalten

Determinismus ist Voraussetzung für:
- automatisierte Tests
- CI-basierte Bildverifikation
- plattformübergreifende Validierung

---

### 3.4 Klare, stabile Core-API

`spek-core` stellt eine **explizite, funktionale API** bereit:

- keine globalen Zustände
- keine impliziten Defaults
- alle Parameter sind explizit
- keine Abhängigkeit von UI oder Dateisystem

Die API ist so gestaltet, dass sie:
- direkt aus CLI-Tools nutzbar ist
- über **FFI / C ABI** von anderen Sprachen aufgerufen werden kann
- unverändert auf Desktop und Mobile einsetzbar bleibt

---

## 4. Rolle von ffmpeg

`spek-core` ist **kein Ersatz für ffmpeg**.

- ffmpeg kann optional zur **Audio-Dekodierung** genutzt werden
- der Core ist **konzeptionell nicht an ffmpeg gebunden**
- Signalverarbeitung und Rendering sind **vollständig im Core definiert**

Dies erlaubt spätere Alternativen (z. B. MediaCodec auf Android),
ohne den Core neu zu entwerfen.

---

## 5. Zielplattformen (in Reihenfolge)

### Phase 1 – Linux (CLI)
- Headless
- Google Colab
- CI-fähig
- PNG-Export

### Phase 2 – Android
- Rust → C ABI / JNI
- Bitmap- oder RGBA-Output
- Optional: Streaming-API

### Phase 3 – iPadOS (optional)
- Rust → C ABI
- Swift-/Metal-Frontend möglich
- Core bleibt unverändert

---

## 6. Was spek-core **nicht** ist

- ✗ Kein GUI-Programm
- ✗ Kein Audio-Editor
- ✗ Kein Live-Visualizer
- ✗ Kein DAW-Plugin
- ✗ Kein Ersatz für ffmpeg

`spek-core` ist **eine Rechen- und Render-Engine**, sonst nichts.

---

## 7. Erfolgskriterium (Definition of Done)

`spek-core` gilt als erfolgreich, wenn:

- ein **< 500 LOC Core** existiert
- ohne GUI
- mit stabiler API
- mit reproduzierbarem Output
- lauffähig unter Linux
- portierbar nach Android (FFI-fähig)

