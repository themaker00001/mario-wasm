// Main WebAssembly Mario Engine Client
(async function () {
    const canvas = document.getElementById('game-canvas');
    const ctx = canvas.getContext('2d');
    const loadingEl = document.getElementById('loading');
    const muteBtn = document.getElementById('btn-mute');
    const fullBtn = document.getElementById('btn-fullscreen');

    const INPUT_LEFT = 1 << 0;
    const INPUT_RIGHT = 1 << 1;
    const INPUT_UP = 1 << 2;
    const INPUT_DOWN = 1 << 3;
    const INPUT_JUMP = 1 << 4;
    const INPUT_RUN = 1 << 5;
    const INPUT_START = 1 << 6;

    let inputBits = 0;
    let wasmExports = null;
    let wasmMemory = null;
    let fbPtr = 0;
    let screenW = 320;
    let screenH = 240;
    let userHasInteracted = false;

    // Load Wasm binary
    try {
        let wasmModule;
        const wasmUrl = 'mario.wasm';
        if (WebAssembly.instantiateStreaming) {
            try {
                wasmModule = await WebAssembly.instantiateStreaming(fetch(wasmUrl), {});
            } catch (e) {
                const resp = await fetch(wasmUrl);
                const bytes = await resp.arrayBuffer();
                wasmModule = await WebAssembly.instantiate(bytes, {});
            }
        } else {
            const resp = await fetch(wasmUrl);
            const bytes = await resp.arrayBuffer();
            wasmModule = await WebAssembly.instantiate(bytes, {});
        }

        wasmExports = wasmModule.instance.exports;
        wasmMemory = wasmExports.memory;

        screenW = wasmExports.get_screen_width();
        screenH = wasmExports.get_screen_height();
        canvas.width = screenW;
        canvas.height = screenH;

        wasmExports.init_game();
        fbPtr = wasmExports.get_framebuffer_ptr();

        if (loadingEl) {
            loadingEl.style.display = 'none';
        }
    } catch (err) {
        console.error("Failed to load Wasm:", err);
        if (loadingEl) {
            loadingEl.textContent = "Error loading WebAssembly game: " + err.message;
            loadingEl.style.color = "#ff4444";
        }
        return;
    }

    // Keyboard handlers
    const keyMap = {
        'ArrowLeft': INPUT_LEFT,
        'KeyA': INPUT_LEFT,
        'ArrowRight': INPUT_RIGHT,
        'KeyD': INPUT_RIGHT,
        'ArrowUp': INPUT_UP | INPUT_JUMP,
        'KeyW': INPUT_UP | INPUT_JUMP,
        'ArrowDown': INPUT_DOWN,
        'KeyS': INPUT_DOWN,
        'Space': INPUT_JUMP | INPUT_START,
        'KeyZ': INPUT_JUMP,
        'KeyX': INPUT_RUN,
        'ShiftLeft': INPUT_RUN,
        'ShiftRight': INPUT_RUN,
        'Enter': INPUT_START,
    };

    window.addEventListener('keydown', (e) => {
        if (!userHasInteracted) {
            userHasInteracted = true;
            window.audioManager.init();
            window.audioManager.startBGM();
        }

        if (e.code === 'KeyM') {
            const muted = window.audioManager.toggleMute();
            muteBtn.textContent = muted ? '🔇 Sound: Off' : '🔊 Sound: On';
            return;
        }

        if (keyMap[e.code] !== undefined) {
            inputBits |= keyMap[e.code];
            // Prevent default browser scrolling with space/arrows
            if (['Space', 'ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.code)) {
                e.preventDefault();
            }
        }
    });

    window.addEventListener('keyup', (e) => {
        if (keyMap[e.code] !== undefined) {
            inputBits &= ~keyMap[e.code];
        }
    });

    // Mobile / Touch controls setup
    function setupTouchButton(id, bit) {
        const btn = document.getElementById(id);
        if (!btn) return;

        const press = (e) => {
            e.preventDefault();
            if (!userHasInteracted) {
                userHasInteracted = true;
                window.audioManager.init();
                window.audioManager.startBGM();
            }
            inputBits |= bit;
            btn.classList.add('active');
        };

        const release = (e) => {
            e.preventDefault();
            inputBits &= ~bit;
            btn.classList.remove('active');
        };

        btn.addEventListener('touchstart', press, { passive: false });
        btn.addEventListener('touchend', release, { passive: false });
        btn.addEventListener('touchcancel', release, { passive: false });
        btn.addEventListener('mousedown', press);
        btn.addEventListener('mouseup', release);
        btn.addEventListener('mouseleave', release);
    }

    setupTouchButton('btn-left', INPUT_LEFT);
    setupTouchButton('btn-right', INPUT_RIGHT);
    setupTouchButton('btn-down', INPUT_DOWN);
    setupTouchButton('btn-jump', INPUT_JUMP);
    setupTouchButton('btn-run', INPUT_RUN);
    setupTouchButton('btn-start', INPUT_START);

    // Audio & Fullscreen buttons
    if (muteBtn) {
        muteBtn.addEventListener('click', () => {
            if (!userHasInteracted) {
                userHasInteracted = true;
                window.audioManager.init();
            }
            const muted = window.audioManager.toggleMute();
            muteBtn.textContent = muted ? '🔇 Sound: Off' : '🔊 Sound: On';
        });
    }

    if (fullBtn) {
        fullBtn.addEventListener('click', () => {
            const container = document.getElementById('game-container') || canvas;
            if (!document.fullscreenElement) {
                container.requestFullscreen?.().catch(console.warn);
            } else {
                document.exitFullscreen?.().catch(console.warn);
            }
        });
    }

    canvas.addEventListener('click', () => {
        if (!userHasInteracted) {
            userHasInteracted = true;
            window.audioManager.init();
            window.audioManager.startBGM();
        }
    });

    // Fixed timestep 60fps render loop
    let lastTime = performance.now();
    const frameDuration = 1000 / 60;
    let accumulator = 0;

    function gameLoop(now) {
        const delta = Math.min(now - lastTime, 100);
        lastTime = now;
        accumulator += delta;

        while (accumulator >= frameDuration) {
            // Update wasm game
            const soundEvents = wasmExports.update_game(inputBits);
            if (soundEvents > 0) {
                window.audioManager.handleSoundEvents(soundEvents);
            }

            // Sync BGM with game state
            const state = wasmExports.get_game_state();
            if (state === 1) { // Playing
                if (!window.audioManager.bgmPlaying && !window.audioManager.isMuted && userHasInteracted) {
                    window.audioManager.startBGM();
                }
            } else if (state === 2 || state === 3 || state === 6) { // Dying, GameOver, StageClear
                window.audioManager.stopBGM();
            }

            accumulator -= frameDuration;
        }

        // Render frame directly from Wasm memory to Canvas
        fbPtr = wasmExports.get_framebuffer_ptr();
        const byteCount = screenW * screenH * 4;
        const clampedArray = new Uint8ClampedArray(wasmMemory.buffer, fbPtr, byteCount);
        const imgData = new ImageData(clampedArray, screenW, screenH);
        ctx.putImageData(imgData, 0, 0);

        requestAnimationFrame(gameLoop);
    }

    requestAnimationFrame(gameLoop);
})();
