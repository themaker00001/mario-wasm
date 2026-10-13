// Chiptune Retro Audio Synthesizer via Web Audio API
class AudioManager {
    constructor() {
        this.ctx = null;
        this.isMuted = false;
        this.bgmPlaying = false;
        this.bgmTimer = null;
        this.bgmStep = 0;
    }

    init() {
        if (!this.ctx) {
            const AudioContext = window.AudioContext || window.webkitAudioContext;
            this.ctx = new AudioContext();
        }
        if (this.ctx && this.ctx.state === 'suspended') {
            this.ctx.resume();
        }
    }

    toggleMute() {
        this.isMuted = !this.isMuted;
        if (this.isMuted) {
            this.stopBGM();
        } else {
            this.startBGM();
        }
        return this.isMuted;
    }

    // Play retro square/pulse wave tone with envelope
    playTone(freq, type = 'square', duration = 0.1, gainVal = 0.15, freqEnd = null) {
        if (this.isMuted || !this.ctx) return;
        try {
            const now = this.ctx.currentTime;
            const osc = this.ctx.createOscillator();
            const gain = this.ctx.createGain();

            osc.type = type;
            osc.frequency.setValueAtTime(freq, now);
            if (freqEnd !== null) {
                osc.frequency.exponentialRampToValueAtTime(Math.max(10, freqEnd), now + duration);
            }

            gain.gain.setValueAtTime(gainVal, now);
            gain.gain.exponentialRampToValueAtTime(0.001, now + duration);

            osc.connect(gain);
            gain.connect(this.ctx.destination);

            osc.start(now);
            osc.stop(now + duration);
        } catch (e) {
            console.warn(e);
        }
    }

    // Play white noise burst (for stomps, kicks, breakages)
    playNoise(duration = 0.1, gainVal = 0.2) {
        if (this.isMuted || !this.ctx) return;
        try {
            const bufferSize = Math.floor(this.ctx.sampleRate * duration);
            const buffer = this.ctx.createBuffer(1, bufferSize, this.ctx.sampleRate);
            const data = buffer.getChannelData(0);
            for (let i = 0; i < bufferSize; i++) {
                data[i] = Math.random() * 2 - 1;
            }

            const noise = this.ctx.createBufferSource();
            noise.buffer = buffer;

            const filter = this.ctx.createBiquadFilter();
            filter.type = 'lowpass';
            filter.frequency.setValueAtTime(800, this.ctx.currentTime);

            const gain = this.ctx.createGain();
            const now = this.ctx.currentTime;
            gain.gain.setValueAtTime(gainVal, now);
            gain.gain.exponentialRampToValueAtTime(0.001, now + duration);

            noise.connect(filter);
            filter.connect(gain);
            gain.connect(this.ctx.destination);

            noise.start(now);
            noise.stop(now + duration);
        } catch (e) {
            console.warn(e);
        }
    }

    // Handle sound event flags from WebAssembly
    handleSoundEvents(flags) {
        if (!flags || this.isMuted) return;
        this.init();

        // Check individual sound bitflags
        if (flags & (1 << 1)) this.playJumpSmall();
        if (flags & (1 << 2)) this.playJumpSuper();
        if (flags & (1 << 3)) this.playCoin();
        if (flags & (1 << 4)) this.playStomp();
        if (flags & (1 << 5)) this.playPowerupSpawn();
        if (flags & (1 << 6)) this.playPowerup();
        if (flags & (1 << 7)) this.playBump();
        if (flags & (1 << 8)) this.playBreak();
        if (flags & (1 << 9)) this.playPipe();
        if (flags & (1 << 10)) this.playDie();
        if (flags & (1 << 11)) this.playFlagpole();
        if (flags & (1 << 12)) this.playStageClear();
        if (flags & (1 << 13)) this.playKick();
        if (flags & (1 << 15)) this.playGameOver();
    }

    playJumpSmall() {
        this.playTone(160, 'square', 0.18, 0.15, 450);
    }

    playJumpSuper() {
        this.playTone(130, 'square', 0.22, 0.18, 520);
    }

    playCoin() {
        if (this.isMuted || !this.ctx) return;
        const now = this.ctx.currentTime;
        // B5 (987Hz) then E6 (1319Hz)
        this.playTone(987, 'square', 0.08, 0.18);
        setTimeout(() => {
            this.playTone(1319, 'square', 0.28, 0.22);
        }, 75);
    }

    playStomp() {
        this.playTone(180, 'square', 0.12, 0.2, 50);
        this.playNoise(0.08, 0.15);
    }

    playKick() {
        this.playTone(220, 'square', 0.15, 0.22, 60);
        this.playNoise(0.12, 0.25);
    }

