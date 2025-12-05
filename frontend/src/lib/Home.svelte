<script lang="ts">
  import Sidebar from './Sidebar.svelte';
  import Library from './Library.svelte';
  import Background from './home-components/Background.svelte';
  import Orb from './home-components/Orb.svelte';
  import BookSearchModule from './home-components/BookSearchModule.svelte';

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

  // Search / result state
  type SearchState = "idle" | "loading" | "result";
  let searchState: SearchState = "idle";

  type MockBook = {
    title: string;
    author: string;
    year: string;
    pages: string;
    summary: string;       // truncated
    fullSummary?: string;  // full text for modal
    coverUrl?: string | null;
  };

  // Top N search results
  let foundBooks: MockBook[] = [];

  // "Add" and "Done" confirmation animation states (global for now)
  let isAdding = false;
  let isDiscarding = false;

  // Which book’s description is open in the modal (null = closed)
  let summaryBook: MockBook | null = null;

  // Reference to the orb element for smooth float → center transition
  let orbElement: HTMLDivElement | null = null;

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

  function shorten(text: string, maxChars = 260): string {
    if (!text) return "";
    if (text.length <= maxChars) return text;
    const cut = text.slice(0, maxChars);
    const lastSpace = cut.lastIndexOf(" ");
    const trimmed = lastSpace > 0 ? cut.slice(0, lastSpace) : cut;
    return trimmed + "…";
  }

  function cleanDescription(raw: string): string {
    if (!raw) return "";

    let text = raw;

    // 1) Strip everything after common separators / meta sections
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

    // 2) Remove inline markdown links: [text](url) -> text
    text = text.replace(/\[(.*?)\]\(.*?\)/g, "$1");

    // 3) Remove reference-style uses like "([source][1])"
    text = text.replace(/\(\[source]\[\d+]\)/gi, "");

    // 4) Remove reference definitions like "[1]: https?:..."
    text = text.replace(/\[\d+]:\s*https?:\/\/\S+/gi, "");

    // 5) Collapse excessive whitespace
    text = text.replace(/\s+/g, " ").trim();

    return text;
  }

  // Derived visual states for Orb
  $: isGlowing =
    (isReturning && returnStage !== "idle") ||
    (isFocused && activeTab === "home") ||
    searchState === "loading" ||
    searchState === "result" ||
    isAdding;

  $: shouldScale = isFocused && activeTab === "home" && !isReturning;

  // Input interactions (BookSearchModule dispatches CustomEvents)
  function handleInput(_event: CustomEvent<Event>) {
    if (activeTab !== "home") return;
    isPulsing = true;
    setTimeout(() => (isPulsing = false), 100);
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

  function firstSentence(text: string): string {
  if (!text) return "";
  const match = text.match(/[^.!?]+[.!?]/); // up to first ., ! or ?
  return match ? match[0].trim() : text;
}

  // Hit Enter to search Open Library (with cover + descriptions) and keep top 3
  async function handleKeydown(event: CustomEvent<KeyboardEvent>) {
    const e = event.detail;

    if (e.key !== "Enter" || !bookTitle.trim() || searchState !== "idle") return;

    const query = bookTitle.trim();
    bookTitle = "";
    searchState = "loading";

    try {
      const res = await fetch(
        `https://openlibrary.org/search.json?q=${encodeURIComponent(query)}`
      );
      if (!res.ok) {
        throw new Error(`Open Library error: ${res.status}`);
      }

      const data: any = await res.json();
      const docs = (data.docs ?? []).slice(0, 3); // top 3 hits

      if (!docs.length) {
        searchState = "idle";
        return;
      }

      const books: MockBook[] = [];

      for (const doc of docs) {
        // Build cover URL from cover_i or first ISBN
        let coverUrl: string | null = null;
        if (doc.cover_i) {
          coverUrl = `https://covers.openlibrary.org/b/id/${doc.cover_i}-M.jpg?default=false`;
        } else if (doc.isbn?.[0]) {
          coverUrl = `https://covers.openlibrary.org/b/isbn/${doc.isbn[0]}-M.jpg?default=false`;
        }

        // Fetch a description from the Work endpoint if available
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

        books.push({
          title: doc.title ?? query,
          author: doc.author_name?.[0] ?? "Unknown author",
          year: doc.first_publish_year?.toString() ?? "—",
          pages: doc.number_of_pages_median?.toString() ?? "—",
          summary: shorten(teaser, 40),
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

  async function handleAdd(event: CustomEvent<MockBook>) {
    const book = event.detail;
    if (!book || isAdding || isDiscarding) return;

    console.log("Add book:", book);
    isAdding = true;
    await wait(600);
    isAdding = false;
    resetSearch();
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
    // Only close when the user clicks on the backdrop, not inside the dialog
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
          on:add={handleAdd}
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
    >
      <div class="summary-dialog">
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
</main>

<style>
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

  /* Full summary modal overlay */
  .summary-overlay {
    position: fixed;
    inset: 0;
    z-index: 999;
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
    background: rgba(255, 255, 255, 0.75);
    border-radius: 28px;
    padding: 24px 26px 20px;
    box-shadow:
      0 18px 50px rgba(0, 0, 0, 0.18),
      inset 0 0 18px rgba(255, 255, 255, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.9);
    display: flex;
    flex-direction: column;
    gap: 16px;
    animation: summaryIn 0.2s cubic-bezier(0.25, 0.9, 0.3, 1) forwards;
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

  @keyframes summaryIn {
    from {
      opacity: 0;
      transform: translateY(10px) scale(0.97);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
</style>
