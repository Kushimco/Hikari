<script lang="ts">
  import { onMount } from 'svelte';
  import { spring } from 'svelte/motion';
  // FIX: Use 'getCurrentWindow' from the window module, NOT 'exit' from process
  import { getCurrentWindow } from '@tauri-apps/api/window';

  const appWindow = getCurrentWindow();

  async function closeApp() {
    await appWindow.close();
  }

  let bookTitle = "";
  let isPulsing = false;
  let isFocused = false;

  // --- PHYSICS ENGINE ---
  const orbY = spring(0, { stiffness: 0.02, damping: 0.1 });

  let animationFrame: number;
  let startTime = Date.now();

  function loop() {
    if (!isFocused) {
      const time = (Date.now() - startTime) / 1200;
      const y = Math.sin(time) * 15;
      orbY.set(y);
    } else {
      orbY.set(0);
    }
    animationFrame = requestAnimationFrame(loop);
  }

  onMount(() => {
    loop();
    return () => cancelAnimationFrame(animationFrame);
  });

  function handleInput() {
    isPulsing = true;
    setTimeout(() => isPulsing = false, 100);
  }

  function handleFocus() { isFocused = true; }
  function handleBlur() { isFocused = false; }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && bookTitle.trim()) {
      console.log("Adding book:", bookTitle);
      bookTitle = ""; 
    }
  }
</script>

<!-- Draggable Title Bar with Exit Button -->
<div data-tauri-drag-region class="titlebar">
  <!-- The Exit Button -->
  <button class="exit-btn" on:click={closeApp} aria-label="Close application">
    <svg viewBox="0 0 24 24" width="16" height="16" stroke="currentColor" stroke-width="2.5" fill="none" stroke-linecap="round" stroke-linejoin="round">
      <line x1="18" y1="6" x2="6" y2="18"></line>
      <line x1="6" y1="6" x2="18" y2="18"></line>
    </svg>
  </button>
</div>

<main>
  <div class="ambient-light one"></div>
  <div class="ambient-light two"></div>

  <aside class="sidebar">
    <div class="line"></div>
  </aside>

  <section class="orb-stage">
    <div class="orb-floater" style="transform: translateY({$orbY}px)">
      <div class="orb {isPulsing ? 'pulsing' : ''}">
        <div class="glass-capsule">
          <input 
            type="text" 
            bind:value={bookTitle} 
            placeholder="What are you reading?"
            on:input={handleInput}
            on:keydown={handleKeydown}
            on:focus={handleFocus}
            on:blur={handleBlur}
          />
        </div>
      </div>
    </div>
  </section>
</main>

<style>
  /* --- Global & Layout --- */
  :global(body) {
    margin: 0;
    background: linear-gradient(180deg, #FFF8F3 0%, #DEAA84 100%);
    min-height: 100vh;
    height: 100vh;
    overflow: hidden;
    font-family: 'Inter', sans-serif;
  }

  main { display: flex; height: 100vh; width: 100vw; position: relative; }

  /* --- Ambient Blobs --- */
  .ambient-light { position: absolute; border-radius: 50%; filter: blur(80px); opacity: 0.4; z-index: 0; animation: floatBlob 20s infinite ease-in-out alternate; }
  .one { width: 600px; height: 600px; background: #ffffff; top: -200px; left: -100px; }
  .two { width: 500px; height: 500px; background: #ffd1bc; bottom: -150px; right: -100px; }

  /* --- Sidebar --- */
  .sidebar { width: 100px; position: relative; z-index: 10; }
  .line { position: absolute; left: 50%; top: 60px; bottom: 60px; width: 2px; background: rgba(41, 32, 27, 0.2); }

  /* --- Stage --- */
  .orb-stage { flex: 1; display: flex; justify-content: center; align-items: center; position: relative; z-index: 5; }
  .orb-floater { width: 780px; height: 780px; margin-left: -200px; display: flex; justify-content: center; align-items: center; will-change: transform; transform: translateZ(0); }

  /* --- Orb & Input --- */
  .orb {
    width: 100%; height: 100%; border-radius: 50%;
    background: linear-gradient(180deg, rgb(255, 204, 146) 0%, rgba(250, 194, 147, 0.9) 60%, rgba(203, 189, 255, 0.7) 90%);
    box-shadow: inset 2px 4px 20px rgba(255, 255, 255, 0.6), inset -2px -4px 30px rgba(0, 0, 0, 0.05), 0 25px 60px rgba(219, 168, 172, 0.35);        
    display: flex; justify-content: center; align-items: center;
    transform: scale(1); transition: transform 0.1s cubic-bezier(0.4, 0, 0.2, 1); 
  }
  .orb.pulsing { transform: scale(1.015); }
  .glass-capsule { background: rgba(255, 255, 255, 0.3); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); padding: 18px 36px; border-radius: 100px; border: 1px solid rgba(255, 255, 255, 0.5); box-shadow: 0 8px 32px rgba(0,0,0,0.05); width: 340px; transition: all 0.3s ease; z-index: 20; will-change: transform; }
  .glass-capsule:focus-within { transform: scale(1.03); background: rgba(255, 255, 255, 0.45); box-shadow: 0 12px 40px rgba(0,0,0,0.08); }
  input { width: 100%; background: transparent; border: none; outline: none; font-size: 1.2rem; color: #5e4b4b; text-align: center; font-weight: 500; font-family: 'Inter', sans-serif; }
  input::placeholder { color: rgba(94, 75, 75, 0.45); font-weight: 400; }

  /* --- Titlebar & Exit Button --- */
  .titlebar {
    height: 40px; width: 100vw; position: fixed; top: 0; left: 0; z-index: 9999;
    cursor: default; pointer-events: auto;
    display: flex; justify-content: flex-end; align-items: center;
    padding-right: 20px;
    box-sizing: border-box;
  }

  .exit-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    color: rgba(41, 32, 27, 0.4); 
    padding: 8px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    -webkit-app-region: no-drag; 
  }

  .exit-btn:hover {
    background: rgba(41, 32, 27, 0.1);
    color: rgba(41, 32, 27, 0.8);
    transform: scale(1.1);
  }

  @keyframes floatBlob { 0% { transform: translate(0, 0); } 100% { transform: translate(40px, 60px); } }
</style>
