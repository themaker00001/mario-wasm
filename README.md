# 🍄 Super Mario WASM

A classic side-scrolling platformer recreation of Super Mario (World 1-1) written in **100% Rust**, compiled to **WebAssembly (WASM)**, and playable directly in the browser via **GitHub Pages**.

🎮 **[Play Live on GitHub Pages](https://themaker00001.github.io/mario-wasm/)**

---

## ✨ Features

- **Pure Rust Game Engine**: Platformer physics, tile collisions, entity state machines, sprite blitting, and HUD rendering are all computed natively in WebAssembly at a rock-solid 60 FPS.
- **Classic World 1-1**:
  - Breakable brick blocks and mystery `?` question blocks
  - Super Mushrooms that transform Small Mario into Super Mario (with variable jumping and brick-smashing abilities)
  - Coins, spinning coin animations, and 1-Up counter
  - Goombas with stomping & squishing mechanics
  - Koopa Troopas that retreat into shells that can be kicked to wipe out rows of enemies
  - Classic green pipes, bottomless pits, staircase pyramid, and the iconic Flagpole & Castle sequence!
- **8-Bit Chiptune Audio Synthesizer**: Procedural retro audio generated on-the-fly using the Web Audio API — jump sweeps, stomp pops, coin chimes, pipe thuds, power-up arpeggios, death jingles, and a faithful chiptune background theme loop. No external audio files needed!
- **Responsive Controls**:
  - Full desktop keyboard support (Arrow keys / WASD, Space / Z to jump, Shift / X to sprint dash).
  - Virtual retro D-pad and A/B touch controls for mobile phones and tablets.
  - Fullscreen toggle and audio mute switch.

---

## 🕹️ Controls

| Action | Keyboard | Touch / Mobile |
| :--- | :--- | :--- |
| **Move Left / Right** | `←` `→` or `A` `D` | Virtual D-Pad Left / Right |
| **Jump** (hold for higher hop) | `Space`, `Z`, or `↑` | Virtual `A` Button |
| **Duck** (Super Mario) | `↓` or `S` | Virtual D-Pad Down |
| **Sprint / Dash** | `Shift` or `X` | Virtual `B` Button |
| **Start / Pause / Restart** | `Enter` or `Space` | Virtual `START` Button |
| **Mute / Unmute Audio** | `M` | Top Bar Sound Button |

---

## 🛠️ Architecture & Tech Stack

```
┌────────────────────────────────────────────────────────┐
│                      Web Browser                       │
│                                                        │
│  ┌─────────────────────────┐  ┌─────────────────────┐  │
│  │       HTML5 Canvas      │  │    Web Audio API    │  │
│  │  (320x240 Pixel Buffer) │  │  (Chiptune Synths)  │  │
│  └────────────▲────────────┘  └──────────▲──────────┘  │
│               │                          │             │
│               └───────────┬──────────────┘             │
│                           │ JS Glue Layer              │
│                           │ (main.js / audio.js)       │
│                           │                            │
│  ┌────────────────────────▼─────────────────────────┐  │
│  │           WebAssembly Engine (mario.wasm)        │  │
│  │                                                  │  │
│  │   • Rust Platformer Physics & AABB Collisions   │  │
│  │   • Tilemap & World 1-1 Layout                   │  │
│  │   • Entity System (Mario, Goomba, Koopa, Items)  │  │
│  │   • 16x16 / 16x32 Pixel Art Sprite Rasterizer   │  │
│  │   • Dynamic HUD & Overlay State Machine          │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────┘
```

- **Language**: Rust 2021 edition
- **Compilation Target**: `wasm32-unknown-unknown`
- **Output Binary Size**: ~350 KB (self-contained, 0 external runtime dependencies)
- **Deployment**: Automatic GitHub Pages via GitHub Actions

---

## 🚀 Building & Running Locally

### Prerequisites
- [Rust & Cargo](https://rustup.rs/)
- `wasm32-unknown-unknown` target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

### Build Wasm Binary
```bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/mario_wasm.wasm ./mario.wasm
```

### Run Local HTTP Server
```bash
python3 -m http.server 8080
```
Open `http://localhost:8080` in your web browser!

---

## 👤 Author

- **GitHub**: [@themaker00001](https://github.com/themaker00001)
- **Email**: the.maker00001@gmail.com
