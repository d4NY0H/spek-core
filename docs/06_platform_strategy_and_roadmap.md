## spek-core – Platform Strategy & Roadmap

---

## 1. Ziel dieses Dokuments

Dieses Dokument definiert **verbindlich**,  
wie `spek-core`:

- gebaut
- ausgeliefert
- eingebettet
- portiert

wird – **ohne** die Core-Architektur jemals zu verändern.

> Plattformen kommen und gehen.  
> Der Core bleibt.

---

## 2. Grundsatz: Core First, Platform Second

`spek-core` ist **kein Endanwenderprogramm**, sondern eine **Engine**.

Alle Plattformen sind:
- Adapter
- Wrapper
- Thin Layers

Kein Plattformcode darf:
- in den Core eindringen
- Core-Logik duplizieren
- Core-APIs verändern

---

## 3. Build-Strategie (grundlegend)

### 3.1 Sprache & ABI

- Implementierung: **Rust**
- Öffentliche Schnittstellen:
  - Rust API (intern)
  - **C ABI** (extern)

Warum C ABI:
- stabil
- universell
- JNI-fähig
- Swift-fähig
- Python-fähig

---

### 3.2 Build-Artefakte

`spek-core` kann gebaut werden als:

- `staticlib`
- `cdylib`
- `rlib`

Je nach Plattform.

---

## 4. Plattform 1: Linux (Phase 1 – Pflicht)

### 4.1 Ziel

- Headless Nutzung
- CI / Server / Colab
- Reproduzierbare PNGs

---

### 4.2 Deliverables

- `libspek_core.so` / `libspek_core.a`
- CLI-Tool (`spek-core-cli`)
- Keine GUI
- Keine Abhängigkeit von Display-Servern

---

### 4.3 CLI-Rolle

Die CLI ist **kein Core**, sondern:

- Referenz-Frontend
- Debug-Werkzeug
- Batch-Interface

Beispiel:

```bash
spek-core \
  input.wav \
  --out output.png \
  --width 4096 \
  --height 1024 \
  --scale log \
  --window hann
```


---

### 4.4 Linux als Referenzplattform

Linux ist:

- erste Zielplattform  
- technische Referenz  
- Basis für alle weiteren Ports  

Alles, was unter Linux nicht sauber läuft,  
wird nicht portiert.

---

## 5. Plattform 2: Android (Phase 2 – strategisch)

### 5.1 Ziel

- Nutzung auf Mobilgeräten  
- Offline-Analyse  
- Einbettung in eigene Apps  

---

### 5.2 Architektur

Android App  
  ↓ JNI  
C ABI (spek-core)  
  ↓  
Rust Core

---

### 5.3 JNI-Anforderungen

- Keine Panics über FFI-Grenzen  
- Klar definierte Fehlercodes  
- Speicher explizit allokiert / freigegeben  
- Keine globalen Zustände  

---

### 5.4 Output

- RGBA Bitmap  
- ByteBuffer  
- Optional: direkter PNG-Export  

---

### 5.5 Kein Android-spezifischer Code im Core

❌ Kein ndk  
❌ Kein android_log  
❌ Kein Threading durch Java  

Android ist reiner Konsument.

---

## 6. Plattform 3: iPadOS / iOS (Phase 3 – optional)

### 6.1 Ziel

- Nutzung in Analyse-Apps  
- Offline-Rendering  
- Kein App-Store-Zwang für Core  

---

### 6.2 Architektur

Swift / Objective-C  
  ↓ C ABI  
spek-core

---

### 6.3 Rendering

Core liefert Pixel.

UI entscheidet:

- UIKit  
- SwiftUI  
- Metal  
- CoreGraphics  

---

### 6.4 Einschränkungen

- Speicherlimits beachten  
- Chunked Processing möglich  
- Keine dynamischen Codepfade im Core  

---

## 7. Nicht-Zielplattformen (bewusst ausgeschlossen)

`spek-core` wird nicht:

- als DAW-Plugin gebaut  
- als WebAssembly initial entwickelt  
- als Live-Visualizer optimiert  
- als Streaming-Engine betrieben  

Diese Dinge können später entstehen,  
aber nicht im Scope dieses Projekts.

---

## 8. Versionierung & Stabilität

### 8.1 API-Stabilität

- SemVer  
- Breaking Changes nur bei Major-Versionen  
- C ABI besonders konservativ  

---

### 8.2 Reproduzierbarkeit

Ein Build von Version X.Y.Z muss:

- auf allen Plattformen  
- bei gleichen Inputs  
- identische Outputs erzeugen  

---

## 9. Roadmap (verbindlich)

**Phase 1 – Core stabilisieren**

- Linux Build  
- CLI  
- Test-Suite  
- PNG-Vergleiche  

**Phase 2 – Android**

- JNI-Bindings  
- Example App  
- Performance-Tuning  

**Phase 3 – Optional Extensions**

- iPadOS  
- Python Binding  
- Batch Tools  

---

## 10. Projekt-Leitsatz

> spek-core ist kein Produkt.  
> Es ist eine Grundlage.

Alles andere ist austauschbar.
