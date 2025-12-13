<script lang="ts">
  import { onMount } from 'svelte';
  import { fly, scale } from 'svelte/transition';
  import { elasticOut } from 'svelte/easing';

  // Placeholder setting categories
  const settingsItems = [
    { id: 'theme', label: 'Theme', icon: '🎨', color: '#E8D5C4' },
    { id: 'data', label: 'Data', icon: '💾', color: '#D5E8C4' },
    { id: 'account', label: 'Account', icon: '👤', color: '#C4D5E8' },
    { id: 'about', label: 'About', icon: 'ℹ️', color: '#E8C4C4' }
  ];

  let visible = false;

  onMount(() => {
    visible = true;
  });
</script>

<div class="settings-container">
  <div class="center-label" in:scale={{ duration: 600, delay: 200, easing: elasticOut }}>
    <h2>Settings</h2>
  </div>

  <div class="satellites">
    {#each settingsItems as item, i}
      <button 
        class="satellite-btn"
        style="--delay: {i * 0.1}s; --angle: {i * (360 / settingsItems.length)}deg; background: {item.color};"
        in:fly={{ y: 20, duration: 500, delay: 300 + (i * 100) }}
        aria-label={item.label}
      >
        <span class="icon">{item.icon}</span>
        <span class="label">{item.label}</span>
      </button>
    {/each}
  </div>
</div>

<style>
  .settings-container {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
  }

  .center-label h2 {
    font-size: 1.5rem;
    font-weight: 600;
    color: #5e4b4b;
    text-transform: uppercase;
    letter-spacing: 0.2em;
    margin: 0;
    opacity: 0.8;
  }

  .satellites {
    position: absolute;
    width: 100%;
    height: 100%;
    pointer-events: none; /* Let clicks pass through empty space */
  }

  .satellite-btn {
    pointer-events: auto;
    position: absolute;
    top: 50%;
    left: 50%;
    width: 80px;
    height: 80px;
    border-radius: 50%;
    border: 2px solid rgba(255, 255, 255, 0.8);
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    box-shadow: 
      0 8px 20px rgba(0,0,0,0.1),
      inset 0 0 10px rgba(255,255,255,0.5);
    transition: transform 0.3s ease, box-shadow 0.3s ease;
    
    /* Radial positioning math */
    --distance: 140px;
    transform: 
      translate(-50%, -50%) 
      rotate(var(--angle)) 
      translate(var(--distance)) 
      rotate(calc(var(--angle) * -1));
  }

  .satellite-btn:hover {
    transform: 
      translate(-50%, -50%) 
      rotate(var(--angle)) 
      translate(var(--distance)) 
      rotate(calc(var(--angle) * -1)) 
      scale(1.1);
    box-shadow: 0 12px 25px rgba(0,0,0,0.15);
    z-index: 10;
  }

  .icon {
    font-size: 1.5rem;
    margin-bottom: 2px;
  }

  .label {
    font-size: 0.7rem;
    font-weight: 600;
    color: rgba(60, 50, 50, 0.8);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
</style>
