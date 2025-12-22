## spek-core – Core-Architektur

---

## 1. Architektur-Ziel

`spek-core` besitzt eine **kleine, klar getrennte Kernarchitektur**, die:

- **ohne UI**
- **ohne Plattformannahmen**
- **ohne globale Zustände**
- **ohne versteckte Nebenwirkungen**

arbeitet.

Die Architektur folgt strikt dem Prinzip:

> **Audio rein → Daten transformieren → Bild raus**

Kein Modul kennt:
- Fenster
- Events
- UI-Frameworks
- Plattformdetails
- Lebenszyklen anderer Module

---

## 2. Überblick: Modulstruktur

Die finale Core-Struktur besteht aus **maximal sechs Modulen**:

spek-core/ 
├── audio/ 
├── analysis/ 
├── color/ 
├── render/ 
├── legend/ 
└── api/

Jedes Modul hat **eine klar abgegrenzte Verantwortung**  
und **keine zyklischen Abhängigkeiten**.

Abhängigkeiten verlaufen **nur nach unten**, niemals seitlich.

---

## 3. Modulverantwortlichkeiten

---

### 3.1 `audio/` – Audio-Zugriff & Metadaten

**Aufgabe**
- Öffnen von Audiodateien oder Streams
- Extraktion von Metadaten
- Bereitstellung von PCM-Samples

**Beinhaltet**
- Duration
- Sample Rate
- Channels
- Bit Depth
- Kanal-Layout
- PCM-Stream (interleaved oder planar)

**Explizit nicht enthalten**
- FFT
- Analyse
- Rendering
- Farben
- Legenden

**Architektur-Regel**
> `audio/` liefert **nur rohe Daten** – keine Interpretation.

---

### 3.2 `analysis/` – Signalverarbeitung

**Aufgabe**
- Fensterfunktionen
- FFT
- Magnitude / Power
- dBFS-Normalisierung
- Skalierungen (lin, log, sqrt, …)

**Output**
- Reines **numerisches Spektrogramm**

Beispielhafte Struktur:

spectrogram[freq_index][time_index] -> intensity (float)

**Eigenschaften**
- Keine Pixel
- Keine Farben
- Kein Wissen über Zeit- oder Frequenzbeschriftungen
- Deterministischer Output

**Architektur-Regel**
> `analysis/` kennt **nur Mathematik**, keine Darstellung.

---

### 3.3 `color/` – Farbraum & Paletten

**Aufgabe**
- Mapping von Intensitätswerten → Farbe
- Unterstützung verschiedener Farbräume
- Sättigungslogik
- Palette-Interpolation

**Input**
- Normalisierte Intensität (0.0 – 1.0)

**Output**
- RGBA oder YUV-Farbwerte

**Explizit nicht enthalten**
- Zeit
- Frequenz
- Pixelposition
- Texte

**Architektur-Regel**
> `color/` ist eine **reine Transferfunktion**:  
> Zahl → Farbe

---

### 3.4 `render/` – Bildaufbau

**Aufgabe**
- Umwandlung des Spektrogramm-Gitters in ein Bild
- Pixelgenauer Aufbau
- Kanal-Splitting
- Orientierung (vertikal / horizontal)

**Input**
- Numerisches Spektrogramm
- Farbmapper (`color/`)

**Output**
- Reiner Bildbuffer (RGBA)

**Explizit nicht enthalten**
- Texte
- Achsen
- Metadaten
- Legenden

**Architektur-Regel**
> `render/` erzeugt **nur Bildinhalt**, keine Beschriftung.

---

### 3.5 `legend/` – Achsen & Beschriftungen

**Aufgabe**
- Zeitachse
- Frequenzachse
- dBFS-Skala
- Titel / Metadaten
- Schrift-Fallbacks

**Eigenschaften**
- Die Legende ist **immer aktiv**
- Kein optionaler Modus
- Keine ffmpeg-Legende
- Pixelgenaue Überlagerung

**Reihenfolge**
1. Render erzeugt das reine Spektrogramm
2. `legend/` überlagert Skalen und Texte

**Architektur-Regel**
> Ein Bild **ohne Legende existiert in spek-core nicht**.

---

### 3.6 `api/` – Öffentliche Schnittstelle

**Aufgabe**
- Einheitliche, stabile API für:
  - CLI
  - Android (JNI)
  - iPadOS (C ABI)

**Beispielhafte API**

```rust
generate_spectrogram(
    input_audio,
    settings,
) -> ImageBuffer
```

**Eigenschaften**

- Keine UI  
- Keine Plattformlogik  
- Keine Threads sichtbar nach außen  
- Keine globalen Zustände  

**Architektur-Regel**

> Alles, was von außen sichtbar ist, liegt in `api/`.

---

## 4. Threading-Modell

**Grundsatz**  
`spek-core` ist single-shot, nicht eventbasiert.

- Ein Aufruf → ein Ergebnis  

**Optional**

- Abbruch-Token

**Intern**

- Audio lesen → sequentiell  
- FFT → optional parallelisiert  
- Rendering → sequentiell  
- Legend → sequentiell  

**Explizit ausgeschlossen**

- Live-Streaming im Core  
- UI-Threads  
- Channels nach außen  

---

## 5. Abhängigkeiten (bewusst minimal)

**Erlaubt**

- FFT-Bibliothek  
- Image-Buffer  
- Font-Rasterizer (für Legend)  

**Nicht erlaubt**

- GUI-Frameworks  
- Windowing  
- Event-Loops  
- Plattform-spezifische APIs  

---

## 6. Architektur-Leitsatz

> `spek-core` weiß nicht, wo es läuft.  
> Es weiß nur, was es berechnet.

