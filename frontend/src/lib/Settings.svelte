<script lang="ts">
  import { fly, scale } from 'svelte/transition';
  import { elasticOut, cubicOut, backOut } from 'svelte/easing';
  import { createEventDispatcher, tick } from 'svelte';

  export let stage: "idle" | "collapsing" | "glowing" | "dividing" | "gathering" = "idle";

  // Optional: feed real values from your app shell (Tauri, package.json, etc.)
  export let appName = "Hikari";
  export let version: string | null = null;
  export let tagline = "A gentle library for your books.";
  export let repoUrl: string | null = null;

  const dispatch = createEventDispatcher<{
    readyToExpand: void;
    select: { id: string };
  }>();

  const settingsOptions = [
    { id: 'export', label: 'Backup',  path: 'M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M7 10l5 5 5-5 M12 15V3' },
    { id: 'import', label: 'Restore', path: 'M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M17 8l-5-5-5 5 M12 3v12' },
    { id: 'theme',  label: 'Theme',   path: 'M12 21a9 9 0 1 0 0-18c4.97 0 9 2 9 6Z M19 10a1 1 0 1 1 0 2 1 1 0 0 1 0-2Z M15 16a1 1 0 1 1 0 2 1 1 0 0 1 0-2Z M9 16a1 1 0 1 1 0 2 1 1 0 0 1 0-2Z' },
    { id: 'clear',  label: 'Clear',   path: 'M3 6h18 M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2 M10 11v6 M14 11v6' },
    { id: 'about',  label: 'About',   path: 'M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z M12 16v-4 M12 8h.01' }
  ].map((o, idx) => ({ ...o, idx }));

  const total = settingsOptions.length;

  // division/gather reveal
  let revealCount = 0;
  let timer: number | null = null;
  let lastStage: typeof stage = "idle";

  const spawnEveryMs = 90;
  const gatherEveryMs = 85;
  const flyDurationMs = 520;
  const scaleDurationMs = 220;

  // pulse tick key (fires each spawn/collect)
  let pulseKey = 0;

  function stop() {
    if (timer != null) {
      clearInterval(timer);
      timer = null;
    }
  }

  function startDividing() {
    stop();
    revealCount = 0;
    pulseKey = 0;

    timer = window.setInterval(() => {
      revealCount += 1;
      pulseKey += 1;

      if (revealCount >= total) {
        revealCount = total;
        stop();
      }
    }, spawnEveryMs);
  }

  function startGathering() {
    stop();
    pulseKey = 0;

    // start from all bubbles visible
    revealCount = total;

    timer = window.setInterval(() => {
      revealCount -= 1;
      pulseKey += 1;

      if (revealCount <= 0) {
        revealCount = 0;
        stop();

        window.setTimeout(() => {
          dispatch('readyToExpand');
        }, 180);
      }
    }, gatherEveryMs);
  }

  $: {
    if (stage !== lastStage) {
      if (stage === "dividing") startDividing();
      else if (stage === "gathering") startGathering();
      else {
        stop();
        if (stage === "idle") revealCount = 0;
      }
      lastStage = stage;
    }
  }

  // Keep bubbles mounted only during dividing/gathering
  $: visibleOptions =
    (stage === "dividing" || stage === "gathering")
      ? settingsOptions.slice(0, revealCount)
      : [];

  // Seed visible during all non-idle stages
  $: seedVisible =
    stage === "collapsing" ||
    stage === "glowing" ||
    stage === "dividing" ||
    stage === "gathering";

  // Seed scale: shrink while dividing, grow while gathering
  $: seedScale = (() => {
    if (stage === "dividing") {
      return Math.max(0.18, 1 - revealCount / (total * 1.1));
    }
    if (stage === "gathering") {
      const collected = total - revealCount;
      return Math.min(1, 0.18 + (collected / total) * 0.82);
    }
    return 1;
  })();

  function getPosition(index: number, totalCount: number, radius: number) {
    const angle = (index / totalCount) * 2 * Math.PI - (Math.PI / 2);
    return { x: Math.cos(angle) * radius, y: Math.sin(angle) * radius };
  }

  let pulsingId: string | null = null;

  // ABOUT modal state + DOM ref
  let aboutOpen = false;
  let aboutDialog: HTMLDialogElement | null = null;

  async function openAbout() {
    aboutOpen = true;
    await tick();              // wait until dialog exists in DOM [web:87]
    aboutDialog?.focus();      // bind:this gives the element reference [web:71]
  }

  function closeAbout() {
    aboutOpen = false;
  }

  function onWindowKeydown(e: KeyboardEvent) {
    if (!aboutOpen) return;
    if (e.key === 'Escape') closeAbout();
  }

  function handleClick(id: string) {
    pulsingId = id;

    if (id === 'about') {
      openAbout();
    } else {
      dispatch('select', { id });
    }

    setTimeout(() => (pulsingId = null), 180);
  }
