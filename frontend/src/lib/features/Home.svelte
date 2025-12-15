<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import { fade, scale, fly } from 'svelte/transition';
  import { cubicIn, cubicOut, elasticOut } from 'svelte/easing';
  import { invoke } from '@tauri-apps/api/core';

  // --- Components ---
  import Sidebar from '$lib/features/Sidebar.svelte';
  import Library from '$lib/features/Library.svelte';
  import Settings from '$lib/features/Settings.svelte';
  
  import Background from '$lib/home-components/Background.svelte';
  import Orb from '$lib/home-components/Orb.svelte';
  import BookSearchModule from '$lib/home-components/BookSearchModule.svelte';
  import AddBookDialog from '$lib/home-components/AddBookModal.svelte';
  import SummaryModal from '$lib/home-components/SummaryModal.svelte';
  import Toast from '$lib/home-components/Toast.svelte';  

  // #region --- STATE MANAGEMENT ---
  
  // Tab State
  let requestedTab: "home" | "menu" | "settings" = "home";
  let activeTab: "home" | "menu" | "settings" = "home";
  let previousTab: "home" | "menu" | "settings" = activeTab;

  // Animation State
  let settingsStage: "idle" | "collapsing" | "glowing" | "dividing" | "gathering" = "idle";
  let returnStage: "idle" | "fading" | "bouncing_down" | "bouncing_up" = "idle";
  let isOpeningSettings = false;
  let returningFromSettings = false;
  let showHomeExpandOrb = false;
  let overlayHoldVisible = false;
  let overlayGlowOff = false;
  let skipHomeIntroOnce = false;
  let suppressHomeBounceOnce = false;
  let isReturning = false;

  const OVERLAY_EXPAND_MS = 650;
  const OVERLAY_FADE_MS = 900;
  const OVERLAY_FADE_LEAD_MS = 250;

  // Search Data State
  let bookTitle = "";
  let searchState: "idle" | "loading" | "result" = "idle";
  let foundBooks: any[] = [];
  let isPulsing = false;
  let isFocused = false;
  let typingPulseTimeout: number | undefined;
  let selectedApi: "openlibrary" | "anilist" = "openlibrary";
  let lastQuery = "";
  let isApiSwitching = false;
  
  let apiResults: { [key in "openlibrary" | "anilist"]: any[] | null } = {
    openlibrary: null,
    anilist: null
  };

  // UI State
  let showAddDialog = false;
  let showDuplicateToast = false;
  let pendingBook: any | null = null;
  let summaryBook: any | null = null;
  let orbElement: HTMLDivElement | null = null;

  // Goal State
  let showGoalToast = false;
  let readingGoal = 10;
  let booksFinishedCount = 0;
  let toastTimer: number | null = null;
  let goalCheckInterval: number | null = null;

  // Computed Glow
  $: isGlowing =
    (isReturning && returnStage !== "idle") ||
    isFocused ||
    searchState !== "idle" ||
    showAddDialog ||
    isPulsing ||
    settingsStage === "collapsing" ||
    showHomeExpandOrb ||
    overlayHoldVisible;

  // #endregion

  // #region --- LIFECYCLE & WATCHERS ---

  onMount(() => {
    checkGoalCompletion();
    
    // Polling every 1.5s to catch updates instantly on any page
    goalCheckInterval = window.setInterval(() => {
        checkGoalCompletion();
    }, 1500);

    window.addEventListener('hikari-update', checkGoalCompletion);
  });

  onDestroy(() => {
    if (goalCheckInterval) clearInterval(goalCheckInterval);
    window.removeEventListener('hikari-update', checkGoalCompletion);
  });

  // Watch for requested tab changes
  $: if (requestedTab !== activeTab) {
    if (requestedTab === "home" || requestedTab === "menu") {
      checkGoalCompletion();
    }

    if (activeTab === "settings" && requestedTab === "home" && !returningFromSettings) {
      returningFromSettings = true;
      settingsStage = "gathering";
    } else if (!returningFromSettings) {
      activeTab = requestedTab;
    }
  }

  // Watch for active tab changes (Animation logic)
  $: if (activeTab !== previousTab) {
    if (activeTab === "home" && !suppressHomeBounceOnce) {
      triggerBounceSequence();
    }
    if (activeTab === "settings" && previousTab !== "settings") {
      restoreOrbFloat();
      isOpeningSettings = true;
      settingsStage = "collapsing";
    }
    if (previousTab === "settings" && activeTab !== "settings") {
      isOpeningSettings = false;
      settingsStage = "idle";
    }
    previousTab = activeTab;
  }

  // #endregion

  // #region --- ANIMATION CONTROLLERS ---

  async function wait(ms: number) {
    return new Promise<void>((r) => setTimeout(r, ms));
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

  function orbSettingsOut(_: Element, { duration, easing }: any) {
    if (isOpeningSettings) {
      return {
        duration,
        easing,
        css: (t: number) => {
          const inverted = 1 - t;
          const s = 0.15 + (0.85 * t);
          const x = -38 * inverted;
          return `transform: translateX(${x}px) scale(${s}); opacity: ${t};`;
        }
      };
    } else {
      return {
        duration,
        easing,
        css: (t: number) => `transform: scale(${0.12 + 0.88 * t}); opacity: ${t};`
      };
    }
  }

  async function handleOrbOutroEnd() {
    if (!isOpeningSettings) return;
    settingsStage = "glowing";
    await wait(240);
    if (activeTab === "settings") settingsStage = "dividing";
    isOpeningSettings = false;
  }

  async function handleSettingsReadyToExpand() {
    if (!returningFromSettings) return;

    overlayHoldVisible = true;
    overlayGlowOff = false;
    showHomeExpandOrb = true;

    const fadeStartAt = Math.max(0, OVERLAY_EXPAND_MS - OVERLAY_FADE_LEAD_MS);
    const fadeTimer = window.setTimeout(() => {
      overlayGlowOff = true;
    }, fadeStartAt);

    await wait(OVERLAY_EXPAND_MS);
    showHomeExpandOrb = false;

    suppressHomeBounceOnce = true;
    skipHomeIntroOnce = true;
    returningFromSettings = false;
    settingsStage = "idle";
    activeTab = "home";
    requestedTab = "home";

    await tick();

    suppressHomeBounceOnce = false;
    skipHomeIntroOnce = false;
    overlayGlowOff = true;
    clearTimeout(fadeTimer);

    const elapsed = OVERLAY_EXPAND_MS - fadeStartAt;
    const remaining = Math.max(0, OVERLAY_FADE_MS - elapsed);
    await wait(remaining);

    overlayHoldVisible = false;
    overlayGlowOff = false;
  }

  // #endregion

  // #region --- INTERACTION HANDLERS ---

  function handleFocus() {
    if (activeTab === "home") {
      isFocused = true;
      settleOrbToCenter();
    }
  }

  function handleBlur() {
    isFocused = false;
    restoreOrbFloat();
  }

  function handleInput() {
    if (activeTab === "home") {
      clearTimeout(typingPulseTimeout);
      isPulsing = true;
      typingPulseTimeout = window.setTimeout(() => (isPulsing = false), 100);
    }
  }

  function settleOrbToCenter() {
    if (!orbElement) return;
    const el = orbElement;
    const computed = window.getComputedStyle(el);
    const transform = computed.transform === 'none' ? '' : computed.transform;

    el.style.animation = "none";
    el.style.transform = transform;
    el.offsetHeight; 

    requestAnimationFrame(() => {
      el.style.transition = "transform 0.6s cubic-bezier(0.25, 0.8, 0.25, 1)";
      el.style.transform = "translateY(0)";
    });
  }

  function restoreOrbFloat() {
    if (!orbElement) return;
    orbElement.style.transition = "";
    orbElement.style.transform = "";
    orbElement.style.animation = "";
  }

  // #endregion

  // #region --- API & SEARCH LOGIC ---

  async function fetchOpenLibraryDescription(key: string): Promise<string> {
    try {
      const res = await fetch(`https://openlibrary.org${key}.json`);
      if (!res.ok) return "No description available.";
      const data = await res.json();
      let desc = "";
      if (typeof data.description === 'string') desc = data.description;
      else if (data.description?.value) desc = data.description.value;
      return desc || "No description available.";
    } catch {
      return "Failed to load description.";
    }
  }

  async function searchOpenLibrary(query: string): Promise<any[]> {
    try {
      const res = await fetch(
        `https://openlibrary.org/search.json?q=${encodeURIComponent(query)}&limit=10&fields=key,title,author_name,first_publish_year,number_of_pages_median,number_of_pages,cover_i,isbn,first_sentence`
      );
      if (!res.ok) return [];
      const data = await res.json();
      return (data.docs ?? []).map((doc: any) => ({
        title: doc.title ?? query,
        author: doc.author_name?.[0] ?? "Unknown author",
        year: doc.first_publish_year?.toString() ?? "—",
        pages: (doc.number_of_pages_median || doc.number_of_pages || 0).toString(),
        summary: doc.first_sentence?.[0] || "",
        fullSummary: null,
        coverUrl: doc.cover_i 
          ? `https://covers.openlibrary.org/b/id/${doc.cover_i}-M.jpg` 
          : (doc.isbn?.[0] ? `https://covers.openlibrary.org/b/isbn/${doc.isbn[0]}-M.jpg` : null),
        key: doc.key
      }));
    } catch (err) {
      console.error("OL Search Error:", err);
      return [];
    }
  }

  async function searchAnilist(query: string): Promise<any[]> {
    try {
      const queryGQL = `
        query ($search: String) {
          Page(perPage: 10) {
            media(search: $search, type: MANGA, sort: POPULARITY_DESC) {
              id
              title { romaji english }
              coverImage { large }
              description
              chapters
              startDate { year }
              staff(perPage: 1, sort: RELEVANCE) {
                edges { node { name { full } } }
              }
            }
          }
        }
      `;
      const res = await fetch('https://graphql.anilist.co', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'Accept': 'application/json' },
        body: JSON.stringify({ query: queryGQL, variables: { search: query } })
      });
      if (!res.ok) return [];
      const data = await res.json();
      return (data.data?.Page?.media ?? []).map((m: any) => {
        let desc = m.description || "";
        desc = desc.replace(/<br\s*\/?>/gi, '\n').replace(/<[^>]+>/g, '');
        return {
          title: m.title.english || m.title.romaji || query,
          author: m.staff.edges?.[0]?.node?.name?.full ?? "Unknown author",
          year: m.startDate?.year?.toString() ?? "—",
          pages: (m.chapters || 0).toString(),
          summary: desc.length > 200 ? desc.substring(0, 200) + "..." : desc,
          fullSummary: desc,
          coverUrl: m.coverImage?.large,
          key: null
        };
      });
    } catch (err) {
      console.error("AniList Search Error:", err);
      return [];
    }
  }

  async function handleApiSwitch(api: "openlibrary" | "anilist") {
    if (selectedApi === api || !lastQuery) return;
    selectedApi = api;
    if (apiResults[api]) {
      foundBooks = apiResults[api]!;
      return;
    }
    isApiSwitching = true;
    try {
      const results = api === "openlibrary" ? await searchOpenLibrary(lastQuery) : await searchAnilist(lastQuery);
      apiResults[api] = results;
      foundBooks = results;
    } catch {
      foundBooks = [];
    } finally {
      isApiSwitching = false;
    }
  }

  async function handleKeydown(event: CustomEvent<KeyboardEvent>) {
    const e = event.detail;
    if (e.key !== "Enter" || !bookTitle.trim() || searchState !== "idle") return;
    const query = bookTitle.trim();
    bookTitle = "";
    lastQuery = query;
    searchState = "loading";
    apiResults = { openlibrary: null, anilist: null };
    try {
      const [olRes, alRes] = await Promise.all([
        searchOpenLibrary(query),
        searchAnilist(query)
      ]);
      apiResults.openlibrary = olRes;
      apiResults.anilist = alRes;
      if (selectedApi === "openlibrary" && olRes.length) foundBooks = olRes;
      else if (selectedApi === "anilist" && alRes.length) foundBooks = alRes;
      else if (olRes.length) { selectedApi = "openlibrary"; foundBooks = olRes; }
      else if (alRes.length) { selectedApi = "anilist"; foundBooks = alRes; }
      else foundBooks = [];
      searchState = "result";
    } catch (err) {
      console.error(err);
      searchState = "idle";
    }
  }

  // #endregion

  // #region --- MODALS & GOAL LOGIC ---

  function handleAddRequest(event: CustomEvent) {
    pendingBook = event.detail;
    showAddDialog = true;
  }

  async function handleOpenSummary(event: CustomEvent) {
    const book = event.detail;
    if (book.key && !book.fullSummary) {
      summaryBook = { ...book, fullSummary: "Loading full description..." };
      const fullDesc = await fetchOpenLibraryDescription(book.key);
      summaryBook = { ...book, fullSummary: fullDesc, summary: fullDesc };
      const foundIndex = foundBooks.findIndex(b => b.key === book.key);
      if (foundIndex !== -1) {
        foundBooks[foundIndex].fullSummary = fullDesc;
        foundBooks[foundIndex].summary = fullDesc;
      }
    } else {
      summaryBook = book;
    }
  }

  async function saveBook(event: CustomEvent) {
    const { status, pagesRead, totalPages, book } = event.detail;
    isPulsing = true;
    setTimeout(() => (isPulsing = false), 600);
    showAddDialog = false;
    pendingBook = null;
    searchState = "idle";
    foundBooks = [];
    apiResults = { openlibrary: null, anilist: null };
    lastQuery = "";
    try {
      await invoke('add_book', {
        title: book.title,
        author: book.author,
        cover: book.coverUrl ?? "",
        status,
        pagesRead,
        totalPages
      });
      checkGoalCompletion();
    } catch {
      showDuplicateToast = true;
      setTimeout(() => (showDuplicateToast = false), 3000);
    }
  }

  // --- REFINED GOAL CHECK ---
  async function checkGoalCompletion() {
    try {
      const savedGoal = localStorage.getItem('hikari_reading_goal');
      const currentGoal = savedGoal ? parseInt(savedGoal, 10) : 10;
      readingGoal = currentGoal;

      // 1. Detect Goal Change (Settings) -> RESET
      const lastKnownGoalStr = localStorage.getItem('hikari_last_known_goal');
      const lastKnownGoal = lastKnownGoalStr ? parseInt(lastKnownGoalStr, 10) : null;
      
      if (lastKnownGoal !== currentGoal) {
        localStorage.setItem('hikari_goal_notified_master', 'false');
        localStorage.setItem('hikari_last_known_goal', currentGoal.toString());
      }

      // 2. Get Count
      const books: any[] = await invoke('get_books');
      booksFinishedCount = books.filter(b => b.status === 'finished').length;

      // 3. RE-ARM LOGIC: If we dropped BELOW the goal, reset the flag.
      // This allows the notification to trigger again when we go back UP.
      if (booksFinishedCount < currentGoal) {
        localStorage.setItem('hikari_goal_notified_master', 'false');
      }

      // 4. Notify
      const alreadyNotified = localStorage.getItem('hikari_goal_notified_master') === 'true';

      if (booksFinishedCount >= currentGoal && booksFinishedCount > 0) {
        if (!alreadyNotified) {
            showGoalToast = true;
            localStorage.setItem('hikari_goal_notified_master', 'true');
            if (toastTimer) clearTimeout(toastTimer);
            toastTimer = window.setTimeout(() => (showGoalToast = false), 6000);
        }
      }
    } catch (err) {
      // Silent fail during polling
    }
  }

  // #endregion
