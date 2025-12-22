## spek-core – Legend System

---

## 1. Rolle der Legende in spek-core

Die Legende ist **kein optionales Feature**, sondern ein **integraler Bestandteil**
des Outputs von `spek-core`.

Ein von spek-core erzeugtes Bild ist **immer vollständig beschriftet**.

> **Kein Spektrogramm ohne Kontext.**

Zeit-, Frequenz- und Pegelinformationen sind **untrennbar** mit der Visualisierung
verbunden.

---

## 2. Grundprinzip

Die Legende wird als **separater, deterministischer Rendering-Schritt**
nach dem Spektrogramm-Rendering ausgeführt.

Pipeline (vereinfacht):

Audio → Analyse → Spektrogramm → Rendering → Legend Overlay → Final Image

Die Legende:
- liest Metadaten
- liest Rendering-Parameter
- schreibt **nur Pixel**
- kennt keine UI
- kennt keine Plattform

---

## 3. Bestandteile der Legende

Die Legende besteht aus **vier klar definierten Bereichen**:

1. Zeitachse
2. Frequenzachse
3. Pegelskala (dBFS)
4. Kopfzeile (Metadaten)

Jeder Bereich ist **optional konfigurierbar**, aber **standardmäßig aktiv**.

---

## 4. Zeitachse

### 4.1 Position

- Standard: **unterhalb** des Spektrogramms
- Optional: zusätzlich **oberhalb** (ohne Labels)

### 4.2 Inhalte

- Ticks in festen Intervallen
- Beschriftung im Format:

mm:ss

### 4.3 Berechnung

- Dauer aus `AudioInfo.duration`
- Tick-Anzahl abhängig von Bildbreite
- Gleichmäßige Verteilung

### 4.4 Eigenschaften

- Linear
- Keine Rundungsfehler > 1 Pixel
- Keine Überlappung der Labels

---

## 5. Frequenzachse

### 5.1 Position

- Standard: **links** vom Spektrogramm
- Optional: zusätzlich **rechts** (ohne Labels)

### 5.2 Skala

- Anzeige in **kHz**
- 0 Hz → Nyquist-Frequenz

### 5.3 Logik

- Beschriftung folgt der **Darstellung**, nicht der Pipeline
- Bei log-rendering:
- Labels folgen log-Abständen
- Bei linear-rendering:
- Gleichmäßige Abstände

### 5.4 Multi-Channel

- Bei Split-Channels:
- Jede Kanalfläche erhält eigene Skala
- Maximalwert wird nicht doppelt beschriftet (Überlappungsschutz)

---

## 6. Pegelskala (dBFS)

### 6.1 Zweck

Die Pegelskala erklärt die **Farbintensität** des Spektrogramms.

Ohne Pegelskala ist das Bild **interpretationslos**.

---

### 6.2 Wertebereich

- Obergrenze: `0 dBFS`
- Untergrenze: typischerweise `-120 dBFS`

Diese Grenzen sind:
- fest definiert
- nicht dynamisch
- identisch auf allen Plattformen

---

### 6.3 Darstellung

- Vertikale Skala
- Textlabels:

0 -20 -40 -60 -80 -100 -120

- Zusätzlich:
- Farbgradient (Palette)
- Sättigung berücksichtigt

---

## 7. Kopfzeile (Header)

### 7.1 Inhalte

Die Kopfzeile kann enthalten:

- Dateiname
- Audioformat (Codec oder Container)
- Sample Rate
- Bit Depth
- Kanalanzahl
- Analyseparameter (optional)

Beispiel:

FLAC, 44100 Hz, 16 bit, Stereo, Hann, Log

---

### 7.2 Layout

- Oben links: Dateiname
- Darunter: technische Metadaten
- Oben rechts: Programminfo (optional)

---

## 8. Typografie

### 8.1 Schrift

- Serifenlose Standardschrift
- Unicode-fähig
- Monospace **nicht erforderlich**

### 8.2 Font-Fallback

- Wenn Glyph fehlt:
  - automatische Fallback-Suche
- Kein Abbruch bei fehlenden Glyphen
- Fehlende Zeichen werden still ersetzt

---

## 9. Layout & Abstände

### 9.1 Margins

Standardwerte (Beispiel):

TOP    = 64 px BOTTOM = 64 px LEFT   = 80 px RIGHT  = 100 px

Diese Werte:
- sind deterministisch
- können konfigurierbar sein
- ändern **nie** das Spektrogramm selbst

---

### 9.2 Überlappungsschutz

Die Legende:
- kürzt Texte bei Platzmangel
- nutzt Ellipsen (`...`)
- verschiebt niemals das Spektrogramm unkontrolliert

---

## 10. Rendering-Reihenfolge

1. Hintergrund (schwarz)
2. Spektrogramm (fertig gerendert)
3. Achsenlinien
4. Ticks
5. Textlabels
6. Farbskala

Diese Reihenfolge ist **fest**.

---

## 11. Determinismus

Die Legende ist vollständig deterministisch:

- Gleiche AudioInfo
- Gleiche Bildgröße
- Gleiche Parameter

⇒ **Bit-identische Pixel**

Keine:
- Layout-Abhängigkeit vom OS
- DPI-Skalierung
- Subpixel-Text-Rasterisierung

---

## 12. Abgrenzung

Die Legende:

❌ nutzt keine ffmpeg-Legende  
❌ nutzt keine UI-Fonts  
❌ nutzt kein Betriebssystem-Layout  
❌ kennt keine Interaktion  

Sie ist **reines Pixel-Rendering**.

---

## 13. Konfigurierbarkeit

Die API erlaubt:

- Ein/Aus einzelner Legendenteile
- Schriftgröße (diskret)
- Tick-Dichte
- Farbschema-Anpassung

Aber:
> Die Legende als Konzept kann **nicht deaktiviert** werden.

---

## 14. Legend-Leitsatz

> Eine Visualisierung ohne Legende ist Dekoration.  
> spek-core erzeugt Analyse.
