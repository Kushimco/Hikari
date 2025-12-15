<script lang="ts">
  /* 
     ================================================================
     SECTION 1: PROPS & DERIVED STATE
     ================================================================
  */
  // {
    export let activeTab: "home" | "menu" | "settings" = "home";
    export let isReturning = false;
    export let returnStage: "idle" | "fading" | "bouncing_down" | "bouncing_up" = "idle";
    export let isGlowing = false; 
    export let isPulsing = false; 
    export let shouldScale = false; 
    export let isAdding = false;
    export let orbElement: HTMLDivElement | null = null;

    $: isTiny = isReturning && returnStage === "bouncing_down";
    $: glowTiny = isGlowing && isTiny;

    $: containerClass = `
      orb 
      ${activeTab === "menu" || (isReturning && returnStage === "fading") ? 'expanded' : ''}
      ${isTiny ? 'small-orb' : ''}
      ${isGlowing ? 'glowing' : ''}
      ${glowTiny ? 'small-glow' : ''}
      ${shouldScale ? 'typing-scale' : ''}
      ${isPulsing && activeTab === "home" ? 'pulsing' : ''}
      ${isAdding ? 'add-success' : ''}
    `;
  // }
</script>

<div
  class="orb-floater"
  class:expanded-floater={activeTab === "menu" || (isReturning && returnStage === "fading")}
>
  <div
    class={containerClass}
    bind:this={orbElement}
  >
    <!-- FLUID ENGINE -->
    <div class="fluid-engine">
      <div class="blob blob-1"></div> 
      <div class="blob blob-2"></div> 
      <div class="blob blob-3"></div> 
    </div>

    <!-- GLOW LAYER -->
    <div class="glow-layer"></div>

    <!-- GLASS SURFACE -->
    <div class="glass-surface"></div>

    <!-- CONTENT SLOT -->
    <div class="orb-content">
      <slot />
    </div>
  </div>
</div>

