<script lang="ts">
  import { fly, scale } from 'svelte/transition';
  import { elasticOut } from 'svelte/easing';

  // --- SETTINGS DATA ---
  const settingsOptions = [
    { 
      id: 'export', 
      label: 'Backup', 
      path: 'M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M7 10l5 5 5-5 M12 15V3' 
    },
    { 
      id: 'import', 
      label: 'Restore', 
      path: 'M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M17 8l-5-5-5 5 M12 3v12' 
    },
    { 
      id: 'theme', 
      label: 'Theme', 
      path: 'M12 21a9 9 0 1 0 0-18c4.97 0 9 2 9 6Z M19 10a1 1 0 1 1 0 2 1 1 0 0 1 0-2Z M15 16a1 1 0 1 1 0 2 1 1 0 0 1 0-2Z M9 16a1 1 0 1 1 0 2 1 1 0 0 1 0-2Z' 
    },
    { 
      id: 'clear', 
      label: 'Clear', 
      path: 'M3 6h18 M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2 M10 11v6 M14 11v6' 
    },
    { 
      id: 'about', 
      label: 'About', 
      path: 'M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z M12 16v-4 M12 8h.01' 
    },
  ];

  // We track pulse ID only for the quick reaction animation
  let pulsingId: string | null = null;

  function getPosition(index: number, total: number, radius: number) {
    const angle = (index / total) * 2 * Math.PI - (Math.PI / 2);
    return {
      x: Math.cos(angle) * radius,
      y: Math.sin(angle) * radius
    };
  }

  function handleClick(id: string) {
    // Trigger fast pulse animation
    pulsingId = id;
    
    // Clear pulse after 200ms
    setTimeout(() => {
      pulsingId = null;
    }, 200);

    console.log("Clicked setting:", id);
  }
</script>

<div class="settings-container">
  {#each settingsOptions as item, i}
    {@const pos = getPosition(i, settingsOptions.length, 145)} 
    
    <button
      class="bubble"
      class:click-pulse={pulsingId === item.id}
      style="
        --tx: {pos.x}px; 
        --ty: {pos.y}px;
        --float-delay: -{i * 1.2}s;
      "
      in:fly={{ 
        x: -pos.x, 
        y: -pos.y, 
        duration: 1000, 
        delay: i * 50, 
        easing: elasticOut 
      }}
      out:scale={{ duration: 200, delay: 0 }}
      on:click={() => handleClick(item.id)}
      aria-label={item.label}
    >
      <div class="bubble-content">
        <svg 
          class="icon" 
          viewBox="0 0 24 24" 
          fill="none" 
          stroke="currentColor" 
          stroke-width="1.5" 
          stroke-linecap="round" 
          stroke-linejoin="round"
        >
          <path d={item.path} />
        </svg>
        <span class="label">{item.label}</span>
      </div>
      
      <div class="glow-layer"></div>
    </button>
  {/each}
</div>

<style>
  .settings-container {
    position: absolute;
    inset: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    pointer-events: none;
    margin-left: -105px;
  }

  /* Standard floating animation */
  @keyframes float {
    0%, 100% { transform: translate(var(--tx), var(--ty)); }
    50% { transform: translate(var(--tx), calc(var(--ty) - 10px)); }
  }

  /* Quick pop animation for click reaction */
  @keyframes pop {
    0% { transform: scale(1); }
    50% { transform: scale(1.08); filter: brightness(1.1); }
    100% { transform: scale(1); filter: brightness(1); }
  }

  .bubble {
    pointer-events: auto;
    position: absolute;
    width: 96px;
    height: 96px;
    border-radius: 50%;
    border: none;
    outline: none;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;

    /* Gradient */
    background: linear-gradient(
      180deg,
      rgba(254, 214, 169, 0.85) 0%,
      rgba(244, 202, 167, 0.95) 55%,
      rgba(255, 189, 245, 0.8) 100%
    );

    /* Shadows */
    box-shadow:
      inset 1px 2px 15px rgba(255, 255, 255, 0.7),
      inset -1px -2px 10px rgba(0, 0, 0, 0.05),
      0 12px 28px rgba(94, 75, 75, 0.15);

    /* Floating Animation ALWAYS active */
    animation: float 4s ease-in-out infinite;
    animation-delay: var(--float-delay);
    
    /* We use 'composite' transform logic if possible, 
       but standard CSS animation overwrites 'transform'.
       So we'll use a wrapper approach OR rely on scale transition 
       on hover which composes with the keyframe translation if carefully managed.
       
       However, simplified: The animation controls translate. 
       We will use transition for scale/shadow changes. */
    transition: 
      box-shadow 0.3s ease,
      filter 0.2s ease;
  }

  .bubble:hover {
    z-index: 20;
    /* We DON'T pause animation anymore based on your request, 
       but standard behavior usually pauses. If you want it to keep moving, 
       remove 'animation-play-state: paused'. 
       I will keep it running as requested ("not stop floating"). */
    
    /* Since we can't easily scale AND translate via keyframes simultaneously without conflicts,
       a common trick is to just brighten/shadow on hover if we want to keep the float keyframe running perfectly.
       OR we use a wrapper for the float and inner for scale.
       For simplicity here, I'll just apply shadow/brightness to avoid jumpiness. */
    filter: brightness(1.05);
    
    box-shadow:
      inset 0 0 20px rgba(255, 255, 255, 0.95),
      0 0 25px rgba(255, 220, 180, 0.6),
      0 15px 40px rgba(94, 75, 75, 0.2);
  }

  /* CLICK PULSE: Runs a separate animation? 
     Actually, since 'float' is running on 'transform', we can't easily run 'pop' on 'transform' 
     without overriding the float position. 
     
     SOLUTION: Apply the pop to the CONTENT inside, or use a separate property like scale 
     if supported, or use a wrapper. 
     
     To make it simple and robust: I will animate the .bubble-content scale instead. */
  
  .bubble.click-pulse .bubble-content {
    animation: pop 0.2s ease-out forwards;
  }
  
  /* Also intensify glow on the bubble container during pulse */
  .bubble.click-pulse {
    box-shadow:
      inset 0 0 30px rgba(255, 255, 255, 0.9),
      0 0 50px rgba(255, 220, 180, 0.8),
      0 10px 40px rgba(94, 75, 75, 0.25);
  }

  .glow-layer {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    pointer-events: none;
    opacity: 0;
    transition: opacity 0.4s ease;
    box-shadow: inset 0 0 12px rgba(255, 255, 255, 0.9);
  }

  .bubble:hover .glow-layer {
    opacity: 1;
  }

  .bubble-content {
    position: relative;
    z-index: 2;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 6px;
    pointer-events: none;
    color: #5e4b4b;
    /* Ensure transform-origin is center for the pop effect */
    transform-origin: center center;
  }

  .icon {
    width: 28px;
    height: 28px;
    opacity: 0.85;
    filter: drop-shadow(0 1px 1px rgba(255, 255, 255, 0.5));
    transition: opacity 0.3s ease;
  }
  
  .bubble:hover .icon {
    opacity: 1;
  }

  .label {
    font-family: 'Playfair Display', serif; 
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    opacity: 0.8;
    margin-top: 1px;
    transition: opacity 0.3s ease;
  }

  .bubble:hover .label {
    opacity: 1;
  }
</style>
