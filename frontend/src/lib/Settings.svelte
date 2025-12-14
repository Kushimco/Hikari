<script lang="ts">
  import { fly, scale, fade } from 'svelte/transition';
  import { elasticOut, cubicOut, backOut } from 'svelte/easing';
  import { createEventDispatcher, tick } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { save, open, message, confirm } from '@tauri-apps/plugin-dialog';

  export let stage: "idle" | "collapsing" | "glowing" | "dividing" | "gathering" = "idle";
  export let appName = "Hikari";
  export let version: string | null = null;
  export const tagline = "A gentle library for your books.";
  export let repoUrl: string | null = null;

  const dispatch = createEventDispatcher<{
    readyToExpand: void;
    select: { id: string }; 
  }>();

  // --- 1. SETTINGS CATEGORIES ---
  const settingsOptions = [
    { id: 'library', label: 'Library', path: 'M4 19.5A2.5 2.5 0 0 1 6.5 17H20M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z' },
    { id: 'appearance', label: 'Look', path: 'M12 21a9 9 0 1 0 0-18c4.97 0 9 2 9 6Z M19 10a1 1 0 1 1 0 2 1 1 0 0 1 0-2Z M15 16a1 1 0 1 1 0 2 1 1 0 0 1 0-2Z M9 16a1 1 0 1 1 0 2 1 1 0 0 1 0-2Z' },
    { id: 'data', label: 'Data', path: 'M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M7 10l5 5 5-5 M12 15V3' },
    { id: 'about', label: 'About', path: 'M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z M12 16v-4 M12 8h.01' }
  ].map((o, idx) => ({ ...o, idx }));

  const total = settingsOptions.length;
  const RING_RADIUS = 145; 
  const BUBBLE_RADIUS = 48; 

  // --- Animation Logic ---
  let revealCount = 0;
  let timer: number | null = null;
  let lastStage: typeof stage = "idle";
  const spawnEveryMs = 90;
  const gatherEveryMs = 85;
  const flyDurationMs = 520;
  const scaleDurationMs = 220;
  let pulseKey = 0;

  function stop() { if (timer != null) { clearInterval(timer); timer = null; } }
  function startDividing() {
    stop(); revealCount = 0; pulseKey = 0;
    timer = window.setInterval(() => {
      revealCount += 1; pulseKey += 1;
      if (revealCount >= total) { revealCount = total; stop(); }
    }, spawnEveryMs);
  }
  function startGathering() {
    stop(); pulseKey = 0; revealCount = total;
    timer = window.setInterval(() => {
      revealCount -= 1; pulseKey += 1;
      if (revealCount <= 0) {
        revealCount = 0; stop();
        window.setTimeout(() => { dispatch('readyToExpand'); }, 180);
      }
    }, gatherEveryMs);
  }
  $: {
    if (stage !== lastStage) {
      if (stage === "dividing") startDividing();
      else if (stage === "gathering") startGathering();
      else { stop(); if (stage === "idle") revealCount = 0; }
      lastStage = stage;
    }
  }
  $: visibleOptions = (stage === "dividing" || stage === "gathering") ? settingsOptions.slice(0, revealCount) : [];
  $: seedVisible = stage === "collapsing" || stage === "glowing" || stage === "dividing" || stage === "gathering";
  $: seedScale = (() => {
    if (stage === "dividing") return Math.max(0.18, 1 - revealCount / (total * 1.1));
    if (stage === "gathering") return Math.min(1, 0.18 + ((total - revealCount) / total) * 0.82);
    return 1;
  })();

  function getPosition(index: number, totalCount: number, radius: number) {
    const angle = (index / totalCount) * 2 * Math.PI - (Math.PI / 2);
    return { x: Math.cos(angle) * radius, y: Math.sin(angle) * radius };
  }

  // --- 2. POPUP SYSTEM ---
  let activePopup: string | null = null;
  let popupDialog: HTMLDialogElement | null = null;
  let pulsingId: string | null = null;
  let popupStyle = "";
  let popupOrigin = "center center";

  async function openPopup(id: string, idx: number) {
    const pos = getPosition(idx, total, RING_RADIUS);
    activePopup = id;

    const GAP = 20; 
    const OFFSET = BUBBLE_RADIUS + GAP;

    if (idx === 0) { // Top
      popupStyle = `left: calc(50% + ${pos.x}px); top: calc(50% + ${pos.y - OFFSET}px); transform: translate(-50%, -100%);`;
      popupOrigin = "bottom center";
    } else if (idx === 1) { // Right
      popupStyle = `left: calc(50% + ${pos.x + OFFSET}px); top: calc(50% + ${pos.y}px); transform: translate(0, -50%);`;
      popupOrigin = "center left";
    } else if (idx === 2) { // Bottom
      popupStyle = `left: calc(50% + ${pos.x}px); top: calc(50% + ${pos.y + OFFSET}px); transform: translate(-50%, 0);`;
      popupOrigin = "top center";
    } else { // Left
      popupStyle = `left: calc(50% + ${pos.x - OFFSET}px); top: calc(50% + ${pos.y}px); transform: translate(-100%, -50%);`;
      popupOrigin = "center right";
    }

    await tick();
    popupDialog?.focus();
  }

  function closePopup() { activePopup = null; }

  function onWindowKeydown(e: KeyboardEvent) {
    if (!activePopup) return;
    if (e.key === 'Escape') closePopup();
  }

  function handleClick(id: string, idx: number) {
    if (activePopup === id) { closePopup(); return; }
    pulsingId = id;
    openPopup(id, idx);
    setTimeout(() => (pulsingId = null), 180);
  }

  // --- 3. DATA MANAGEMENT LOGIC ---

  async function handleBackup() {
    try {
      const path = await save({
        filters: [{ name: 'JSON', extensions: ['json'] }],
        defaultPath: 'hikari_library_backup.json'
      });
      if (path) {
        await invoke('backup_library', { targetPath: path });
        await message('Library backed up successfully!', { title: 'Success', kind: 'info' });
      }
    } catch (err) {
      console.error(err);
      await message(`Backup failed: ${err}`, { title: 'Error', kind: 'error' });
    }
  }

  async function handleRestore() {
    try {
      const path = await open({
        multiple: false,
        directory: false,
        filters: [{ name: 'JSON', extensions: ['json'] }]
      });
      if (path) {
        const confirmed = await confirm('This will overwrite your current library. Are you sure?', { title: 'Restore Library', kind: 'warning' });
        if (confirmed) {
          await invoke('restore_library', { sourcePath: path });
          await message('Library restored! Please restart or refresh.', { title: 'Success', kind: 'info' });
          dispatch('select', { id: 'refresh' }); 
        }
      }
    } catch (err) {
      console.error(err);
      await message(`Restore failed: ${err}`, { title: 'Error', kind: 'error' });
    }
  }

  async function handleClearDB() {
    const confirmed = await confirm('Are you sure you want to DELETE ALL BOOKS? This cannot be undone.', { title: 'Clear Database', kind: 'warning' });
    if (confirmed) {
       try {
         await invoke('delete_library');
         await message('Library cleared.', { title: 'Done' });
         dispatch('select', { id: 'refresh' }); 
       } catch (err) {
         await message(`Failed to clear: ${err}`, { kind: 'error' });
       }
    }
  }

  function handleAction(actionId: string) {
    if (actionId === 'theme') dispatch('select', { id: 'theme' });
  }
