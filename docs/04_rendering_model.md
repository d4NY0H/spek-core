## spek-core – Rendering Model

---

## 1. Ziel des Rendering-Modells

Das Rendering-Modell beschreibt den **deterministischen Übergang**
vom **numerischen Spektrogramm** (reine Daten)
zu einem **pixelgenauen Bildbuffer**.

Es ist vollständig:
- UI-unabhängig
- plattformneutral
- zustandslos
- reproduzierbar

Das Rendering kennt **keine Fenster**, **keine Events** und **keine Legenden**.

---

## 2. Eingabe & Ausgabe

### 2.1 Eingabe

- `Spectrogram` (aus der Signal-Pipeline)
- Rendering-Parameter:
  - Bildbreite / -höhe (in Pixeln)
  - Orientierung (vertikal / horizontal)
  - Kanalmodus (combined / split)
  - Farbschema (Palette)
  - Sättigung

### 2.2 Ausgabe

- Reiner **RGBA-Buffer**
- Kein PNG, kein Dateiformat
- Kein Alpha-Blending mit Fremdinhalten

ImageBuffer { width: usize, height: usize, pixels: Vec<RGBA> }

---

## 3. Grundprinzip

Das Rendering folgt strikt:

> **Ein Datenpunkt → ein Pixelbereich**

Es gibt:
- kein Resampling während des Renderns
- kein dynamisches Skalieren
- keine UI-bedingten Anpassungen

Die Bildgröße bestimmt **die Anzahl der Zeit- und Frequenz-Pixel**.

---

## 4. Zeit-Achse (X-Dimension)

### 4.1 Zuordnung

- Zeitbins werden **linear** auf die Bildbreite gemappt
- Jeder Zeitbin entspricht exakt einer Spalte oder einem Spaltenblock

Beispiel:

image_width = time_bins

oder bei Skalierung:

time_bin_width = image_width / time_bins

### 4.2 Eigenschaften

- Keine Interpolation zwischen Zeitbins
- Keine Glättung
- Keine Antialiasing-Logik

---

## 5. Frequenz-Achse (Y-Dimension)

### 5.1 Zuordnung

- Frequenzbins werden **linear** auf die Bildhöhe gemappt
- Niedrige Frequenzen unten, hohe oben (Standard)

freq 0 Hz → bottom Nyquist → top

### 5.2 Logarithmische Darstellung

- Logarithmische Skalen sind **kein Teil der Signal-Pipeline**
- Log-Mapping erfolgt **ausschließlich im Rendering**

Beispiel:

y = log_map(freq_bin)

---

## 6. Kanal-Layout

### 6.1 Combined

- Ein Spektrogramm
- Vollständige Bildhöhe

### 6.2 Split Channels

- Jeder Kanal erhält einen festen Bildbereich
- Typisch: vertikale Aufteilung

Beispiel (Stereo):

Top    → Left Channel Bottom → Right Channel

**Wichtig**
- Kanaltrennung ist geometrisch
- Keine Farb- oder Intensitätsmodifikation

---

## 7. Farbzuordnung

### 7.1 Input

- Intensitätswert ∈ [0.0 … 1.0]

### 7.2 Mapping

- Intensität → Palette → Farbe
- Farbmodell unabhängig vom Spektrogramm

color = palette.map(intensity, saturation)

### 7.3 Eigenschaften

- Keine Kenntnis von Zeit oder Frequenz
- Kein Clipping außerhalb der Palette
- Identisches Mapping für alle Plattformen

---

## 8. Pixel-Füllstrategie

### 8.1 Einzelpixel

- 1 Datenpunkt → 1 Pixel

### 8.2 Pixelblöcke

Falls Bildgröße ≠ Bin-Anzahl:
- Jeder Datenpunkt füllt einen rechteckigen Block
- Blockgröße ist konstant

**Keine:**
- bilineare Interpolation
- Kantenglättung
- Filter

---

## 9. Orientierung

### 9.1 Vertikal (Standard)

- Zeit: links → rechts
- Frequenz: unten → oben

### 9.2 Horizontal (optional)

- Zeit: unten → oben
- Frequenz: links → rechts

Orientierung ist eine **reine Koordinatentransformation**.

---

## 10. Performance-Charakteristik

- Rendering ist O(width × height)
- Keine Allokationen im inneren Loop
- SIMD optional, aber nicht erforderlich
- Parallelisierung möglich, aber nicht verpflichtend

---

## 11. Fehlerfreiheit & Stabilität

Das Rendering:
- kann nicht abstürzen durch Datenwerte
- akzeptiert nur normalisierte Intensitäten
- produziert niemals NaN oder Inf im Bild

---

## 12. Determinismus

Für das Rendering gilt:

> Gleiche Daten + gleiche Parameter  
> ⇒ **bit-identischer RGBA-Buffer**

Keine:
- Plattformabhängigkeit
- GPU-Zustände
- Floating-Point-Drift durch Reihenfolge

---

## 13. Abgrenzung zur Legende

Das Rendering:
- zeichnet **keine Achsen**
- zeichnet **keine Texte**
- kennt **keine Metadaten**

Die Legende wird **nachgelagert** als eigener Schritt angewendet.

---

## 14. Rendering-Leitsatz

> Rendering ist Geometrie + Farbe.  
> Alles andere ist nicht seine Aufgabe.
