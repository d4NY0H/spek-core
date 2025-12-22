## spek-core – Signal-Pipeline

---

## 1. Ziel der Signal-Pipeline

Die Signal-Pipeline beschreibt den **reinen Rechenweg** von Audio-Rohdaten
hin zu einem **numerischen Spektrogramm**.

Sie ist der **zentrale Kern von spek-core** und vollständig:

- deterministisch
- zustandslos
- UI-unabhängig
- plattformneutral

Die Pipeline erzeugt **keine Bilder**, sondern **reine Daten**.

---

## 2. Gesamtübersicht

Audio Samples ↓ Channel Handling ↓ Windowing ↓ FFT ↓ Magnitude / Power ↓ dBFS Mapping ↓ Intensity Scaling ↓ Spectrogram Grid

**Output-Form:**

spectrogram[freq_bin][time_bin] -> intensity (float)

---

## 3. Eingangsdaten

### 3.1 Audio-Format

Die Pipeline arbeitet ausschließlich auf **PCM-Floatdaten**:

[-1.0 … +1.0]

**Vorbedingungen:**
- Sample Rate bekannt
- Channel Count bekannt
- Daten liegen vollständig im Speicher oder als Stream vor
- Interleaved oder planar → **vor Pipeline vereinheitlichen**

Die Pipeline selbst übernimmt **keine Dekodierung**.

---

## 4. Kanalbehandlung

### 4.1 Modi

**Combined**
- Alle Kanäle werden gemittelt
- Ein Spektrogramm

**Split**
- Jeder Kanal separat
- Pipeline liefert mehrere Spektrogramme

Vec<ChannelSpectrogram>

**Wichtig**
- Die Pipeline entscheidet **nicht**, wie Kanäle dargestellt werden
- Layout ist Aufgabe des Renderers

---

## 5. Windowing

### 5.1 Zweck

Windowing reduziert:
- Spectral Leakage
- harte Diskontinuitäten an Frame-Grenzen

### 5.2 Unterstützte Fenster

**Minimal erforderlich**
- Rectangular
- Hann
- Hamming
- Blackman

**Optional**
- Nuttall
- Kaiser
- FlatTop

### 5.3 Parameter

- Window Size (= FFT Size)
- Hop Size (Overlap)

Hop Size ist **konstant** über die gesamte Pipeline.

---

## 6. FFT

### 6.1 FFT-Größe

FFT Size = Window Size

Typische Werte:
- 1024
- 2048
- 4096

### 6.2 FFT-Ergebnis

FFT liefert komplexe Frequenzbins:

(re, im)

Es wird ausschließlich die **positive Frequenzhälfte** genutzt:

0 Hz → Nyquist

---

## 7. Magnitude & Power

### 7.1 Definitionen

**Magnitude**

sqrt(re² + im²)

**Power**

re² + im²

### 7.2 Entscheidung

`spek-core` verwendet **Power**, da:
- numerisch stabiler
- besser geeignet für Log-Skalierung
- konsistent mit dBFS

---

## 8. dBFS-Mapping

### 8.1 Referenz

- 0 dBFS = maximale Amplitude
- Untergrenze konfigurierbar (typisch: -120 dBFS)

### 8.2 Formel

dB = 10 * log10(power + ε)

`ε` verhindert log(0) und ist konstant.

---

## 9. Intensitätsskalierung

### 9.1 Zweck

Anpassung der Werte an:
- menschliche Wahrnehmung
- spätere visuelle Kontraste

### 9.2 Skalen

- Linear
- Sqrt
- Cbrt
- Log
- Custom Exponent

Beispiel:

intensity = pow(normalized_db, exponent)

---

## 10. Normalisierung

Nach Skalierung gilt:

0.0 ≤ intensity ≤ 1.0

Diese Werte sind:
- farbunabhängig
- pixelunabhängig
- renderer-agnostisch
- mehrfach verwendbar

---

## 11. Zeit-Frequenz-Gitter

### 11.1 Struktur

```rust
Spectrogram {
    time_bins: usize,
    freq_bins: usize,
    data: Vec<Vec<f32>>
}
```

### 11.2 Eigenschaften

- Frequenzachse: linear  
- Zeitachse: konstant (Hop Size)  
- Logarithmen werden an anderer Stelle angewendet

---

## 12. Abbruch & Fehlerbehandlung

Die Pipeline:

- akzeptiert ein Cancel-Token  
- bricht sauber ab  
- liefert kein partielles Ergebnis  

Fehler führen zu:

- klar definiertem Abbruch  
- keinem undefinierten Zustand  

---

## 13. Determinismus-Garantie

Für `spek-core` gilt:

> Gleiche Samples + gleiche Parameter  
> ⇒ bit-identisches Spektrogramm

Keine:

- Zufallsanteile  
- Zeitabhängigkeiten  
- Frame-Jitter  

---

## 14. Pipeline-Leitsatz

> Alles hier ist Mathematik.  
> Nichts hier weiß, wie es aussieht.
