<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { fade, scale, fly } from 'svelte/transition';
  import { cubicIn, cubicOut, elasticOut } from 'svelte/easing';
  import { tick, onMount } from 'svelte';

  // Components
  import Sidebar from './Sidebar.svelte';
  import Library from './Library.svelte';
  import Settings from './Settings.svelte';
  import Background from './home-components/Background.svelte';
  import Orb from './home-components/Orb.svelte';
  import BookSearchModule from './home-components/BookSearchModule.svelte';
  import AddBookDialog from './home-components/AddBookModal.svelte';
  import SummaryModal from './home-components/SummaryModal.svelte';
  import Toast from './home-components/Toast.svelte';

  // --- Tabs ---
  let requestedTab: "home" | "menu" | "settings" = "home";
  let activeTab: "home" | "menu" | "settings" = "home";
  let previousTab: "home" | "menu" | "settings" = activeTab;

  // --- Settings animation stages ---
  let settingsStage: "idle" | "collapsing" | "glowing" | "dividing" | "gathering" = "idle";
  let isOpeningSettings = false;

  // --- Return from settings -> home ---
  let returningFromSettings = false;

  // Overlay expansion (the “big expand” after gather)
  let showHomeExpandOrb = false;

  // Keep overlay mounted while we fade its glow out
  let overlayHoldVisible = false;
  let overlayGlowOff = false;

  // Prevent second “home orb intro / bounce” after overlay expansion
  let skipHomeIntroOnce = false;
  let suppressHomeBounceOnce = false;

  // --- Overlay timing (MUST match CSS durations below) ---
  const overlayExpandMs = 650;
  // Total fade duration (match .expand-overlay transition)
  const overlayFadeMs = 900;
  // Start fading this much BEFORE expand ends (so it begins disappearing sooner)
  const overlayFadeLeadMs = 250;

  // Return animation (your existing home bounce)
  let returnStage: "idle" | "fading" | "bouncing_down" | "bouncing_up" = "idle";
  let isReturning = false;

  // Search & Data State
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

  // Modal States
  let showAddDialog = false;
  let showDuplicateToast = false;
  let pendingBook: any | null = null;
  let summaryBook: any | null = null;
  let orbElement: HTMLDivElement | null = null;

  // --- GOAL TOAST STATE ---
  let showGoalToast = false;
  let readingGoal = 10;
  let booksFinishedCount = 0;
  let toastTimer: number | null = null;

  // Keep glow while collapsing + while overlay is visible
  $: isGlowing =
    (isReturning && returnStage !== "idle") ||
    isFocused ||
    searchState !== "idle" ||
    showAddDialog ||
    isPulsing ||
    settingsStage === "collapsing" ||
    showHomeExpandOrb ||
    overlayHoldVisible;

  function wait(ms: number) {
    return new Promise<void>((r) => setTimeout(r, ms));
  }

  // --- CUSTOM TRANSITION: Moves Orb Left & Shrinks when opening settings ---
  function orbSettingsOut(_: Element, { duration, easing }: any) {
    // Check if we are specifically transitioning TO settings
    if (isOpeningSettings) {
        return {
            duration,
            easing,
            css: (t: number) => {
                // t goes from 1 (start) to 0 (end) during 'out'
                const inverted = 1 - t;
                
                // Scale from 1 down to 0.15
                const s = 0.15 + (0.85 * t);
                
                // Move Left: 0px to -45px (updated as requested)
                const x = -38 * inverted; 

                return `transform: translateX(${x}px) scale(${s}); opacity: ${t};`;
            }
        };
    } else {
        // Fallback for other exits (just standard scale)
        return {
            duration,
            easing,
            css: (t: number) => `transform: scale(${0.12 + 0.88 * t}); opacity: ${t};`
        };
    }
  }

  // --- GOAL CHECKER LOGIC ---
  async function checkGoalCompletion() {
    try {
        const savedGoal = localStorage.getItem('hikari_reading_goal');
        if (savedGoal) readingGoal = parseInt(savedGoal, 10);

        const books: any[] = await invoke('get_books');
        booksFinishedCount = books.filter(b => b.status === 'finished').length;

        const alreadyNotified = localStorage.getItem('hikari_goal_notified') === 'true';

        if (booksFinishedCount >= readingGoal && booksFinishedCount > 0) {
            if (!alreadyNotified) {
                showGoalToast = true;
                localStorage.setItem('hikari_goal_notified', 'true');
                if (toastTimer) clearTimeout(toastTimer);
                toastTimer = window.setTimeout(() => {
                    showGoalToast = false;
                }, 6000);
            }
        }
    } catch (err) {
        console.error("Goal check failed", err);
    }
  }

  function closeGoalToast() {
    showGoalToast = false;
    if (toastTimer) clearTimeout(toastTimer);
  }

  onMount(() => {
    checkGoalCompletion();
  });

  // Intercept requested tab changes to allow animations
  $: if (requestedTab !== activeTab) {
    // Check goal when switching back to home or library
    if (requestedTab === "home" || requestedTab === "menu") {
         checkGoalCompletion();
    }

    // SETTINGS -> HOME (play gathering first)
    if (activeTab === "settings" && requestedTab === "home" && !returningFromSettings) {
      returningFromSettings = true;
      settingsStage = "gathering";
    }
    // Normal switch (no special animation)
    else if (!returningFromSettings) {
      activeTab = requestedTab;
    }
  }

  // Track previous tab for other logic
  $: if (activeTab !== previousTab) {
    // Returning to home bounce (disable if we just came from settings expansion)
    if (activeTab === "home") {
      if (!suppressHomeBounceOnce) {
        triggerBounceSequence();
      }
    }

    // Going TO settings: start collapsing immediately
    if (activeTab === "settings" && previousTab !== "settings") {
      restoreOrbFloat();
      isOpeningSettings = true;
      settingsStage = "collapsing";
    }

    // Leaving settings (after commit)
    if (previousTab === "settings" && activeTab !== "settings") {
      isOpeningSettings = false;
      settingsStage = "idle";
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

  // Called when big orb OUT transition completes (home->settings chain)
  async function handleOrbOutroEnd() {
    if (!isOpeningSettings) return;

    settingsStage = "glowing";
    await wait(240);

    if (activeTab === "settings") settingsStage = "dividing";
    isOpeningSettings = false;
  }

  // SETTINGS -> HOME: called by Settings when all bubbles have merged into seed
  async function handleSettingsReadyToExpand() {
    if (!returningFromSettings) return;

    // 1) Expand overlay orb ONCE (keep it mounted afterward for glow fade)
    overlayHoldVisible = true;
    overlayGlowOff = false;
    showHomeExpandOrb = true;

    // Start fading BEFORE the expand ends (so glow starts disappearing sooner)
    const fadeStartAt = Math.max(0, overlayExpandMs - overlayFadeLeadMs);
    const fadeTimer = window.setTimeout(() => {
      overlayGlowOff = true;
    }, fadeStartAt);

    await wait(overlayExpandMs); // must match overlay in:scale duration

    // Expand finished; stop showing "expand overlay" transition wrapper
    showHomeExpandOrb = false;

    // 2) Commit to Home but skip Home's own orb intro/bounce for the first render
    suppressHomeBounceOnce = true;
    skipHomeIntroOnce = true;

    returningFromSettings = false;
    settingsStage = "idle";
    activeTab = "home";
    requestedTab = "home";

    // Wait until the DOM applied the Home mount before clearing flags
    await tick();

    suppressHomeBounceOnce = false;
    skipHomeIntroOnce = false;

    // Ensure fade definitely started (in case timings were changed)
    overlayGlowOff = true;
    clearTimeout(fadeTimer);

    // Only keep overlay mounted for the REMAINING fade time (no lingering)
    const elapsedFadeByNow = overlayExpandMs - fadeStartAt;
    const remainingFade = Math.max(0, overlayFadeMs - elapsedFadeByNow);
    await wait(remainingFade);

    overlayHoldVisible = false;
    overlayGlowOff = false;
  }

  // --- ORB HANDLERS ---
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

  function settleOrbToCenter() {
    if (!orbElement) return;
    const el = orbElement;

    const computedStyle = window.getComputedStyle(el);
    const currentTransform = computedStyle.transform === 'none' ? '' : computedStyle.transform;

    el.style.animation = "none";
    el.style.transform = currentTransform;
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

  function handleInput() {
    if (activeTab === "home") {
      clearTimeout(typingPulseTimeout);
      isPulsing = true;
      typingPulseTimeout = window.setTimeout(() => (isPulsing = false), 100);
    }
  }

  // --- API FUNCTIONS ---
  
  // Helper to fetch full description from OpenLibrary when a book is selected
  async function fetchOpenLibraryDescription(key: string): Promise<string> {
      try {
          const res = await fetch(`https://openlibrary.org${key}.json`);
          if (!res.ok) return "No description available.";
          const data = await res.json();
          // OL descriptions can be strings or objects: { type: 'text', value: '...' }
          if (typeof data.description === 'string') return data.description;
          if (data.description?.value) return data.description.value;
          return "No description available.";
      } catch {
          return "Failed to load description.";
      }
  }

  async function searchOpenLibrary(query: string): Promise<any[]> {
    try {
      // Added 'first_sentence' to fields for better immediate context
      const res = await fetch(
        `https://openlibrary.org/search.json?q=${encodeURIComponent(query)}&limit=10&fields=key,title,author_name,first_publish_year,number_of_pages_median,number_of_pages,cover_i,isbn,first_sentence`
      );
      if (!res.ok) return [];
      const data: any = await res.json();
      const docs: any[] = data.docs ?? [];
      if (!docs.length) return [];

      const books = docs.map(doc => {
        let coverUrl: string | null = null;
        if (doc.cover_i) coverUrl = `https://covers.openlibrary.org/b/id/${doc.cover_i}-M.jpg`;
        else if (doc.isbn?.[0]) coverUrl = `https://covers.openlibrary.org/b/isbn/${doc.isbn[0]}-M.jpg`;

        // Use first_sentence as a preview if available
        const preview = doc.first_sentence?.[0] || "Click for details...";

        return {
          title: doc.title ?? query,
          author: doc.author_name?.[0] ?? "Unknown author",
          year: doc.first_publish_year?.toString() ?? "—",
          pages: (doc.number_of_pages_median || doc.number_of_pages || 0).toString(),
          summary: preview,
          fullSummary: null, // Will be fetched on demand
          coverUrl,
          key: doc.key
        };
      });

      // Removed the aggressive image pre-loading loop here
      // This prevents network congestion and makes results appear faster

      return books;
    } catch (err) {
      console.error("Open Library search failed:", err);
      return [];
    }
  }

  async function searchAnilist(query: string): Promise<any[]> {
    try {
      const anilistQuery = `
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

      const anilistRes = await fetch('https://graphql.anilist.co', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'Accept': 'application/json' },
        body: JSON.stringify({ query: anilistQuery, variables: { search: query } })
      });

      if (!anilistRes.ok) return [];
      const anilistData = await anilistRes.json();
      const media = anilistData.data?.Page?.media ?? [];

      return media.map((manga: any) => {
        // Strip HTML tags from AniList description
        let cleanDesc = manga.description || "No description available.";
        cleanDesc = cleanDesc.replace(/<br\s*\/?>/gi, '\n').replace(/<[^>]+>/g, '');
        
        return {
            title: manga.title.english || manga.title.romaji || query,
            author: manga.staff.edges?.[0]?.node?.name?.full ?? "Unknown author",
            year: manga.startDate?.year?.toString() ?? "—",
            pages: (manga.chapters || 0).toString(),
            summary: cleanDesc.length > 200 ? cleanDesc.substring(0, 200) + "..." : cleanDesc,
            fullSummary: cleanDesc,
            coverUrl: manga.coverImage?.large,
            key: null
        };
      });
    } catch (err) {
      console.error("AniList search failed:", err);
      return [];
    }
  }

  async function handleApiSwitch(api: "openlibrary" | "anilist") {
    if (selectedApi === api || !lastQuery) return;
    selectedApi = api;

    const results = apiResults[api];
    if (results) {
      foundBooks = results;
      return;
    }

    isApiSwitching = true;
    try {
      let searchResults: any[] = [];
      if (api === "openlibrary") searchResults = await searchOpenLibrary(lastQuery);
      else searchResults = await searchAnilist(lastQuery);

      apiResults[api] = searchResults;
      foundBooks = searchResults;
    } catch {
      foundBooks = [];
    } finally {
      isApiSwitching = false;
    }
  }

  // --- DATA & MODALS ---
  async function handleKeydown(event: CustomEvent<KeyboardEvent>) {
    const e = event.detail;
    if (e.key !== "Enter" || !bookTitle.trim() || searchState !== "idle") return;

    const query = bookTitle.trim();
    bookTitle = "";
    lastQuery = query;
    searchState = "loading";
    apiResults = { openlibrary: null, anilist: null };

    try {
      const [openLibraryResults, anilistResults] = await Promise.all([
        searchOpenLibrary(query),
        searchAnilist(query)
      ]);

      apiResults.openlibrary = openLibraryResults;
      apiResults.anilist = anilistResults;

      if (selectedApi === "openlibrary" && openLibraryResults.length > 0) foundBooks = openLibraryResults;
      else if (selectedApi === "anilist" && anilistResults.length > 0) foundBooks = anilistResults;
      else if (openLibraryResults.length > 0) { selectedApi = "openlibrary"; foundBooks = openLibraryResults; }
      else if (anilistResults.length > 0) { selectedApi = "anilist"; foundBooks = anilistResults; }
      else foundBooks = [];

      searchState = "result";
    } catch (err) {
      console.error(err);
      searchState = "idle";
    }
  }

  function handleAddRequest(event: CustomEvent) {
    pendingBook = event.detail;
    showAddDialog = true;
  }

  // Handler for opening the Summary Modal
  // This now checks if we need to fetch a full description
  async function handleOpenSummary(event: CustomEvent) {
      let book = event.detail;
      
      // If it's an OpenLibrary book (has a key) and we don't have the full summary yet
      if (book.key && !book.fullSummary) {
          // Show modal immediately with what we have
          summaryBook = { ...book, fullSummary: "Loading full description..." };
          
          // Fetch description in background
          const fullDesc = await fetchOpenLibraryDescription(book.key);
          
          // Update the modal
          summaryBook = { ...book, fullSummary: fullDesc, summary: fullDesc };
          
          // Also update the book in the search results list so we don't fetch again
          foundBooks = foundBooks.map(b => b.key === book.key ? { ...b, fullSummary: fullDesc, summary: fullDesc } : b);
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
      // Re-check goal after adding a book
      checkGoalCompletion();
    } catch (err) {
      showDuplicateToast = true;
      setTimeout(() => (showDuplicateToast = false), 3000);
    }
  }
</script>

<main>
  <Background />
  <Sidebar bind:activeTab={requestedTab} />

  <!-- GLOBAL GOAL TOAST (Right Positioned) -->
  {#if showGoalToast}
    <div class="goal-toast" transition:fly={{ y: -50, x: 20, duration: 800, easing: elasticOut }}>
        <div class="toast-icon-circle">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"/></svg>
        </div>
        <div class="toast-content">
            <div class="toast-title">Goal Reached!</div>
            <div class="toast-msg">You've finished {readingGoal} books.</div>
        </div>
        <button class="toast-close" on:click={closeGoalToast} aria-label="Close notification">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18M6 6l12 12"/></svg>
        </button>
        <div class="toast-glow"></div>
    </div>
  {/if}

  <section class="orb-stage">
    <!-- SETTINGS stays mounted while gathering/expanding back -->
    {#if activeTab === "settings"}
      <div class="settings-layer">
        <Settings stage={settingsStage} on:readyToExpand={handleSettingsReadyToExpand} />
      </div>
    {/if}

    <!-- Normal orb when not in settings -->
    {#if activeTab !== "settings"}
      <div
        class="orb-wrapper"
        in:scale={skipHomeIntroOnce
          ? { duration: 0 }
          : { duration: 600, easing: cubicOut, start: 0.2, delay: 200 }
        }
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
            <!-- keep your requested 700ms delay -->
            <div class="library-container" class:fade-out={returnStage === "fading"} in:fade={{ duration: 400, delay: 700 }}>
              <Library />
            </div>
          {/if}
        </Orb>
      </div>
    {/if}

    <!-- EXPAND overlay orb while still “in settings”, and keep it during glow fade-out -->
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
    <SummaryModal book={summaryBook} on:close={() => (summaryBook = null)} />
  {/if}

  {#if showAddDialog && pendingBook}
    <AddBookDialog book={pendingBook} on:cancel={() => (showAddDialog = false)} on:save={saveBook} />
  {/if}

  {#if showDuplicateToast}
    <Toast title="Book Exists" message="This book is already in your library." />
  {/if}
</main>

<style>
  main { display: flex; height: 100vh; width: 100vw; position: relative; }
  .orb-stage { flex: 1; display: flex; justify-content: center; align-items: center; position: relative; z-index: 5; }

  .orb-wrapper, .settings-layer {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    position: absolute;
  }

  .orb-wrapper { transform-origin: 50% 50%; }

  .settings-layer { z-index: 1; }
  .orb-wrapper { z-index: 2; }
  .orb-wrapper.expand-overlay { z-index: 3; pointer-events: none; }

  /* Fade must match overlayFadeMs (900ms) and should be smooth (no linger). */
  .expand-overlay {
    opacity: 1;
    transition: opacity 900ms cubic-bezier(0.33, 0, 0.67, 1);
  }
  .expand-overlay.fade-overlay {
    opacity: 0;
  }

  .search-container, .library-container { width: 100%; height: 100%; }
  .search-container { display: flex; flex-direction: column; justify-content: center; align-items: center; }

  .library-container { opacity: 1; }
  .library-container.fade-out { opacity: 0; transition: opacity 0.25s ease-out; }

  /* --- GLOBAL TOAST --- */
  .goal-toast {
    position: fixed;
    top: 30px; 
    right: 30px; 
    z-index: 20000;
    
    display: flex; align-items: center; gap: 12px;
    padding: 12px 18px;
    min-width: 280px;
    
    background: linear-gradient(135deg, rgba(255, 250, 245, 0.95), rgba(255, 235, 225, 0.90));
    border: 1px solid rgba(255, 255, 255, 0.6);
    box-shadow: 
        0 10px 40px rgba(94, 75, 75, 0.15), 
        0 4px 12px rgba(0,0,0,0.05),
        inset 0 0 0 1px rgba(255,255,255,0.4);
    
    border-radius: 20px;
    color: #5e4b4b;
    backdrop-filter: blur(12px);
    pointer-events: auto;
  }

  .toast-icon-circle {
    width: 32px; height: 32px;
    background: linear-gradient(135deg, #a7e8bd, #76c690);
    border-radius: 50%;
    display: flex; align-items: center; justify-content: center;
    color: #fff;
    box-shadow: 0 4px 12px rgba(134, 214, 160, 0.4);
    flex-shrink: 0;
  }
  .toast-icon-circle svg { width: 18px; height: 18px; }

  .toast-content { flex: 1; }
  .toast-title { font-weight: 800; font-size: 0.9rem; letter-spacing: 0.01em; margin-bottom: 2px; }
  .toast-msg { font-size: 0.8rem; opacity: 0.7; font-weight: 500; }

  .toast-close {
    background: transparent; border: none; cursor: pointer;
    padding: 6px; border-radius: 50%;
    color: #5e4b4b; opacity: 0.4;
    display: flex; align-items: center; justify-content: center;
    transition: all 0.2s;
  }
  .toast-close:hover { opacity: 1; background: rgba(94, 75, 75, 0.08); transform: rotate(90deg); }
  .toast-close svg { width: 16px; height: 16px; }

  .toast-glow {
    position: absolute; inset: 0; z-index: -1; pointer-events: none;
    border-radius: 20px;
    background: radial-gradient(circle at 10% 50%, rgba(167, 232, 189, 0.15), transparent 50%);
  }
</style>
