<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import Sidebar from './lib/Sidebar.svelte';
  import Library from './lib/Library.svelte';

  const appWindow = getCurrentWindow();

  async function closeApp() {
    await appWindow.close();
  }

  // UI state
  let bookTitle = "";
  let isPulsing = false;
  let isFocused = false;
  let activeTab: "home" | "menu" = "home";
  let previousTab: "home" | "menu" = activeTab;

  // Return animation state (home bounce after leaving library)
  type ReturnStage = "idle" | "fading" | "bouncing_down" | "bouncing_up";
  let returnStage: ReturnStage = "idle";
  let isReturning = false;

  // Search / mock result state
  type SearchState = "idle" | "loading" | "result";
  let searchState: SearchState = "idle";

  type MockBook = {
    title: string;
    author: string;
    year: string;
    pages: string;
    summary: string;
  };

  let foundBook: MockBook | null = null;

  // Reference to the orb element for smooth float → center transition
  let orbEl: HTMLDivElement | null = null;

  // Tab change effects
  $: if (activeTab !== previousTab) {
    if (previousTab === "menu" && activeTab === "home") {
      triggerBounceSequence();
    }
    previousTab = activeTab;
  }

  async function triggerBounceSequence() {
    isReturning = true;

    returnStage = "fading";
    await wait(250);

    returnStage = "bouncing_down";
    await wait(700);

    returnStage = "bouncing_up";
    await wait(600);

    returnStage = "idle";
    isReturning = false;
  }

  function wait(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  // Derived visual states
  $: isGlowing =
    (isReturning && returnStage !== "idle") ||
    (isFocused && activeTab === "home") ||
    searchState === "loading" ||
    searchState === "result";

  $: shouldScale = isFocused && activeTab === "home" && !isReturning;

  // Input interactions
  function handleInput() {
    if (activeTab !== "home") return;
    isPulsing = true;
    setTimeout(() => (isPulsing = false), 100);
  }

  function handleFocus() {
    if (activeTab !== "home") return;
    isFocused = true;
    settleOrbToCenter();
  }

  function handleBlur() {
    isFocused = false;
    restoreOrbFloat();
  }

  async function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && bookTitle.trim() && searchState === "idle") {
      const query = bookTitle.trim();
      bookTitle = "";
      searchState = "loading";

      // mock "search"
      await wait(1200);

      foundBook = {
        title: query,
        author: "Haruki Murakami",
        year: "2002",
        pages: "384",
        summary:
          "A quiet, introspective novel about memory, routine, and the small rituals that anchor a life."
      };

      searchState = "result";
    }
  }

  function handleAdd() {
    if (foundBook) {
      console.log("Add book:", foundBook);
    }
    resetSearch();
  }

  function handleDiscard() {
    resetSearch();
  }

  function resetSearch() {
    searchState = "idle";
    foundBook = null;
  }

  /**
   * Freeze the current float animation frame and transition
   * the orb smoothly back to the neutral vertical center.
   */
  function settleOrbToCenter() {
    if (!orbEl) return;

    const el = orbEl;
    const computed = getComputedStyle(el);
    const currentTransform =
      computed.transform === "none" ? "" : computed.transform;

    el.style.animation = "none";
    el.style.transform = currentTransform;

    requestAnimationFrame(() => {
      el.style.transition =
        "transform 0.6s cubic-bezier(0.25, 0.8, 0.25, 1)";
      el.style.transform = "translateY(0)";
    });
  }

  /**
   * Restore the default float animation after focus leaves.
   */
  function restoreOrbFloat() {
    if (!orbEl) return;
    orbEl.style.transition = "";
    orbEl.style.transform = "";
    orbEl.style.animation = "";
  }
</script>

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

