<script lang="ts">
  import Sidebar from './Sidebar.svelte';
  import Library from './Library.svelte';
  import Background from './home-components/Background.svelte';
  import Orb from './home-components/Orb.svelte';
  import BookSearchModule from './home-components/BookSearchModule.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { fade, scale } from 'svelte/transition'; // Import transitions

  // UI state
  let bookTitle = "";
  let isPulsing = false; // This triggers the orb glow up
  let isFocused = false;
  let activeTab: "home" | "menu" = "home";
  let previousTab: "home" | "menu" = activeTab;

  // Return animation state
  type ReturnStage = "idle" | "fading" | "bouncing_down" | "bouncing_up";
  let returnStage: ReturnStage = "idle";
  let isReturning = false;

  // Search / result state
  type SearchState = "idle" | "loading" | "result";
  let searchState: SearchState = "idle";

  type MockBook = {
    title: string;
    author: string;
    year: string;
    pages: string;
    summary: string;
    fullSummary?: string;
    coverUrl?: string | null;
  };

  type LibraryBook = {
    id: string;
    title: string;
    author: string;
    cover: string;
    cover_color: string;
    status: string;
    pages_read: number;
    total_pages: number;
    date_added: string;
  };

  type BookStatus = "to-read" | "reading" | "finished";

  const MAX_RESULTS = 10;

  let foundBooks: MockBook[] = [];

  let isAdding = false;
  let isDiscarding = false;

  let summaryBook: MockBook | null = null;
  let orbElement: HTMLDivElement | null = null;

  // Add-details popup state
  let showAddDialog = false;
  let pendingBook: MockBook | null = null;
  let statusInput: BookStatus = "reading";
  let pagesReadInput = "0";
  let totalPagesInput = "0";

  // Tab change effects
  $: if (activeTab !== previousTab) {
    if (previousTab === "menu" && activeTab === "home") {
      triggerBounceSequence();
    }
    previousTab = activeTab;
  }

  // Reactive Logic for "Finished" status
  $: if (statusInput === "finished") {
    pagesReadInput = totalPagesInput;
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

  function shorten(text: string, maxChars = 260): string {
    if (!text) return "";
    if (text.length <= maxChars) return text;
    const cut = text.slice(0, maxChars);
    const lastSpace = cut.lastIndexOf(" ");
    const trimmed = lastSpace > 0 ? cut.slice(0, lastSpace) : cut;
    return trimmed + "…";
  }

  function firstSentence(text: string): string {
    if (!text) return "";
    const match = text.match(/[^.!?]+[.!?]/);
    return match ? match[0].trim() : text;
  }

  function cleanDescription(raw: string): string {
    if (!raw) return "";
    let text = raw;
    const cutMarkers = [
      "Also contained in:",
      "This work has also been published",
      "----------"
    ];
    for (const marker of cutMarkers) {
      const idx = text.indexOf(marker);
      if (idx !== -1) {
        text = text.slice(0, idx);
      }
    }
    text = text.replace(/\[(.*?)\]\(.*?\)/g, "$1");
    text = text.replace(/\(\[source]\[\d+]\)/gi, "");
    text = text.replace(/\[\d+]:\s*https?:\/\/\S+/gi, "");
    text = text.replace(/\s+/g, " ").trim();
    return text;
  }

  $: isGlowing =
    (isReturning && returnStage !== "idle") ||
    (isFocused && activeTab === "home") ||
    searchState === "loading" ||
    searchState === "result" ||
    isAdding ||
    isPulsing; // Add isPulsing here so orb glows when saving

  $: shouldScale = (isFocused && activeTab === "home" && !isReturning) || isPulsing;

  function handleInput(_event: CustomEvent<Event>) {
    if (activeTab !== "home") return;
    // Small pulse on typing
    // isPulsing = true; 
    // setTimeout(() => (isPulsing = false), 100); 
    // Commented out to prioritize save pulse, or keep if you like it
  }

  function handleFocus(_event: CustomEvent<FocusEvent>) {
    if (activeTab !== "home") return;
    isFocused = true;
    settleOrbToCenter();
  }

  function handleBlur(_event: CustomEvent<FocusEvent>) {
    isFocused = false;
    restoreOrbFloat();
  }

  async function handleKeydown(event: CustomEvent<KeyboardEvent>) {
    const e = event.detail;
    if (e.key !== "Enter" || !bookTitle.trim() || searchState !== "idle") return;

    const query = bookTitle.trim();
    bookTitle = "";
    searchState = "loading";

    try {
      const res = await fetch(
        `https://openlibrary.org/search.json?q=${encodeURIComponent(query)}&limit=${MAX_RESULTS}&fields=key,title,author_name,first_publish_year,number_of_pages_median,number_of_pages,cover_i,isbn`
      );

      if (!res.ok) {
        throw new Error(`Open Library error: ${res.status}`);
      }

      const data: any = await res.json();
      const docs: any[] = data.docs ?? [];

      if (!docs.length) {
        searchState = "idle";
        return;
      }

      const books: MockBook[] = [];

      for (const doc of docs) {
        let coverUrl: string | null = null;
        if (doc.cover_i) {
          coverUrl = `https://covers.openlibrary.org/b/id/${doc.cover_i}-M.jpg?default=false`;
        } else if (doc.isbn?.[0]) {
          coverUrl = `https://covers.openlibrary.org/b/isbn/${doc.isbn[0]}-M.jpg?default=false`;
        }

        let fullSummary = "";
        if (doc.key) {
          try {
            const workRes = await fetch(`https://openlibrary.org${doc.key}.json`);
            if (workRes.ok) {
              const workData: any = await workRes.json();
              const desc = workData.description;
              if (typeof desc === "string") {
                fullSummary = desc;
              } else if (desc && typeof desc === "object" && desc.value) {
                fullSummary = desc.value;
              }
            }
          } catch (err) {
            console.error("Failed to fetch work description", err);
          }
        }

        if (!fullSummary) {
          fullSummary = "No description available.";
        }

        const cleaned = cleanDescription(fullSummary);
        const teaser = firstSentence(cleaned);
        
        const pageCount = doc.number_of_pages_median || doc.number_of_pages || 0;

        books.push({
          title: doc.title ?? query,
          author: doc.author_name?.[0] ?? "Unknown author",
          year: doc.first_publish_year?.toString() ?? "—",
          pages: pageCount.toString(),
          summary: shorten(teaser, 120),
          fullSummary: cleaned,
          coverUrl
        });
      }

      foundBooks = books;
      searchState = "result";
    } catch (err) {
      console.error(err);
      searchState = "idle";
    }
  }

  function handleAddRequest(event: CustomEvent<MockBook>) {
    const book = event.detail;
    if (!book || isAdding || isDiscarding) return;

    pendingBook = book;
    statusInput = "reading";
    
    const p = parseInt(book.pages || "0", 10);
    totalPagesInput = isNaN(p) ? "0" : String(p);
    pagesReadInput = "0";
    
    showAddDialog = true;
  }

  function cancelAddDialog() {
    showAddDialog = false;
    pendingBook = null;
  }

  function incrementPages() {
    if (statusInput === "finished") return;
    const current = Number.parseInt(pagesReadInput || "0", 10);
    const total = Number.parseInt(totalPagesInput || "0", 10);
    const base = Number.isNaN(current) || current < 0 ? 0 : current;
    if (total > 0 && base >= total) {
       pagesReadInput = String(total);
       return; 
    }
    pagesReadInput = String(base + 1);
  }

  function decrementPages() {
    if (statusInput === "finished") return;
    const n = Number.parseInt(pagesReadInput || "0", 10);
    const base = Number.isNaN(n) || n <= 0 ? 0 : n;
    pagesReadInput = String(Math.max(0, base - 1));
  }

  function handlePageInput() {
      const current = parseInt(pagesReadInput);
      const total = parseInt(totalPagesInput);
      if (!isNaN(total) && total > 0 && current > total) {
          pagesReadInput = String(total);
      }
  }

  function confirmAddDialog() {
    if (!pendingBook || isAdding) return;

    const pages_read = Number.parseInt(pagesReadInput || "0", 10);
    const total_pages = Number.parseInt(totalPagesInput || "0", 10);
    const safePagesRead = Number.isNaN(pages_read) || pages_read < 0 ? 0 : pages_read;
    const safeTotalPages = Number.isNaN(total_pages) || total_pages < 0 ? 0 : total_pages;

    // 1. TRIGGER ORB GLOW ANIMATION
    isPulsing = true; // Grows the orb
    setTimeout(() => { isPulsing = false; }, 600); // Reset after animation

    // 2. FADE OUT DIALOG (By setting showAddDialog to false)
    showAddDialog = false;
    isAdding = false;
    
    const payload = {
        title: pendingBook.title,
        author: pendingBook.author,
        cover: pendingBook.coverUrl ?? "", 
        status: statusInput,
        pagesRead: safePagesRead,
        totalPages: safeTotalPages,
    };

    pendingBook = null;
    resetSearch();

    invoke('add_book', payload)
      .then((saved) => console.log("Book saved:", saved))
      .catch((err) => console.error("Save failed:", err));
  }

  async function handleDone() {
    if (isAdding || isDiscarding) return;
    isDiscarding = true;
    await wait(300);
    isDiscarding = false;
    resetSearch();
  }

  function resetSearch() {
    searchState = "idle";
    foundBooks = [];
    isAdding = false;
    isDiscarding = false;
    summaryBook = null;
  }

  function handleOpenSummary(event: CustomEvent<MockBook>) {
    const book = event.detail;
    if (!book) return;
    summaryBook = book;
  }

  function handleCloseSummary() {
    summaryBook = null;
  }

  function handleOverlayClick(event: MouseEvent) {
    if (event.target !== event.currentTarget) return;
    handleCloseSummary();
  }

  function handleOverlayKeydown(event: KeyboardEvent) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      handleCloseSummary();
    } else if (event.key === "Escape") {
      handleCloseSummary();
    }
  }

  function settleOrbToCenter() {
    if (!orbElement) return;
    const el = orbElement;
    const computed = getComputedStyle(el);
    const currentTransform = computed.transform === "none" ? "" : computed.transform;
    el.style.animation = "none";
    el.style.transform = currentTransform;
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
</script>

<main>
  <Background />
  <Sidebar bind:activeTab={activeTab} />

  <section class="orb-stage">
    <Orb
      bind:orbElement={orbElement}
      {activeTab}
      {isReturning}
      {returnStage}
      {isGlowing}
      {shouldScale}
      {isPulsing}
      {isAdding}
    >
      {#if activeTab === "home" && !isReturning}
        <BookSearchModule
          bind:bookTitle={bookTitle}
          {searchState}
          books={foundBooks}
          {isAdding}
          {isDiscarding}
          on:input={handleInput}
          on:keydown={handleKeydown}
          on:focus={handleFocus}
          on:blur={handleBlur}
          on:add={handleAddRequest}
          on:done={handleDone}
          on:openSummary={handleOpenSummary}
        />
      {:else if activeTab === "menu" || (isReturning && returnStage === "fading")}
        <div class="library-container" class:fade-out={returnStage === "fading"}>
          <Library />
        </div>
      {/if}
    </Orb>
  </section>

  {#if summaryBook}
    <div
      class="summary-overlay"
      role="button"
      tabindex="0"
      aria-label="Close description"
      on:click={handleOverlayClick}
      on:keydown={handleOverlayKeydown}
      transition:fade={{ duration: 200 }}
    >
      <div class="summary-dialog" transition:scale={{ duration: 300, start: 0.95 }}>
        <div class="summary-header">
          <h3>{summaryBook.title}</h3>
          <p class="summary-author">{summaryBook.author}</p>
        </div>
        <div class="summary-body">
          <p>{summaryBook.fullSummary ?? summaryBook.summary}</p>
        </div>
        <div class="summary-actions">
          <button class="pill-btn summary-close" on:click={handleCloseSummary}>
            Close
          </button>
        </div>
      </div>
    </div>
  {/if}

  {#if showAddDialog && pendingBook}
    <!-- FADE TRANSITION ADDED HERE -->
    <div class="add-overlay" transition:fade={{ duration: 250 }}>
      <div class="add-dialog">
        <div class="add-header">
          <h3>ADD TO LIBRARY</h3>
          <p>{pendingBook.title}</p>
          <span class="add-author">{pendingBook.author}</span>
        </div>

        <div class="add-body">
          <label class="field">
            <span>Status</span>
            <div class="status-row">
              <button
                type="button"
                class="status-pill"
                class:status-pill-active={statusInput === "to-read"}
                on:click={() => (statusInput = "to-read")}
              >
                To read
              </button>
              <button
                type="button"
                class="status-pill"
                class:status-pill-active={statusInput === "reading"}
                on:click={() => (statusInput = "reading")}
              >
                Reading
              </button>
              <button
                type="button"
                class="status-pill"
                class:status-pill-active={statusInput === "finished"}
                on:click={() => (statusInput = "finished")}
              >
                Finished
              </button>
            </div>
          </label>

          <label class="field field-center">
            <span>Pages read / {totalPagesInput}</span>
            <div class="pages-row" class:disabled-row={statusInput === "finished"}>
              <button
                type="button"
                class="pages-arrow pages-arrow-left"
                on:click={decrementPages}
                disabled={statusInput === "finished"}
                aria-label="Decrease pages read"
              >
                –
              </button>
              <input
                type="number"
                min="0"
                bind:value={pagesReadInput}
                on:input={handlePageInput}
                disabled={statusInput === "finished"}
                class="pages-input no-spin"
              />
              <button
                type="button"
                class="pages-arrow pages-arrow-right"
                on:click={incrementPages}
                disabled={statusInput === "finished"}
                aria-label="Increase pages read"
              >
                +
              </button>
            </div>
          </label>
        </div>

        <div class="add-actions">
          <button
            type="button"
            class="pill-btn pill-secondary"
            on:click={cancelAddDialog}
          >
            Cancel
          </button>
          <button
            type="button"
            class="pill-btn pill-primary"
            on:click={confirmAddDialog}
          >
            Save
          </button>
        </div>
      </div>
    </div>
  {/if}
</main>

<style>
  /* ... (Global/Layout Styles) ... */
  main {
    display: flex;
    height: 100vh;
    width: 100vw;
    position: relative;
  }

  .orb-stage {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
    z-index: 5;
  }

  .library-container {
    width: 100%;
    height: 100%;
    opacity: 1;
    transition: opacity 0.25s ease-out;
  }

  .library-container.fade-out {
    opacity: 0;
  }

  /* Summary modal */
  .summary-overlay {
    position: fixed;
    inset: 0;
    z-index: 900;
    background: rgba(34, 22, 19, 0.35);
    backdrop-filter: blur(14px);
    -webkit-backdrop-filter: blur(14px);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .summary-dialog {
    max-width: 620px;
    max-height: 70vh;
    width: 90%;
    background: rgba(255, 255, 255, 0.78);
    border-radius: 28px;
    padding: 24px 26px 20px;
    box-shadow:
      0 18px 50px rgba(0, 0, 0, 0.18),
      inset 0 0 18px rgba(255, 255, 255, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.9);
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .summary-header h3 {
    margin: 0;
    font-size: 1.2rem;
    font-weight: 600;
    color: #4b332e;
  }

  .summary-author {
    margin: 4px 0 0;
    font-size: 0.9rem;
    color: rgba(75, 51, 46, 0.7);
  }

  .summary-body {
    flex: 1;
    overflow-y: auto;
    padding-right: 4px;
  }

  .summary-body p {
    margin: 0;
    font-size: 0.95rem;
    line-height: 1.5;
    color: rgba(75, 51, 46, 0.9);
  }

  .summary-actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 8px;
  }

  .summary-close {
    min-width: 90px;
  }

  /* Add-details popup */
  .add-overlay {
    position: fixed;
    inset: 0;
    z-index: 950;
    display: flex;
    align-items: center;
    justify-content: center;
    background:
      radial-gradient(circle at top left, rgba(255, 190, 150, 0.4), transparent 55%),
      radial-gradient(circle at bottom right, rgba(255, 140, 180, 0.35), transparent 60%),
      rgba(255, 220, 200, 0.35);
    backdrop-filter: blur(22px) saturate(1.25);
    -webkit-backdrop-filter: blur(22px) saturate(1.25);
  }

  .add-dialog {
    width: 420px;
    max-width: 90vw;
    background: rgba(255, 255, 255, 0.68);
    border-radius: 26px;
    padding: 20px 22px 18px;
    box-shadow:
      0 20px 40px rgba(180, 110, 80, 0.35),
      inset 0 0 18px rgba(255, 255, 255, 0.9);
    border: 1px solid rgba(255, 245, 240, 0.95);
    display: flex;
    flex-direction: column;
    gap: 16px;
    transform-origin: center;
    /* Used standard svelte fade, removed custom animation here to avoid conflicts */
  }

  .add-header h3 {
    margin: 0;
    font-size: 0.95rem;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: rgba(131, 91, 74, 0.9);
    text-align: center;
  }

  .add-header p {
    margin: 4px 0 0;
    font-size: 1.05rem;
    font-weight: 600;
    color: #4b332e;
    text-align: center;
  }

  .add-author {
    font-size: 0.85rem;
    color: rgba(75, 51, 46, 0.7);
    text-align: center;
  }

  .add-body {
    display: flex;
    flex-direction: column;
    gap: 14px;
    margin-top: 10px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-size: 0.85rem;
    color: rgba(75, 51, 46, 0.8);
  }

  .field-center {
    align-items: center;
    text-align: center;
  }

  .field-center > span {
    width: 100%;
    text-align: center;
  }

  .status-row {
    display: flex;
    gap: 8px;
  }

  .status-pill {
    flex: 1;
    padding: 7px 10px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.5);
    color: #5b3b30;
    font-size: 0.8rem;
    cursor: pointer;
    transition:
      background 0.18s ease,
      box-shadow 0.18s ease,
      transform 0.12s ease;
  }

  .status-pill-active {
    background: linear-gradient(135deg, #ffcf9f, #f8a3b0);
    box-shadow: 0 8px 18px rgba(200, 120, 90, 0.3);
    transform: translateY(-1px);
  }

  /* Pages input + custom arrows */
  .pages-row {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    transition: opacity 0.2s ease, filter 0.2s ease;
  }
  
  /* Disabled state styling */
  .pages-row.disabled-row {
    opacity: 0.5;
    filter: grayscale(0.5);
    pointer-events: none;
  }

  .pages-input {
    width: 90px;
    padding: 7px 10px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.55);
    box-shadow: inset 0 0 10px rgba(255, 255, 255, 0.9);
    color: #4b332e;
    font-size: 0.9rem;
    text-align: center;
    outline: none;
  }

  .no-spin {
    -moz-appearance: textfield;
    appearance: textfield;
  }
  .no-spin::-webkit-outer-spin-button,
  .no-spin::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .pages-arrow {
    width: 28px;
    height: 28px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.42);
    color: #5b3b30;
    font-size: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow:
      0 6px 14px rgba(200, 120, 90, 0.25),
      inset 0 0 8px rgba(255, 255, 255, 0.7);
    transition:
      background 0.15s ease,
      transform 0.12s ease,
      box-shadow 0.15s ease;
  }

  .pages-arrow:hover {
    background: rgba(255, 255, 255, 0.65);
    transform: translateY(-1px);
  }
  
  .pages-arrow:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .pages-arrow-left,
  .pages-arrow-right {
    padding-bottom: 1px;
  }

  .add-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 6px;
  }

  /* -- UPDATED BUTTON STYLES -- */
  .pill-btn {
    min-width: 90px;
    padding: 8px 20px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.7);
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    transition:
      background 0.18s ease,
      transform 0.12s ease,
      box-shadow 0.18s ease,
      color 0.2s ease;
  }

  .pill-secondary {
    background: rgba(255, 255, 255, 0.4);
    color: #5b3b30;
  }
  
  /* Save button: Ghost by default, Colorful only on Active/Click */
  .pill-primary {
    background: rgba(255, 255, 255, 0.5); /* Ghost-ish default */
    color: #4b332e;
    box-shadow: 0 4px 12px rgba(200, 120, 90, 0.15);
  }

  .pill-primary:hover {
    background: rgba(255, 255, 255, 0.75); /* Slightly more opaque on hover */
    transform: translateY(-1px);
    box-shadow: 0 8px 20px rgba(200, 120, 90, 0.2);
  }

  /* The "Burst" color effect only happens when clicking (active) */
  .pill-primary:active {
    background: linear-gradient(135deg, #ffcf9f, #f8a3b0);
    color: #2c1810;
    transform: translateY(1px);
    box-shadow: 0 2px 8px rgba(200, 120, 90, 0.3);
  }

  .pill-secondary:hover {
    background: rgba(255, 255, 255, 0.6);
    transform: translateY(-1px);
  }
</style>