<style>
  /* #region --- 1. FLOATER WRAPPER --- */
  .orb-floater {
    width: 780px; height: 780px; margin-left: -102px;
    display: flex; justify-content: center; align-items: center;
    will-change: transform;
    transition:
      transform 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      margin 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      width 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      height 1.5s cubic-bezier(0.25, 1, 0.5, 1);
  }

  .orb-floater.expanded-floater {
    width: 96%; height: 95%; margin-left: 0; padding: 20px;
    padding-right: 75px; box-sizing: border-box; transform: translateY(0);
  }
  /* #endregion */

  /* #region --- 2. ORB CONTAINER --- */
  .orb {
    position: relative; overflow: visible; 
    width: 100%; height: 100%; border-radius: 50%;
    background: linear-gradient(180deg, rgba(254, 214, 169, 0.4) 0%, rgba(255, 189, 245, 0.3) 100%);
    box-shadow: 0 25px 60px rgba(94, 75, 75, 0.15), 0 10px 20px rgba(0,0,0,0.02);
    display: flex; justify-content: center; align-items: center;
    max-width: 780px; max-height: 780px;
    transition:
      width 0.7s cubic-bezier(0.25, 1, 0.5, 1),
      height 0.7s cubic-bezier(0.25, 1, 0.5, 1),
      border-radius 0.7s cubic-bezier(0.25, 1, 0.5, 1),
      transform 0.6s cubic-bezier(0.25, 0.8, 0.25, 1),
      box-shadow 0.9s cubic-bezier(0.33, 0, 0.67, 1);
    animation: float 8s ease-in-out infinite;
  }

  @keyframes float {
    0% { transform: translateY(0px); }
    25% { transform: translateY(-12px); }
    75% { transform: translateY(12px); }
    100% { transform: translateY(0px); }
  }
  /* #endregion */

  /* #region --- 3. FLUID ENGINE --- */
  .fluid-engine {
    position: absolute; inset: 0; z-index: 0; border-radius: inherit;
    overflow: hidden; pointer-events: none; background: transparent;
  }

  .blob {
    position: absolute; border-radius: 50%; filter: blur(60px); 
    opacity: 0.9; will-change: transform; mix-blend-mode: normal; 
    transition: background 0.8s ease, opacity 0.8s ease, transform 0.8s ease;
  }

  /* Blob 1: Main Peach */
  .blob-1 {
    top: -10%; left: -10%; width: 90%; height: 90%;
    background: rgba(254, 214, 169, 1); 
    animation: liquid1 7s infinite ease-in-out;
  }

  /* Blob 2: Pink/Red */
  .blob-2 {
    bottom: -10%; right: -10%; width: 90%; height: 90%;
    background: rgba(255, 189, 245, 1);
    animation: liquid2 8s infinite ease-in-out; 
  }

  /* Blob 3: Orange Mixer */
  .blob-3 {
    top: 30%; left: 40%; width: 70%; height: 70%;
    background: #ffcba4; opacity: 0.85;
    animation: liquid3 6s infinite linear; 
  }
  /* #endregion */

  /* #region --- 4. GLOW & GLASS LAYERS --- */
  .glow-layer {
    position: absolute; inset: 0; z-index: 1; border-radius: inherit;
    pointer-events: none; opacity: 0;
    transition: opacity 0.6s cubic-bezier(0.25, 1, 0.5, 1);
    box-shadow:
      inset 0 0 50px rgba(255, 255, 255, 0.8),
      0 0 80px rgba(254, 214, 169, 0.8),
      0 0 150px rgba(255, 189, 245, 0.5);
  }

  /* Library Glow (Max Gold) */
  .orb.expanded .glow-layer {
    opacity: 0.85; 
    box-shadow:
      inset 0 0 150px rgba(255, 215, 120, 0.75), /* Intense Gold Inner */
      0 0 120px rgba(254, 180, 100, 0.6);        /* Rich Orange Outer */
  }

  .orb.glowing .glow-layer { opacity: 1; }

  .orb.small-glow .glow-layer {
    opacity: 1;
    box-shadow:
      inset 0 0 60px rgba(255, 255, 255, 0.95),
      0 0 100px rgba(254, 214, 169, 1),
      0 0 180px rgba(255, 189, 245, 0.7);
  }

  .glass-surface {
    position: absolute; inset: 0; z-index: 2; border-radius: inherit; 
    box-shadow: inset 0 0 50px rgba(255, 255, 255, 0.5), inset -2px -4px 20px rgba(94, 75, 75, 0.05);
    pointer-events: none;
  }
  /* #endregion */

  /* #region --- 5. LIQUID ANIMATIONS --- */
  @keyframes liquid1 {
    0% { transform: translate(0, 0) rotate(0deg) scale(1, 1); }
    33% { transform: translate(15%, 10%) rotate(90deg) scale(1.1, 0.9); }
    66% { transform: translate(-10%, 20%) rotate(180deg) scale(0.9, 1.1); }
    100% { transform: translate(0, 0) rotate(360deg) scale(1, 1); }
  }

  @keyframes liquid2 {
    0% { transform: translate(0, 0) rotate(0deg) scale(1); }
    50% { transform: translate(-15%, -15%) rotate(-120deg) scale(1.2); }
    100% { transform: translate(0, 0) rotate(-360deg) scale(1); }
  }

  @keyframes liquid3 {
    0% { transform: translate(0, 0) scale(1); }
    25% { transform: translate(-30%, 20%) scale(1.2); }
    50% { transform: translate(10%, -10%) scale(0.8); }
    75% { transform: translate(20%, 30%) scale(1.1); }
    100% { transform: translate(0, 0) scale(1); }
  }
  /* #endregion */

  /* #region --- 6. CONTENT & STATES --- */
  .orb-content {
    position: relative; z-index: 3; width: 100%; height: 100%;
    display: flex; justify-content: center; align-items: center;
  }

  .orb.expanded {
    border-radius: 40px; width: 100%; height: 100%;
    max-width: 100%; max-height: 100%; padding: 40px;
    box-sizing: border-box; cursor: default; display: block; 
    transform: scale(1) !important; overflow: hidden;
    /* Tint the shadow warmer */
    box-shadow: 0 20px 80px rgba(255, 180, 100, 0.5), 0 10px 30px rgba(94, 75, 75, 0.1);
  }
  
  .orb.expanded .orb-content {
    display: block; overflow-y: auto; scrollbar-width: none; height: 100%;
  }
  .orb.expanded .orb-content::-webkit-scrollbar { display: none; }

  .orb.small-orb { width: 60px !important; height: 60px !important; }
  .orb.glowing { filter: brightness(1.05); }

  .orb.typing-scale {
    transform: scale(1.02); transition: transform 0.4s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  .orb.pulsing {
    transform: translateY(0) scale(1.05) !important;
    transition: transform 0.05s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  .orb.add-success { animation: float 8s ease-in-out infinite, addPulse 0.6s ease-out; }

  @keyframes addPulse {
    0% { filter: brightness(1.05); }
    40% { filter: brightness(1.15); box-shadow: 0 0 120px rgba(167, 232, 189, 0.6); } 
    100% { filter: brightness(1.05); }
  }
  /* #endregion */
</style>