</script>

<svelte:window on:keydown={onWindowKeydown} />

<div class="settings-container">
  {#if seedVisible}
    <div
      class="seed"
      out:scale={{ duration: 260, easing: cubicOut, start: 0 }}
      style="transform: scale({seedScale});"
    >
      {#if stage === "dividing" || stage === "gathering"}
        {#key pulseKey}
          <div class="seed-tick"></div>
        {/key}
      {/if}
    </div>
  {/if}

  {#each visibleOptions as item (item.id)}
    {@const pos = getPosition(item.idx, total, 145)}

    <button
      class="bubble"
      style="
        left: calc(50% + {pos.x}px);
        top: calc(50% + {pos.y}px);
        --float-delay: -{item.idx * 1.2}s;
        --float-start-delay: {flyDurationMs + item.idx * 30}ms;
      "
      in:fly={{
        x: -pos.x,
        y: -pos.y,
        duration: flyDurationMs,
        easing: elasticOut
      }}
      out:fly={{
        x: -pos.x,
        y: -pos.y,
        duration: 420,
        easing: backOut
      }}
      on:click={() => handleClick(item.id)}
      aria-label={item.label}
      type="button"
    >
      <div
        class="bubble-skin"
        class:returning={stage === "gathering"}
        class:click-pulse={pulsingId === item.id}
        in:scale={{ start: 0, duration: scaleDurationMs, easing: backOut }}
        out:scale={{ start: 0, duration: 220, easing: cubicOut }}
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
            aria-hidden="true"
          >
            <path d={item.path} />
          </svg>
          <span class="label">{item.label}</span>
        </div>
        <div class="glow-layer"></div>
      </div>
    </button>
  {/each}

  {#if aboutOpen}
    <div class="about-overlay">
      <!-- full-screen click target -->
      <button
        class="about-backdrop"
        type="button"
        aria-label="Close About"
        on:click={closeAbout}
      ></button>

      <!-- Wrapper gets fly (can't combine multiple in: on same element) -->
      <div
        class="about-fly"
        in:fly={{ y: 12, duration: 200, easing: cubicOut }}
        out:fly={{ y: 12, duration: 170, easing: cubicOut }}
      >
        <!-- Dialog gets scale -->
        <dialog
          class="about-card"
          open
          aria-labelledby="about-title"
          bind:this={aboutDialog}
          tabindex="-1"
          on:click|stopPropagation
          in:scale={{ start: 0.94, duration: 170, easing: cubicOut }}
          out:scale={{ start: 0.94, duration: 140, easing: cubicOut }}
        >
          <header class="about-header">
            <div class="about-orb" aria-hidden="true"></div>

            <div class="about-title">
              <h2 id="about-title">{appName}</h2>
              {#if version}
                <p class="about-sub">Version {version}</p>
              {:else}
                <p class="about-sub">{tagline}</p>
              {/if}
            </div>

            <button class="about-close" type="button" on:click={closeAbout} aria-label="Close About">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                <path d="M18 6 6 18M6 6l12 12" />
              </svg>
            </button>
          </header>

          <div class="about-body">
            <p class="about-text">
              Hikari is built to keep your library calm, searchable, and beautiful — like a softly lit reading nook.
            </p>

            <div class="about-meta">
              <div class="meta-row">
                <span class="meta-k">Motion</span>
                <span class="meta-v">Orb seed → divide → gather</span>
              </div>
              <div class="meta-row">
                <span class="meta-k">Palette</span>
                <span class="meta-v">Warm paper + sakura glow</span>
              </div>
              {#if repoUrl}
                <div class="meta-row">
                  <span class="meta-k">Repo</span>
                  <a class="meta-v link" href={repoUrl} target="_blank" rel="noreferrer">Open</a>
                </div>
              {/if}
            </div>
          </div>

          <footer class="about-footer">
            <button class="about-action" type="button" on:click={closeAbout}>
              Back to settings
            </button>
          </footer>
        </dialog>
      </div>
    </div>
  {/if}
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

  /* SEED */
  .seed {
    position: absolute;
    width: 96px;
    height: 96px;
    border-radius: 50%;
    z-index: 30;
    pointer-events: none;

    background: linear-gradient(
      180deg,
      rgba(254, 214, 169, 0.85) 0%,
      rgba(244, 202, 167, 0.95) 55%,
      rgba(255, 189, 245, 0.8) 100%
    );

    box-shadow:
      inset 1px 2px 18px rgba(255, 255, 255, 0.75),
      0 0 95px rgba(255, 220, 180, 0.75),
      0 0 140px rgba(255, 200, 150, 0.35);

    filter: brightness(1.08);
    animation: seedGlowAlways 0.22s ease-in-out infinite alternate;
    transition: transform 110ms ease-out;
  }

  .seed::before {
    content: "";
    position: absolute;
    inset: -26px;
    border-radius: 50%;
    background: radial-gradient(circle, rgba(255, 230, 200, 0.55), rgba(255, 200, 160, 0) 65%);
    filter: blur(16px);
    opacity: 1;
    pointer-events: none;
  }

  @keyframes seedGlowAlways {
    from {
      filter: brightness(1.05);
      box-shadow:
        inset 1px 2px 18px rgba(255, 255, 255, 0.72),
        0 0 85px rgba(255, 220, 180, 0.70),
        0 0 120px rgba(255, 200, 150, 0.30);
    }
    to {
      filter: brightness(1.22);
      box-shadow:
        inset 0 0 24px rgba(255, 255, 255, 0.92),
        0 0 130px rgba(255, 220, 180, 0.95),
        0 0 210px rgba(255, 200, 150, 0.55);
    }
  }

  .seed-tick {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    animation: seedTickStrong 0.18s cubic-bezier(0.2, 0.9, 0.2, 1) forwards;
    pointer-events: none;
  }

  @keyframes seedTickStrong {
    0% { transform: scale(1); filter: brightness(1); box-shadow: none; opacity: 1; }
    40% {
      transform: scale(1.16);
      filter: brightness(1.35);
      box-shadow:
        0 0 35px rgba(255, 255, 255, 0.55),
        0 0 95px rgba(255, 220, 180, 0.90),
        0 0 160px rgba(255, 200, 150, 0.60);
      opacity: 1;
    }
    100% { transform: scale(1); filter: brightness(1); box-shadow: none; opacity: 1; }
  }

  .bubble {
    pointer-events: auto;
    position: absolute;
    width: 96px;
    height: 96px;
    margin-left: -48px;
    margin-top: -48px;
    border: none;
    outline: none;
    cursor: pointer;
    padding: 0;
    background: transparent;
  }

  .bubble-skin {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;

    background: linear-gradient(
      180deg,
      rgba(254, 214, 169, 0.85) 0%,
      rgba(244, 202, 167, 0.95) 55%,
      rgba(255, 189, 245, 0.8) 100%
    );

    box-shadow:
      inset 1px 2px 15px rgba(255, 255, 255, 0.7),
      inset -1px -2px 10px rgba(0, 0, 0, 0.05),
      0 12px 28px rgba(94, 75, 75, 0.15);

    transition: box-shadow 0.3s ease, filter 0.2s ease;

    animation: floatY 3.2s ease-in-out infinite;
    animation-delay: calc(var(--float-start-delay) + var(--float-delay));
  }

  .bubble-skin.returning {
    animation: none;
  }

  @keyframes floatY {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-8px); }
  }

  .bubble:hover .bubble-skin {
    filter: brightness(1.05);
    box-shadow:
      inset 0 0 20px rgba(255, 255, 255, 0.95),
      0 0 25px rgba(255, 220, 180, 0.6),
      0 15px 40px rgba(94, 75, 75, 0.2);
  }

  @keyframes clickPop {
    0% { transform: scale(1); }
    40% { transform: scale(1.06); filter: brightness(1.1); }
    100% { transform: scale(1); filter: brightness(1); }
  }

  .bubble-skin.click-pulse .bubble-content {
    animation: clickPop 0.18s ease-out forwards;
  }

  .glow-layer {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    pointer-events: none;
    opacity: 0;
    transition: opacity 0.35s ease;
    box-shadow: inset 0 0 12px rgba(255, 255, 255, 0.9);
  }

  .bubble:hover .glow-layer { opacity: 1; }

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
    transform-origin: center center;
  }

  .icon {
    width: 28px;
    height: 28px;
    opacity: 0.85;
    filter: drop-shadow(0 1px 1px rgba(255, 255, 255, 0.5));
    transition: opacity 0.3s ease;
  }

  .bubble:hover .icon { opacity: 1; }

  .label {
    font-family: 'Playfair Display', serif;
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    opacity:  0.8;
    margin-top: 1px;
    transition: opacity 0.3s ease;
  }

  .bubble:hover .label { opacity: 1; }

  /* ABOUT OVERLAY */
  .about-overlay {
    position: absolute;
    inset: 0;
    z-index: 120;
    pointer-events: auto;

    display: grid;
    place-items: center;
  }

  /* It's a button now; style it as a backdrop */
  .about-backdrop {
    position: absolute;
    inset: 0;

    border: 0;
    padding: 0;
    background: radial-gradient(circle at 50% 45%,
      rgba(255, 240, 225, 0.55),
      rgba(25, 18, 18, 0.28)
    );
    backdrop-filter: blur(10px);

    cursor: pointer;
  }

  .about-fly {
    position: relative;
    z-index: 1;
  }

  .about-card[open] {
    width: min(520px, calc(100vw - 34px));
    border-radius: 22px;
    overflow: hidden;

    color: #5e4b4b;

    background:
      radial-gradient(circle at 18% 10%, rgba(255, 255, 255, 0.95), rgba(255, 255, 255, 0.72) 55%, rgba(255, 255, 255, 0.62) 100%),
      linear-gradient(180deg,
        rgba(254, 214, 169, 0.25) 0%,
        rgba(244, 202, 167, 0.22) 55%,
        rgba(255, 189, 245, 0.18) 100%
      );

    box-shadow:
      0 28px 80px rgba(15, 10, 10, 0.35),
      inset 0 1px 0 rgba(255, 255, 255, 0.65);

    border: 1px solid rgba(255, 255, 255, 0.55);

    /* remove default dialog look */
    padding: 0;
  }

  .about-header {
    display: grid;
    grid-template-columns: 56px 1fr 42px;
    gap: 12px;
    align-items: center;
    padding: 18px 18px 14px 18px;
  }

  .about-orb {
    width: 56px;
    height: 56px;
    border-radius: 50%;

    background: linear-gradient(
      180deg,
      rgba(254, 214, 169, 0.9) 0%,
      rgba(244, 202, 167, 0.95) 55%,
      rgba(255, 189, 245, 0.82) 100%
    );

    box-shadow:
      inset 1px 2px 16px rgba(255, 255, 255, 0.85),
      0 0 38px rgba(255, 220, 180, 0.55),
      0 0 70px rgba(255, 200, 150, 0.24);
  }

  .about-title h2 {
    margin: 0;
    font-family: 'Playfair Display', serif;
    font-weight: 700;
    letter-spacing: 0.02em;
    font-size: 1.25rem;
  }

  .about-sub {
    margin: 2px 0 0 0;
    opacity: 0.78;
    font-size: 0.9rem;
  }

  .about-close {
    width: 42px;
    height: 42px;
    border-radius: 14px;
    border: 1px solid rgba(255, 255, 255, 0.55);
    background: rgba(255, 255, 255, 0.55);
    color: #5e4b4b;
    cursor: pointer;
    display: grid;
    place-items: center;
    transition: filter 0.15s ease, transform 0.15s ease, background 0.2s ease;
  }

  .about-close:hover {
    filter: brightness(1.04);
    transform: translateY(-1px);
    background: rgba(255, 255, 255, 0.7);
  }

  .about-close svg {
    width: 18px;
    height: 18px;
    opacity: 0.9;
  }

  .about-body {
    padding: 0 18px 16px 18px;
  }

  .about-text {
    margin: 0 0 14px 0;
    line-height: 1.5;
    opacity: 0.9;
  }

  .about-meta {
    border-radius: 16px;
    padding: 12px 12px;
    background: rgba(255, 255, 255, 0.48);
    border: 1px solid rgba(255, 255, 255, 0.55);
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.55);
  }

  .meta-row {
    display: grid;
    grid-template-columns: 90px 1fr;
    gap: 10px;
    padding: 8px 6px;
  }

  .meta-k {
    font-weight: 700;
    font-size: 0.78rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    opacity: 0.65;
  }

  .meta-v {
    opacity: 0.9;
  }

  .link {
    color: #5e4b4b;
    text-decoration: none;
    border-bottom: 1px solid rgba(94, 75, 75, 0.35);
  }

  .link:hover {
    border-bottom-color: rgba(94, 75, 75, 0.65);
  }

  .about-footer {
    padding: 14px 18px 18px 18px;
    display: flex;
    justify-content: flex-end;
  }

  .about-action {
    border: none;
    cursor: pointer;
    border-radius: 999px;
    padding: 10px 14px;

    font-family: 'Playfair Display', serif;
    font-weight: 700;
    letter-spacing: 0.03em;

    color: #5e4b4b;
    background: linear-gradient(
      180deg,
      rgba(254, 214, 169, 0.9) 0%,
      rgba(244, 202, 167, 0.92) 55%,
      rgba(255, 189, 245, 0.78) 100%
    );

    box-shadow:
      inset 0 1px 0 rgba(255, 255, 255, 0.7),
      0 10px 30px rgba(94, 75, 75, 0.18);

    transition: transform 0.15s ease, filter 0.15s ease;
  }

  .about-action:hover {
    transform: translateY(-1px);
    filter: brightness(1.03);
  }
</style>