</script>

<main>
  <Background />
  <Sidebar bind:activeTab={requestedTab} />

  <section class="orb-stage">
    {#if activeTab === "settings"}
      <div class="settings-layer">
        <Settings stage={settingsStage} on:readyToExpand={handleSettingsReadyToExpand} />
      </div>
    {/if}

    {#if activeTab !== "settings"}
      <div
        class="orb-wrapper"
        in:scale={skipHomeIntroOnce ? { duration: 0 } : { duration: 600, easing: cubicOut, start: 0.2, delay: 200 }}
        out:orbSettingsOut={{ duration: 650, easing: cubicIn }}
        on:outroend={handleOrbOutroEnd}
      >
        <Orb
          bind:orbElement={orbElement}
          {activeTab}
          {isReturning}
          {returnStage}
          {isGlowing}
          {isPulsing}
          shouldScale={(isFocused && activeTab === "home" && !isReturning) || isPulsing}
          isAdding={showAddDialog}
        >
          {#if activeTab === "home" && !isReturning}
            <div class="search-container" in:fade={{ duration: 260, delay: 80 }} out:fade={{ duration: 180 }}>
              <BookSearchModule
                bind:bookTitle={bookTitle}
                {searchState}
                books={foundBooks}
                {selectedApi}
                {isApiSwitching}
                on:input={handleInput}
                on:keydown={handleKeydown}
                on:focus={handleFocus}
                on:blur={handleBlur}
                on:add={handleAddRequest}
                on:openSummary={handleOpenSummary}
                on:done={() => {
                  searchState = "idle";
                  foundBooks = [];
                  apiResults = { openlibrary: null, anilist: null };
                  lastQuery = "";
                }}
                on:apiSwitch={(e) => handleApiSwitch(e.detail)}
              />
            </div>
          {:else if activeTab === "menu" || (isReturning && returnStage === "fading")}
            <div class="library-container" class:fade-out={returnStage === "fading"} in:fade={{ duration: 400, delay: 700 }}>
              <Library on:change={checkGoalCompletion} on:update={checkGoalCompletion} />
            </div>
          {/if}
        </Orb>
      </div>
    {/if}

    {#if showHomeExpandOrb || overlayHoldVisible}
      <div
        class="orb-wrapper expand-overlay"
        class:fade-overlay={overlayGlowOff}
        in:scale={{ duration: 650, easing: cubicOut, start: 0.12 }}
      >
        <Orb
          bind:orbElement={orbElement}
          activeTab={"home"}
          isReturning={false}
          returnStage={"idle"}
          isGlowing={!overlayGlowOff}
          shouldScale={false}
          isPulsing={false}
          isAdding={false}
        />
      </div>
    {/if}
  </section>

  {#if summaryBook}
    <SummaryModal book={summaryBook} on:close={() => (summaryBook = null)} on:update={checkGoalCompletion} />
  {/if}

  {#if showAddDialog && pendingBook}
    <AddBookDialog book={pendingBook} on:cancel={() => (showAddDialog = false)} on:save={saveBook} />
  {/if}

  {#if showDuplicateToast}
    <Toast title="Book Exists" message="This book is already in your library." />
  {/if}

  {#if showGoalToast}
    <div class="goal-toast" transition:fly={{ y: -50, x: 20, duration: 800, easing: elasticOut }}>
      <div class="toast-icon-circle">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"/></svg>
      </div>
      <div class="toast-content">
        <div class="toast-title">Goal Reached!</div>
        <div class="toast-msg">You've finished {readingGoal} books.</div>
      </div>
      <button class="toast-close" on:click={() => (showGoalToast = false)} aria-label="Close notification">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M18 6 6 18M6 6l12 12"/>
        </svg>
      </button>
      <div class="toast-glow"></div>
    </div>
  {/if}
</main>

<style>
  /* #region --- LAYOUT & CONTAINERS --- */
  main { display: flex; height: 100vh; width: 100vw; position: relative; }
  
  .orb-stage { flex: 1; display: flex; justify-content: center; align-items: center; position: relative; z-index: 5; }
  
  .orb-wrapper, .settings-layer {
    width: 100%; height: 100%;
    display: flex; justify-content: center; align-items: center;
    position: absolute;
  }
  
  .orb-wrapper { transform-origin: 50% 50%; z-index: 2; }
  .settings-layer { z-index: 1; }
  .orb-wrapper.expand-overlay { z-index: 3; pointer-events: none; }
  
  .expand-overlay { opacity: 1; transition: opacity 900ms cubic-bezier(0.33, 0, 0.67, 1); }
  .expand-overlay.fade-overlay { opacity: 0; }

  .search-container, .library-container { width: 100%; height: 100%; }
  .search-container { display: flex; flex-direction: column; justify-content: center; align-items: center; }
  
  .library-container { opacity: 1; }
  .library-container.fade-out { opacity: 0; transition: opacity 0.25s ease-out; }
  /* #endregion */

  /* #region --- TOAST STYLES --- */
  .goal-toast {
    position: fixed; top: 30px; right: 30px; 
    z-index: 2147483647; /* Highest possible Z-Index */
    display: flex; align-items: center; gap: 12px;
    padding: 12px 18px; min-width: 280px;
    background: linear-gradient(135deg, rgba(255, 250, 245, 0.95), rgba(255, 235, 225, 0.90));
    border: 1px solid rgba(255, 255, 255, 0.6);
    box-shadow: 0 10px 40px rgba(94, 75, 75, 0.15), inset 0 0 0 1px rgba(255,255,255,0.4);
    border-radius: 20px; color: #5e4b4b; backdrop-filter: blur(12px); pointer-events: auto;
  }

  .toast-icon-circle {
    width: 32px; height: 32px;
    background: linear-gradient(135deg, #a7e8bd, #76c690);
    border-radius: 50%;
    display: flex; align-items: center; justify-content: center;
    color: #fff; box-shadow: 0 4px 12px rgba(134, 214, 160, 0.4); flex-shrink: 0;
  }
  .toast-icon-circle svg { width: 18px; height: 18px; }

  .toast-content { flex: 1; }
  .toast-title { font-weight: 800; font-size: 0.9rem; margin-bottom: 2px; }
  .toast-msg { font-size: 0.8rem; opacity: 0.7; font-weight: 500; }

  .toast-close {
    background: transparent; border: none; cursor: pointer; padding: 6px;
    border-radius: 50%; color: #5e4b4b; opacity: 0.4;
    display: flex; align-items: center; justify-content: center; transition: all 0.2s;
  }
  .toast-close:hover { opacity: 1; background: rgba(94, 75, 75, 0.08); }
  .toast-close svg { width: 16px; height: 16px; }

  .toast-glow {
    position: absolute; inset: 0; z-index: -1; pointer-events: none; border-radius: 20px;
    background: radial-gradient(circle at 10% 50%, rgba(167, 232, 189, 0.15), transparent 50%);
  }
  /* #endregion */
</style>