    playBump() {
        this.playTone(120, 'triangle', 0.1, 0.2, 70);
    }

    playBreak() {
        this.playNoise(0.2, 0.3);
        this.playTone(140, 'square', 0.15, 0.15, 50);
    }

    playPowerupSpawn() {
        if (this.isMuted || !this.ctx) return;
        const notes = [330, 392, 659, 523, 587, 784];
        notes.forEach((freq, i) => {
            setTimeout(() => {
                this.playTone(freq, 'triangle', 0.08, 0.15);
            }, i * 40);
        });
    }

    playPowerup() {
        if (this.isMuted || !this.ctx) return;
        // Classic ascending powerup arpeggio
        const notes = [330, 392, 659, 523, 587, 784, 880, 1046];
        notes.forEach((freq, i) => {
            setTimeout(() => {
                this.playTone(freq, 'square', 0.09, 0.16);
            }, i * 45);
        });
    }

    playPipe() {
        this.playTone(150, 'triangle', 0.25, 0.25, 60);
    }

    playFlagpole() {
        this.playTone(400, 'sawtooth', 0.6, 0.18, 100);
    }

    playDie() {
        this.stopBGM();
        if (this.isMuted || !this.ctx) return;
        const notes = [500, 480, 450, 400, 350, 300, 200];
        notes.forEach((freq, i) => {
            setTimeout(() => {
                this.playTone(freq, 'square', 0.12, 0.2);
            }, i * 65);
        });
    }

    playGameOver() {
        this.stopBGM();
        if (this.isMuted || !this.ctx) return;
        const notes = [392, 330, 261, 196];
        notes.forEach((freq, i) => {
            setTimeout(() => {
                this.playTone(freq, 'square', 0.25, 0.2);
            }, i * 160);
        });
    }

    playStageClear() {
        this.stopBGM();
        if (this.isMuted || !this.ctx) return;
        // Fanfare notes: G3, C4, E4, G4, C5, E5, G5, E5, G5
        const notes = [
            { f: 392, d: 0.12 },
            { f: 523, d: 0.12 },
            { f: 659, d: 0.12 },
            { f: 784, d: 0.18 },
            { f: 1046, d: 0.22 },
            { f: 1318, d: 0.35 },
        ];
        let delay = 0;
        notes.forEach((n) => {
            setTimeout(() => {
                this.playTone(n.f, 'square', n.d, 0.2);
            }, delay);
            delay += n.d * 1000 + 40;
        });
    }

    // 8-bit NES Overworld Chiptune Background Music Loop
    startBGM() {
        if (this.bgmPlaying || this.isMuted) return;
        this.init();
        this.bgmPlaying = true;
        this.bgmStep = 0;

        // Simplified Mario main theme motif
        // E5 E5 . E5 . C5 E5 . G5 . . . G4 . . .
        const melody = [
            659, 659, 0, 659, 0, 523, 659, 0,
            784, 0, 0, 0, 392, 0, 0, 0,
            523, 0, 0, 392, 0, 0, 330, 0,
            0, 440, 0, 494, 0, 466, 440, 0,
            392, 659, 784, 880, 0, 698, 784, 0,
            659, 0, 523, 587, 494, 0, 0, 0,
        ];

        const bass = [
            261, 0, 261, 0, 261, 0, 261, 0,
            196, 0, 196, 0, 196, 0, 196, 0,
            261, 0, 261, 0, 220, 0, 220, 0,
            175, 0, 175, 0, 196, 0, 196, 0,
            261, 0, 261, 0, 220, 0, 220, 0,
            175, 0, 175, 0, 196, 0, 196, 0,
        ];

        const stepTimeMs = 125; // 120 BPM sixteenth notes

        const loop = () => {
            if (!this.bgmPlaying || this.isMuted) return;

            const mFreq = melody[this.bgmStep % melody.length];
            if (mFreq > 0) {
                this.playTone(mFreq, 'square', 0.1, 0.08);
            }

            const bFreq = bass[this.bgmStep % bass.length];
            if (bFreq > 0) {
                this.playTone(bFreq, 'triangle', 0.12, 0.12);
            }

            // High-hat / snare noise on beats 2 and 4
            if ((this.bgmStep % 4) === 2) {
                this.playNoise(0.04, 0.04);
            }

            this.bgmStep = (this.bgmStep + 1) % melody.length;
            this.bgmTimer = setTimeout(loop, stepTimeMs);
        };

        loop();
    }

    stopBGM() {
        this.bgmPlaying = false;
        if (this.bgmTimer) {
            clearTimeout(this.bgmTimer);
            this.bgmTimer = null;
        }
    }
}

window.audioManager = new AudioManager();
