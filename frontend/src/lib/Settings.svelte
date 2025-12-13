<script lang="ts">
  import { fly, scale } from 'svelte/transition';
  import { elasticOut } from 'svelte/easing';
  // import { invoke } from '@tauri-apps/api/core'; // Uncomment when connecting to backend
  export let isReturning = false;
  export let returnStage: "idle" | "fading" | "bouncing_down" | "bouncing_up" = "idle";
  // --- SETTINGS DATA ---
  const settingsOptions = [
    { id: 'export', label: 'Backup', icon: '📥', color: 'rgba(213, 232, 196, 0.4)' },
    { id: 'import', label: 'Restore', icon: '📤', color: 'rgba(196, 213, 232, 0.4)' },
    { id: 'theme', label: 'Theme', icon: '🎨', color: 'rgba(232, 196, 196, 0.4)' },
    { id: 'clear', label: 'Clear', icon: '🗑️', color: 'rgba(232, 215, 196, 0.4)' },
    { id: 'about', label: 'About', icon: 'ℹ️', color: 'rgba(220, 196, 232, 0.4)' },
  ];

  function getPosition(index: number, total: number, radius: number) {
    const angle = (index / total) * 2 * Math.PI - (Math.PI / 2);
    return {
      x: Math.cos(angle) * radius,
      y: Math.sin(angle) * radius
    };
  }

  function handleClick(id: string) {
    console.log("Clicked setting:", id);
    // invoke('your_command_here', { ... });
  }
</script>

<div class="settings-container">
  <!-- Center Label/Hub -->
  <div 
    class="center-hub"
    in:scale={{ duration: 600, easing: elasticOut, start: 0 }}
    out:scale={{ duration: 300, start: 0 }}
  >
    <span>Settings</span>
  </div>

  <!-- The Bubbles -->
  {#each settingsOptions as item, i}
    {@const pos = getPosition(i, settingsOptions.length, 140)}
    
    <button
      class="bubble"
      class:flying-back={isReturning && returnStage === "bouncing_down"}
      class:hidden={isReturning && returnStage === "bouncing_up"}
      style="
        --x: {pos.x}px; 
        --y: {pos.y}px; 
        --color: {item.color};
        --delay: {i * 0.1}s;
      "
      in:fly={{ 
        x: 0, 
        y: 0, 
        duration: 800, 
        delay: i * 50, 
        easing: elasticOut 
      }}
      out:scale={{ duration: 250, delay: (settingsOptions.length - i) * 50 }}
      on:click={() => handleClick(item.id)}
      aria-label={item.label}
    >
      <div class="bubble-content">
        <span class="icon">{item.icon}</span>
        <span class="label">{item.label}</span>
      </div>
      <div class="glow"></div>
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
  }

  .center-hub {
    position: absolute;
    width: 80px;
    height: 80px;
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 0.8rem;
    font-weight: 600;
    color: #5e4b4b;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    border: 1px solid rgba(255, 255, 255, 0.4);
    z-index: 10;
    box-shadow: 0 4px 12px rgba(0,0,0,0.05);
  }

  .bubble {
    pointer-events: auto;
    position: absolute;
    width: 90px;
    height: 90px;
    border-radius: 50%;
    border: 1px solid rgba(255, 255, 255, 0.6);
    background: var(--color);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    transform: translate(var(--x), var(--y));
    animation: float 6s ease-in-out infinite;
    animation-delay: var(--delay);
    transition: transform 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275), box-shadow 0.2s ease, border-color 0.2s ease;
    box-shadow: 
      0 10px 30px rgba(0, 0, 0, 0.1),
      inset 0 0 20px rgba(255, 255, 255, 0.4);
  }

  .bubble:hover {
    z-index: 20;
    transform: translate(var(--x), var(--y)) scale(1.15);
    box-shadow: 
      0 15px 40px rgba(0, 0, 0, 0.15),
      inset 0 0 30px rgba(255, 255, 255, 0.6),
      0 0 20px var(--color);
    border-color: rgba(255, 255, 255, 0.9);
  }

  .bubble:active {
    transform: translate(var(--x), var(--y)) scale(0.95);
  }

  .bubble-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    pointer-events: none;
  }

  .icon {
    font-size: 1.6rem;
    filter: drop-shadow(0 2px 4px rgba(0,0,0,0.1));
  }

  .label {
    font-size: 0.7rem;
    font-weight: 600;
    color: #4b332e;
    text-transform: uppercase;
    opacity: 0.8;
  }

  .bubble.flying-back {
    transform: translate(-50%, -50%) !important;
    transition: transform 0.5s ease;
  }

  .bubble.hidden {
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  @keyframes float {
    0%, 100% { margin-top: 0px; }
    50% { margin-top: -15px; } 
  }
</style>