<main>
  <div class="ambient-light one"></div>
  <div class="ambient-light two"></div>

  <Sidebar bind:activeTab={activeTab} />

  <section class="orb-stage">
    <div
      class="orb-floater"
      class:expanded-floater={activeTab === "menu" || (isReturning && returnStage === "fading")}
    >
      <div
        class="orb"
        bind:this={orbEl}
        class:expanded={activeTab === "menu" || (isReturning && returnStage === "fading")}
        class:small-orb={isReturning && returnStage === "bouncing_down"}
        class:glowing={isGlowing}
        class:typing-scale={shouldScale}
        class:pulsing={isPulsing && activeTab === "home"}
      >
        {#if activeTab === "home" && !isReturning}
          {#if searchState === "idle"}
            <div class="glass-capsule fade-in-delayed">
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
          {:else if searchState === "loading"}
            <div class="search-loading">
              <div class="loading-circle"></div>
            </div>
          {:else if searchState === "result" && foundBook}
            <div class="book-result">
              <div class="book-card">
                <div class="book-cover"></div>
                <div class="book-info">
                  <h2>{foundBook.title}</h2>
                  <p class="book-meta">
                    {foundBook.author} · {foundBook.year} · {foundBook.pages} pages
                  </p>
                  <p class="book-summary">{foundBook.summary}</p>
                </div>
              </div>
              <div class="book-actions">
                <button class="pill-btn pill-add" on:click={handleAdd}>
                  Add
                </button>
                <button class="pill-btn pill-discard" on:click={handleDiscard}>
                  Discard
                </button>
              </div>
            </div>
          {/if}
        {:else if activeTab === "menu" || (isReturning && returnStage === "fading")}
          <div class="library-container" class:fade-out={returnStage === "fading"}>
            <Library />
          </div>
        {/if}
      </div>
    </div>
  </section>
</main>

<style>
  /* Global page layout */
  :global(body) {
    margin: 0;
    background: linear-gradient(180deg, #fff8f3 0%, #deaa84 100%);
    min-height: 100vh;
    height: 100vh;
    overflow: hidden;
    font-family: "Inter", sans-serif;
  }

  main {
    display: flex;
    height: 100vh;
    width: 100vw;
    position: relative;
  }

  /* Custom titlebar for Tauri window */
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

  /* Ambient background blobs */
  .ambient-light {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    opacity: 0.4;
    z-index: 0;
    animation: floatBlob 20s infinite ease-in-out alternate;
  }

  .one {
    width: 600px;
    height: 600px;
    background: #ffffff;
    top: -200px;
    left: -100px;
  }

  .two {
    width: 500px;
    height: 500px;
    background: #ffd1bc;
    bottom: -150px;
    right: -100px;
  }

  @keyframes floatBlob {
    0% {
      transform: translate(0, 0);
    }
    100% {
      transform: translate(40px, 60px);
    }
  }

  /* Orb container area */
  .orb-stage {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
    z-index: 5;
  }

  /* Idle float for the orb */
  @keyframes float {
    0% {
      transform: translateY(0px);
    }
    25% {
      transform: translateY(-12px);
    }
    75% {
      transform: translateY(12px);
    }
    100% {
      transform: translateY(0px);
    }
  }

  /* Positions the orb bubble in the scene */
  .orb-floater {
    width: 780px;
    height: 780px;
    margin-left: -160px;
    display: flex;
    justify-content: center;
    align-items: center;
    will-change: transform;
    transition:
      transform 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      margin 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      width 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      height 1.5s cubic-bezier(0.25, 1, 0.5, 1);
  }

  /* Library view: orb expands into a panel */
  .orb-floater.expanded-floater {
    width: 96%;
    height: 95%;
    margin-left: 0;
    padding: 20px;
    padding-right: 75px;
    box-sizing: border-box;
    transform: translateY(0);
  }

  /* Orb core: float, glow, and morph into library */
  .orb {
    position: relative;
    overflow: visible;
    width: 100%;
    height: 100%;
    border-radius: 50%;
    background: linear-gradient(
      180deg,
      rgb(254, 214, 169) 0%,
      rgba(244, 202, 167, 0.9) 60%,
      rgba(255, 189, 245, 0.7) 90%
    );
    box-shadow:
      inset 2px 4px 20px rgba(255, 255, 255, 0.6),
      inset -2px -4px 30px rgba(0, 0, 0, 0.05),
      0 25px 60px rgba(219, 168, 172, 0.35);
    display: flex;
    justify-content: center;
    align-items: center;
    max-width: 780px;
    max-height: 780px;
    transition:
      box-shadow 2.5s ease,
      width 0.7s cubic-bezier(0.25, 1, 0.5, 1),
      height 0.7s cubic-bezier(0.25, 1, 0.5, 1),
      border-radius 0.7s cubic-bezier(0.25, 1, 0.5, 1),
      background 1.5s ease,
      transform 0.6s cubic-bezier(0.25, 0.8, 0.25, 1),
      filter 0.6s ease;
    animation: float 8s ease-in-out infinite;
  }

  /* Rim-light glow layer */
  .orb::before {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: inherit;
    box-shadow:
      inset 0 0 30px rgba(255, 255, 255, 0.9),
      0 0 120px rgba(255, 220, 180, 0.8),
      0 0 200px rgba(255, 200, 150, 0.4);
    opacity: 0;
    pointer-events: none;
    transition: opacity 1.2s ease-out;
    z-index: -1;
  }

  .orb.glowing::before {
    opacity: 1;
  }

  .orb.glowing {
    box-shadow:
      inset 0 0 30px rgba(255, 255, 255, 0.9),
      0 0 120px rgba(255, 220, 180, 0.8),
      0 0 200px rgba(255, 200, 150, 0.4);
    filter: brightness(1.05);
  }

  .orb.small-orb {
    width: 60px !important;
    height: 60px !important;
    border-radius: 50% !important;
  }

  .orb.expanded {
    border-radius: 40px;
    width: 100%;
    height: 100%;
    max-width: 100%;
    max-height: 100%;
    display: block;
    padding: 40px;
    box-sizing: border-box;
    overflow-y: auto;
    cursor: default;
    scrollbar-width: none;
    transform: scale(1) !important;
    box-shadow:
      inset 0 0 30px rgba(255, 255, 255, 0.9),
      0 0 120px rgba(255, 220, 180, 0.8),
      0 0 200px rgba(255, 200, 150, 0.4);
    filter: brightness(1.05);
    transition:
      width 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      height 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      border-radius 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      background 1.5s ease,
      box-shadow 1.5s ease-in-out 0.3s;
  }

  .orb.expanded::-webkit-scrollbar {
    display: none;
  }

  /* Library content fade */
  .library-container {
    width: 100%;
    height: 100%;
    opacity: 1;
    transition: opacity 0.25s ease-out;
  }

  .library-container.fade-out {
    opacity: 0;
  }

  /* Glass input capsule */
  .glass-capsule {
    background: rgba(255, 255, 255, 0.3);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    padding: 18px 36px;
    border-radius: 100px;
    border: 1px solid rgba(255, 255, 255, 0.5);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.05);
    width: 340px;
    transition: all 0.3s ease;
  }

  .glass-capsule.fade-in-delayed {
    animation: fadeIn 0.8s ease forwards;
    opacity: 0;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .glass-capsule:focus-within {
    transform: scale(1.03);
    background: rgba(255, 255, 255, 0.45);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.08);
  }

  /* Loading mock state */
  .search-loading {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .loading-circle {
    width: 52px;
    height: 52px;
    border-radius: 50%;
    border: 3px solid rgba(255, 255, 255, 0.6);
    border-top-color: rgba(205, 132, 94, 1);
    box-shadow: 0 0 24px rgba(205, 132, 94, 0.4);
    animation: spin 0.9s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Mock book result card */
  .book-result {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
    max-width: 580px;
  }

  .book-card {
    display: flex;
    gap: 20px;
    align-items: stretch;
    background: rgba(255, 255, 255, 0.3);
    border-radius: 24px;
    padding: 22px 26px;
    border: 1px solid rgba(255, 255, 255, 0.6);
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.06);
  }

  .book-cover {
    width: 130px;
    min-width: 90px;
    border-radius: 16px;
    background: linear-gradient(145deg, #f0c3a3, #f7e4d3);
    box-shadow:
      0 10px 25px rgba(181, 119, 83, 0.35),
      inset 0 0 12px rgba(255, 255, 255, 0.7);
  }

  .book-info {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .book-info h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #4b332e;
  }

  .book-meta {
    margin: 0;
    font-size: 0.95rem;
    color: rgba(75, 51, 46, 0.7);
  }

  .book-summary {
    margin: 6px 0 0;
    font-size: 0.88rem;
    line-height: 1.4;
    color: rgba(75, 51, 46, 0.8);
  }

  .book-actions {
    display: flex;
    gap: 12px;
    justify-content: center;
  }

  .pill-btn {
    min-width: 110px;
    padding: 9px 22px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.7);
    background: rgba(255, 255, 255, 0.22);
    color: #5b3b30;
    font-size: 0.9rem;
    font-weight: 500;
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    cursor: pointer;
    transition:
      background 0.2s ease,
      transform 0.15s ease,
      box-shadow 0.2s ease;
  }

  .pill-btn:hover {
    background: rgba(255, 255, 255, 0.35);
    transform: translateY(-1px);
    box-shadow: 0 10px 25px rgba(181, 119, 83, 0.25);
  }

  .pill-add {
    border-color: rgba(255, 255, 255, 0.9);
  }

  .pill-discard {
    border-color: rgba(255, 255, 255, 0.6);
    background: rgba(255, 255, 255, 0.16);
  }

  /* Typing feedback */
  .orb.typing-scale {
    transform: scale(1.02);
    transition: transform 0.4s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  .orb.pulsing {
    transform: scale(1.05) !important;
    transition: transform 0.05s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  /* Text input */
  input {
    width: 100%;
    background: transparent;
    border: none;
    outline: none;
    font-size: 1.2rem;
    color: #5e4b4b;
    text-align: center;
    font-weight: 500;
    font-family: "Inter", sans-serif;
  }

  input::placeholder {
    color: rgba(94, 75, 75, 0.45);
    font-weight: 400;
    transition: opacity 0.15s ease-out;
  }

  /* On focus show only caret, no placeholder text */
  input:focus::placeholder {
    opacity: 0;
  }
</style>
