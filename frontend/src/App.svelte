<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import Home from './lib/Home.svelte';

  const appWindow = getCurrentWindow();

  async function closeApp() {
    await appWindow.close();
  }
</script>

<div class="app-shell">
  <div data-tauri-drag-region class="titlebar">
    <button class="exit-btn" on:click={closeApp} aria-label="Close app">
      <svg
        viewBox="0 0 24 24"
        width="16"
        height="16"
        stroke="currentColor"
        stroke-width="2.5"
        fill="none"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <line x1="18" y1="6" x2="6" y2="18" />
        <line x1="6" y1="6" x2="18" y2="18" />
      </svg>
    </button>
  </div>

  <Home />
</div>

<style>
  :global(body) {
    margin: 0;
    background: linear-gradient(180deg, #fff8f3 0%, #deaa84 100%);
    min-height: 100vh;
    height: 100vh;
    overflow: hidden;
    font-family: "Inter", sans-serif;
  }

  .app-shell {
    width: 100vw;
    height: 100vh;
    position: relative;
  }

  .titlebar {
    height: 40px;
    width: 100vw;
    position: fixed;
    top: 0;
    left: 0;
    z-index: 9999;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    padding-right: 20px;
    box-sizing: border-box;
    pointer-events: auto;
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
  }

  .exit-btn:hover {
    background: rgba(41, 32, 27, 0.1);
    color: rgba(41, 32, 27, 0.8);
    transform: scale(1.1);
  }
</style>
