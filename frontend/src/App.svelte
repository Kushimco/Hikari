<script lang="ts">
  import { onMount } from 'svelte';
  import { spring } from 'svelte/motion';

  let bookTitle = "";
  let isPulsing = false;
  let isFocused = false;

  // --- PHYSICS ENGINE ---
  // Controls the Y-position (floating).
  // Stiffness: How "tight" the spring is (lower = floatier).
  // Damping: How much friction (lower = more bounce).
  const orbY = spring(0, { stiffness: 0.02, damping: 0.1 });

  let animationFrame: number;
  let startTime = Date.now();

  // The Animation Loop
  function loop() {
    if (!isFocused) {
      // Floating Mode: Gentle Sine Wave
      const time = (Date.now() - startTime) / 1200; // Speed divisor
      const y = Math.sin(time) * 15; // Amplitude (15px up/down)
      orbY.set(y);
    } else {
      // Focused Mode: Pull smoothly to center (0)
      orbY.set(0);
    }
    animationFrame = requestAnimationFrame(loop);
  }

  onMount(() => {
    loop();
    return () => cancelAnimationFrame(animationFrame);
  });

  // --- INTERACTION HANDLERS ---
  function handleInput() {
    isPulsing = true;
    setTimeout(() => isPulsing = false, 100); // Quick pulse
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

<div data-tauri-drag-region class="titlebar"></div>
<main>
  <!-- Draggable Area (Invisible Title Bar) -->
  <div data-tauri-drag-region class="titlebar">
    <!-- Optional: Add your own Close/Minimize buttons here later -->
  </div>

  <!-- Ambient Background Elements -->
  <div class="ambient-light one"></div>
  <div class="ambient-light two"></div>

  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="line"></div>
  </aside>

  <!-- Main Stage -->
  <section class="orb-stage">
    
    <!-- 
         FLOATING WRAPPER
         Controlled by Svelte Spring (JS), not CSS Animation.
         This ensures 100% smooth transitions without snapping.
    -->
    <div class="orb-floater" style="transform: translateY({$orbY}px)">
      
      <!-- THE ORB (Visuals + Pulse) -->
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

  main {
    display: flex;
    height: 100vh;
    width: 100vw;
    position: relative;
  }

  /* --- Ambient Blobs --- */
  .ambient-light {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    opacity: 0.4;
    z-index: 0;
    animation: floatBlob 20s infinite ease-in-out alternate;
  }
  .one { width: 600px; height: 600px; background: #ffffff; top: -200px; left: -100px; }
  .two { width: 500px; height: 500px; background: #ffd1bc; bottom: -150px; right: -100px; }

  /* --- Sidebar --- */
  .sidebar { width: 100px; position: relative; z-index: 10; }
  .line { 
    position: absolute; 
    left: 50%; 
    top: 60px; 
    bottom: 60px; 
    width: 2px; 
    background: rgba(41, 32, 27, 0.2); 
  }

  /* --- Stage --- */
  .orb-stage {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
    z-index: 5;
  }

  /* --- Floating Wrapper --- */
  .orb-floater {
    width: 780px;
    height: 780px;
    margin-left: -200px;
    display: flex;
    justify-content: center;
    align-items: center;
    will-change: transform;
    transform: translateZ(0); 
  }

  /* --- The Orb Visuals --- */
  .orb {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    
    /* Gradient Background (High Opacity) */
    background: linear-gradient(
      180deg, 
      rgb(255, 204, 146) 0%, 
      rgba(250, 194, 147, 0.9) 60%, 
      rgba(203, 189, 255, 0.7) 90%
    );

    /* Rim Light + Depth Shadows */
    box-shadow: 
      inset 2px 4px 20px rgba(255, 255, 255, 0.6),  
      inset -2px -4px 30px rgba(0, 0, 0, 0.05),     
      0 25px 60px rgba(219, 168, 172, 0.35);        

    display: flex;
    justify-content: center;
    align-items: center;

    /* Pulse Animation Base */
    transform: scale(1);
    transition: transform 0.1s cubic-bezier(0.4, 0, 0.2, 1); 
  }

  .orb.pulsing {
    transform: scale(1.015); /* Subtle pop on type */
  }

  /* --- Glass Input Capsule --- */
  .glass-capsule {
    background: rgba(255, 255, 255, 0.3);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    padding: 18px 36px;
    border-radius: 100px;
    border: 1px solid rgba(255, 255, 255, 0.5);
    box-shadow: 0 8px 32px rgba(0,0,0,0.05);
    width: 340px;
    transition: all 0.3s ease;
    z-index: 20;
    will-change: transform;
  }

  .glass-capsule:focus-within {
    transform: scale(1.03);
    background: rgba(255, 255, 255, 0.45);
    box-shadow: 0 12px 40px rgba(0,0,0,0.08);
  }

  .titlebar {
    height: 40px; 
    width: 100vw; 
    position: fixed; 
    top: 0;
    left: 0;
    z-index: 9999; 
    cursor: default;
    pointer-events: auto;
}

  input {
    width: 100%;
    background: transparent;
    border: none;
    outline: none;
    font-size: 1.2rem;
    color: #5e4b4b;
    text-align: center;
    font-weight: 500;
    font-family: 'Inter', sans-serif;
  }
  input::placeholder { color: rgba(94, 75, 75, 0.45); font-weight: 400; }

  @keyframes floatBlob { 
    0% { transform: translate(0, 0); } 
    100% { transform: translate(40px, 60px); } 
  }
</style>