</script>

<svelte:window on:keydown={onWindowKeydown} />

<div class="settings-container">
  <!-- Seed Logic -->
  {#if seedVisible}
    <div class="seed" out:scale={{ duration: 260, easing: cubicOut, start: 0 }} style="transform: scale({seedScale});">
      {#if stage === "dividing" || stage === "gathering"}
        {#key pulseKey}<div class="seed-tick"></div>{/key}
      {/if}
    </div>
  {/if}

  <!-- Bubbles -->
  {#each visibleOptions as item (item.id)}
    {@const pos = getPosition(item.idx, total, RING_RADIUS)}
    <button
      class="bubble"
      style="left: calc(50% + {pos.x}px); top: calc(50% + {pos.y}px); --float-delay: -{item.idx * 1.2}s; --float-start-delay: {flyDurationMs + item.idx * 30}ms;"
      in:fly={{ x: -pos.x, y: -pos.y, duration: flyDurationMs, easing: elasticOut }}
      out:fly={{ x: -pos.x, y: -pos.y, duration: 420, easing: backOut }}
      on:click={() => handleClick(item.id, item.idx)}
      aria-label={item.label}
      type="button"
    >
      <div class="bubble-skin" class:returning={stage === "gathering"} class:click-pulse={pulsingId === item.id}
        in:scale={{ start: 0, duration: scaleDurationMs, easing: backOut }}
        out:scale={{ start: 0, duration: 220, easing: cubicOut }}>
        <div class="bubble-content">
          <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d={item.path} /></svg>
          <span class="label">{item.label}</span>
        </div>
        <div class="glow-layer"></div>
      </div>
    </button>
  {/each}

  <!-- 3. DYNAMIC POPUP -->
  {#if activePopup}
    <!-- Backdrop: ensure it catches clicks to close -->
    <button class="popup-backdrop" type="button" on:click={closePopup} aria-label="Close" transition:fade={{ duration: 200 }}></button>

    <!-- Anchor + Dialog: ensure they allow clicks inside -->
    <div class="popup-anchor" style="{popupStyle}">
      <dialog 
        class="popup-card themed" 
        open 
        bind:this={popupDialog} 
        tabindex="-1" 
        on:click|stopPropagation
        style="transform-origin: {popupOrigin};"
        in:scale={{ start: 0.8, opacity: 0, duration: 250, easing: elasticOut }}
        out:scale={{ start: 0.9, opacity: 0, duration: 150, easing: cubicOut }}
      >
        <div class="popup-content-wrapper">
            <header class="popup-header mini">
            <div class="popup-title">
                <h2>
                {#if activePopup === 'library'}Library
                {:else if activePopup === 'appearance'}Look & Feel
                {:else if activePopup === 'data'}Data
                {:else if activePopup === 'about'}{appName}
                {/if}
                </h2>
            </div>
            <!-- X CLOSE BUTTON with stopPropagation -->
            <button class="popup-close-btn" type="button" on:click|stopPropagation={closePopup} aria-label="Close">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18M6 6l12 12" /></svg>
            </button>
            </header>

            <div class="popup-body mini">
            {#if activePopup === 'data'}
                <div class="action-grid mini">
                <button class="action-row mini" on:click={handleBackup}>
                    <div class="action-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M7 10l5 5 5-5 M12 15V3"/></svg></div>
                    <span>Backup</span>
                </button>
                <button class="action-row mini" on:click={handleRestore}>
                    <div class="action-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M17 8l-5-5-5 5 M12 3v12"/></svg></div>
                    <span>Restore</span>
                </button>
                <button class="action-row mini danger" on:click={handleClearDB}>
                    <div class="action-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M3 6h18 M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg></div>
                    <span>Clear DB</span>
                </button>
                </div>
            {:else if activePopup === 'about'}
                <p class="popup-text mini">Hikari v{version || '1.0'}</p>
                <div class="about-meta mini">
                {#if repoUrl}<a class="link" href={repoUrl} target="_blank">GitHub</a>{/if}
                <span class="faint">Made with Svelte</span>
                </div>
            {:else if activePopup === 'library'}
                <div class="empty-state mini"><p>Manage Paths</p></div>
            {:else if activePopup === 'appearance'}
                <div class="action-grid mini">
                    <button class="action-row mini" on:click={() => handleAction('theme')}>
                    <div class="action-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 21a9 9 0 1 0 0-18c4.97 0 9 2 9 6Z"/></svg></div>
                    <span>Dark Mode</span>
                    </button>
                </div>
            {/if}
            </div>
        </div>
        <div class="popup-glow"></div>
      </dialog>
    </div>
  {/if}
</div>

<style>
  /* Base Styles */
  .settings-container { position: absolute; inset: 0; display: flex; justify-content: center; align-items: center; pointer-events: none; margin-left: -105px; }
  .seed { position: absolute; width: 96px; height: 96px; border-radius: 50%; z-index: 30; pointer-events: none; background: linear-gradient(180deg, rgba(254, 214, 169, 0.85) 0%, rgba(244, 202, 167, 0.95) 55%, rgba(255, 189, 245, 0.8) 100%); box-shadow: inset 1px 2px 18px rgba(255, 255, 255, 0.75), 0 0 95px rgba(255, 220, 180, 0.75), 0 0 140px rgba(255, 200, 150, 0.35); filter: brightness(1.08); animation: seedGlowAlways 0.22s ease-in-out infinite alternate; transition: transform 110ms ease-out; }
  .seed::before { content: ""; position: absolute; inset: -26px; border-radius: 50%; background: radial-gradient(circle, rgba(255, 230, 200, 0.55), rgba(255, 200, 160, 0) 65%); filter: blur(16px); opacity: 1; pointer-events: none; }
  @keyframes seedGlowAlways { from { filter: brightness(1.05); box-shadow: inset 1px 2px 18px rgba(255, 255, 255, 0.72), 0 0 85px rgba(255, 220, 180, 0.70), 0 0 120px rgba(255, 200, 150, 0.30); } to { filter: brightness(1.22); box-shadow: inset 0 0 24px rgba(255, 255, 255, 0.92), 0 0 130px rgba(255, 220, 180, 0.95), 0 0 210px rgba(255, 200, 150, 0.55); } }
  .seed-tick { position: absolute; inset: 0; border-radius: 50%; animation: seedTickStrong 0.18s cubic-bezier(0.2, 0.9, 0.2, 1) forwards; pointer-events: none; }
  @keyframes seedTickStrong { 0% { transform: scale(1); filter: brightness(1); box-shadow: none; opacity: 1; } 40% { transform: scale(1.16); filter: brightness(1.35); box-shadow: 0 0 35px rgba(255, 255, 255, 0.55), 0 0 95px rgba(255, 220, 180, 0.90), 0 0 160px rgba(255, 200, 150, 0.60); opacity: 1; } 100% { transform: scale(1); filter: brightness(1); box-shadow: none; opacity: 1; } }

  .bubble { pointer-events: auto; position: absolute; width: 96px; height: 96px; margin-left: -48px; margin-top: -48px; border: none; outline: none; cursor: pointer; padding: 0; background: transparent; }
  .bubble-skin { width: 100%; height: 100%; border-radius: 50%; display: flex; justify-content: center; align-items: center; background: linear-gradient(180deg, rgba(254, 214, 169, 0.85) 0%, rgba(244, 202, 167, 0.95) 55%, rgba(255, 189, 245, 0.8) 100%); box-shadow: inset 1px 2px 15px rgba(255, 255, 255, 0.7), inset -1px -2px 10px rgba(0, 0, 0, 0.05), 0 12px 28px rgba(94, 75, 75, 0.15); transition: box-shadow 0.3s ease, filter 0.2s ease; animation: floatY 3.2s ease-in-out infinite; animation-delay: calc(var(--float-start-delay) + var(--float-delay)); }
  .bubble-skin.returning { animation: none; }
  @keyframes floatY { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-8px); } }
  .bubble:hover .bubble-skin { filter: brightness(1.05); box-shadow: inset 0 0 20px rgba(255, 255, 255, 0.95), 0 0 25px rgba(255, 220, 180, 0.6), 0 15px 40px rgba(94, 75, 75, 0.2); }
  @keyframes clickPop { 0% { transform: scale(1); } 40% { transform: scale(1.06); filter: brightness(1.1); } 100% { transform: scale(1); filter: brightness(1); } }
  .bubble-skin.click-pulse .bubble-content { animation: clickPop 0.18s ease-out forwards; }
  .glow-layer { position: absolute; inset: 0; border-radius: 50%; pointer-events: none; opacity: 0; transition: opacity 0.35s ease; box-shadow: inset 0 0 12px rgba(255, 255, 255, 0.9); }
  .bubble:hover .glow-layer { opacity: 1; }
  .bubble-content { position: relative; z-index: 2; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 6px; pointer-events: none; color: #5e4b4b; transform-origin: center center; }
  .icon { width: 28px; height: 28px; opacity: 0.85; filter: drop-shadow(0 1px 1px rgba(255, 255, 255, 0.5)); transition: opacity 0.3s ease; }
  .bubble:hover .icon { opacity: 1; }
  .label { font-family: 'Playfair Display', serif; font-size: 0.7rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.08em; opacity: 0.8; margin-top: 1px; transition: opacity 0.3s ease; }
  .bubble:hover .label { opacity: 1; }

  /* --- POPUP STYLES --- */
  
  .popup-backdrop { 
    /* Enable clicks! */
    pointer-events: auto; 
    position: fixed; inset: 0; border: 0; padding: 0; 
    background: transparent; 
    cursor: default; z-index: 9998; 
  }

  .popup-anchor {
    position: absolute;
    z-index: 9999;
    /* Enable interactions in the anchor area */
    pointer-events: auto;
  }

  /* THEMED CARD STYLE */
  .popup-card.themed {
    /* Explicitly allow interaction inside the card */
    pointer-events: auto;
    position: relative;
    width: 220px;
    border-radius: 20px; 
    color: #5e4b4b;
    background: linear-gradient(135deg, rgba(255, 240, 230, 0.95) 0%, rgba(255, 225, 210, 0.9) 100%);
    box-shadow: 
      0 10px 40px rgba(94, 75, 75, 0.2), 
      inset 0 1px 0 rgba(255, 255, 255, 0.8),
      inset 0 0 20px rgba(255, 200, 150, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.6);
    padding: 0;
    margin: 0;
    overflow: hidden;
  }
  
  .popup-glow {
    position: absolute; inset: 0; z-index: -1; pointer-events: none;
    background: radial-gradient(circle at top left, rgba(255, 180, 140, 0.15), transparent 70%);
  }

  .popup-content-wrapper { position: relative; z-index: 2; }

  .popup-header.mini { padding: 10px 12px; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid rgba(94, 75, 75, 0.06); }
  .popup-title h2 { font-size: 0.9rem; margin: 0; font-weight: 700; font-family: 'Playfair Display', serif; letter-spacing: 0.02em; }
  
  .popup-close-btn { 
    /* Enable pointer events on button explicitly */
    pointer-events: auto;
    width: 24px; height: 24px; padding: 0; border: none; background: transparent; 
    color: #5e4b4b; opacity: 0.6; cursor: pointer; display: grid; place-items: center; 
    border-radius: 50%; transition: all 0.2s ease;
  }
  .popup-close-btn:hover { opacity: 1; background: rgba(94, 75, 75, 0.1); transform: scale(1.05); }
  .popup-close-btn svg { width: 14px; height: 14px; }

  .popup-body.mini { padding: 8px; }
  
  .action-grid.mini { display: flex; flex-direction: column; gap: 4px; }
  .action-row.mini {
    /* Enable pointer events on actions explicitly */
    pointer-events: auto;
    display: flex; align-items: center; gap: 10px; 
    padding: 8px 10px; border-radius: 10px; 
    border: none; background: rgba(255, 255, 255, 0.4); 
    text-align: left; cursor: pointer; color: #5e4b4b;
    transition: background 0.2s, transform 0.1s;
    border: 1px solid rgba(255,255,255,0.3);
  }
  .action-row.mini:hover { background: rgba(255, 255, 255, 0.75); transform: translateY(-1px); box-shadow: 0 2px 5px rgba(0,0,0,0.03); }
  .action-row.mini span { font-size: 0.8rem; font-weight: 600; opacity: 0.9; }
  .action-icon { width: 16px; height: 16px; opacity: 0.75; }
  
  .action-row.mini.danger { color: #a63a3a; background: rgba(255, 230, 230, 0.4); border-color: rgba(255, 200, 200, 0.3); }
  .action-row.mini.danger:hover { background: rgba(255, 220, 220, 0.6); }

  .popup-text.mini { margin: 6px 6px 12px 6px; font-size: 0.8rem; opacity: 0.85; line-height: 1.4; }
  .about-meta.mini { display: flex; flex-direction: column; gap: 4px; padding: 0 6px; }
  .link { font-size: 0.8rem; color: #5e4b4b; font-weight: 700; text-decoration: none; border-bottom: 1px solid rgba(94, 75, 75, 0.3); display: inline-block; width: fit-content; pointer-events: auto; }
  .faint { font-size: 0.7rem; opacity: 0.5; font-style: italic; }
  .empty-state.mini { padding: 12px; text-align: center; font-size: 0.8rem; opacity: 0.6; font-style: italic; }
</style>
