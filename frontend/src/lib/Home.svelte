<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { fade } from 'svelte/transition';

  // Components
  import Sidebar from './Sidebar.svelte';
  import Library from './Library.svelte';
  import Settings from './Settings.svelte';
  import Background from './home-components/Background.svelte';
  import Orb from './home-components/Orb.svelte';
  import BookSearchModule from './home-components/BookSearchModule.svelte';
  import AddBookModal from './home-components/AddBookModal.svelte';
  import SummaryModal from './home-components/SummaryModal.svelte';
  import Toast from './home-components/Toast.svelte';

  // State
  // FIX: Type includes "settings" now
  let activeTab: "home" | "menu" | "settings" = "home";
  //let previousTab: "home" | "menu" | "settings" = activeTab;

  let returnStage: "idle" | "fading" | "bouncing_down" | "bouncing_up" = "idle";
  let isReturning = false;

  // Search State
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

  // Computed properties
  let isGlowing = false;
  $: isGlowing = (isReturning && returnStage !== "idle") || isFocused || searchState !== "idle" || showAddDialog || isPulsing;
  $: showLibrary = activeTab === "menu" || isReturning;

  async function searchOpenLibrary(query: string): Promise<any[]> {
    try {
      const res = await fetch(
        `https://openlibrary.org/search.json?q=${encodeURIComponent(query)}&limit=10&fields=key,title,author_name,first_publish_year,number_of_pages_median,number_of_pages,cover_i,isbn`
      );

      if (!res.ok) return [];

      const data: any = await res.json();
      const docs: any[] = data.docs ?? [];

      if (!docs.length) return [];

      const books = docs.map(doc => {
        let coverUrl: string | null = null;
        if (doc.cover_i) coverUrl = `https://covers.openlibrary.org/b/id/${doc.cover_i}-M.jpg`;
        else if (doc.isbn?.[0]) coverUrl = `https://covers.openlibrary.org/b/isbn/${doc.isbn[0]}-M.jpg`;

        return {
          title: doc.title ?? query,
          author: doc.author_name?.[0] ?? "Unknown author",
          year: doc.first_publish_year?.toString() ?? "—",
          pages: (doc.number_of_pages_median || doc.number_of_pages || 0).toString(),
          summary: "Loading description...",
          fullSummary: null,
          coverUrl,
          key: doc.key
        };
      });

      // Preload cover images
      books.forEach((book: any) => {
        if (book.coverUrl) {
          const img = new Image();
          img.src = book.coverUrl;
        }
      });

      // Fetch descriptions
      await Promise.all(books.map(async (book: any) => {
        if (book.key) {
          try {
            const descRes = await fetch(`https://openlibrary.org${book.key}.json`);
            if (descRes.ok) {
              const descData = await descRes.json();
              const description = descData.description;
              if (typeof description === 'string') {
                book.summary = description.length > 200 ? description.substring(0, 200) + "..." : description;
                book.fullSummary = description;
              } else if (description?.value) {
                const desc = description.value;
                book.summary = desc.length > 200 ? desc.substring(0, 200) + "..." : desc;
                book.fullSummary = desc;
              } else {
                book.summary = "No description available.";
              }
            } else {
              book.summary = "No description available.";
            }
          } catch (err) {
            console.error("Error fetching description for", book.title, err);
            book.summary = "No description available.";
          }
        } else {
          book.summary = "No description available.";
        }
      }));

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
              title {
                romaji
                english
              }
              coverImage {
                large
              }
              description
              chapters
              volumes
              startDate {
                year
              }
              staff(perPage: 1, sort: RELEVANCE) {
                edges {
                  node {
                    name {
                      full
                    }
                  }
                  role
                }
              }
            }
          }
        }
      `;

      const anilistRes = await fetch('https://graphql.anilist.co', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
        },
        body: JSON.stringify({
          query: anilistQuery,
          variables: { search: query }
        })
      });

      if (!anilistRes.ok) return [];

      const anilistData = await anilistRes.json();
      const media = anilistData.data?.Page?.media ?? [];

      if (!media.length) return [];

      const books = media.map((manga: any) => ({
        title: manga.title.english || manga.title.romaji || query,
        author: manga.staff.edges?.[0]?.node?.name?.full ?? "Unknown author",
        year: manga.startDate?.year?.toString() ?? "—",
        pages: (manga.chapters || 0).toString(),
        summary: manga.description ? (manga.description.length > 200 ? manga.description.substring(0, 200) + "..." : manga.description) : "No description available.",
        fullSummary: manga.description,
        coverUrl: manga.coverImage?.large,
        key: null
      }));

      // Preload cover images
      books.forEach((book: any) => {
        if (book.coverUrl) {
          const img = new Image();
          img.src = book.coverUrl;
        }
      });

      return books;
    } catch (err) {
      console.error("AniList search failed:", err);
      return [];
    }
  }
  
  async function handleApiSwitch(api: "openlibrary" | "anilist") {
    if (selectedApi === api || !lastQuery) return;
    selectedApi = api;
    
    // Switch to stored results for this API
    const results = apiResults[api];
    if (results) {
      foundBooks = results;
    } else {
      // If we don't have results for this API yet, search it
      isApiSwitching = true;
      try {
        let searchResults: any[] = [];
        if (api === "openlibrary") {
          searchResults = await searchOpenLibrary(lastQuery);
        } else {
          searchResults = await searchAnilist(lastQuery);
        }
        apiResults[api] = searchResults;
        foundBooks = searchResults;
      } catch (err) {
        console.error(`Error searching ${api}:`, err);
        foundBooks = [];
      } finally {
        isApiSwitching = false;
      }
    }
  }

  // --- ORB ANIMATION LOGIC ---
  function handleFocus() {
    if (activeTab !== "home") return;
    isFocused = true;
    settleOrbToCenter();
  }

  function handleBlur() {
    isFocused = false;
    restoreOrbFloat();
  }

  function settleOrbToCenter() {
    if (!orbElement) return;
    const el = orbElement;
    const currentTransform = getComputedStyle(el).transform;
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

  function handleInput() {
    if (activeTab !== "home") return;
    clearTimeout(typingPulseTimeout);
    isPulsing = true;
    typingPulseTimeout = window.setTimeout(() => isPulsing = false, 100);
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
      // Search both APIs in parallel
      const [openLibraryResults, anilistResults] = await Promise.all([
        searchOpenLibrary(query),
        searchAnilist(query)
      ]);

      apiResults.openlibrary = openLibraryResults;
      apiResults.anilist = anilistResults;

      // Show results from the currently selected API, or switch to the one that has results
      if (selectedApi === "openlibrary" && openLibraryResults.length > 0) {
        foundBooks = openLibraryResults;
      } else if (selectedApi === "anilist" && anilistResults.length > 0) {
        foundBooks = anilistResults;
      } else if (openLibraryResults.length > 0) {
        selectedApi = "openlibrary";
        foundBooks = openLibraryResults;
      } else if (anilistResults.length > 0) {
        selectedApi = "anilist";
        foundBooks = anilistResults;
      } else {
        // Both APIs found nothing
        foundBooks = [];
      }

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

  async function saveBook(event: CustomEvent) {
    const { status, pagesRead, totalPages, book } = event.detail;
    
    // Optimistic UI update: close modal and show success immediately
    isPulsing = true;
    setTimeout(() => isPulsing = false, 600);
    showAddDialog = false;
    pendingBook = null;
    
    // Reset search state after adding a book
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
    } catch (err) {
      console.error("Failed to save book:", err);
      // Show error toast if save failed
      showDuplicateToast = true;
      setTimeout(() => showDuplicateToast = false, 3000);
      // Optionally, could revert UI changes here if needed
    }
  }
</script>

<main>
  <Background />
  <!-- Bind activeTab so sidebar clicks update state -->
  <Sidebar bind:activeTab={activeTab} />

  <section class="orb-stage">
    <Orb
      bind:orbElement={orbElement}
      {activeTab} {isReturning} {returnStage} {isGlowing} {isPulsing}
      shouldScale={(isFocused && activeTab === "home" && !isReturning) || isPulsing}
      isAdding={showAddDialog}
    >
      {#if activeTab === "home" && !isReturning}
        <div class="search-container" in:fade={{ duration: 300, delay: 200 }} out:fade={{ duration: 200 }}>
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
            on:openSummary={(e) => summaryBook = e.detail}
            on:done={() => { 
              searchState = "idle";
              foundBooks = [];
              apiResults = { openlibrary: null, anilist: null };
              lastQuery = "";
            }}
            on:apiSwitch={(e) => handleApiSwitch(e.detail)}
          />
        </div>

      {:else if showLibrary}
        <div class="library-wrapper" in:fade={{ duration: 400, delay: 1000 }} out:fade={{ duration: 200 }}>
          <div class="library-container" class:fade-out={isReturning}>
            <Library />
          </div>
        </div>

      {:else if activeTab === "settings"}
        <!-- SETTINGS VIEW -->
        <div class="settings-wrapper" in:fade={{ duration: 400, delay: 300 }} out:fade={{ duration: 200 }}>
           <Settings />
        </div>
      {/if}
    </Orb>
  </section>

  {#if summaryBook}
    <SummaryModal book={summaryBook} on:close={() => summaryBook = null} />
  {/if}

  {#if showAddDialog && pendingBook}
    <AddBookModal book={pendingBook} on:cancel={() => showAddDialog = false} on:save={saveBook} />
  {/if}

  {#if showDuplicateToast}
    <Toast title="Book Exists" message="This book is already in your library." />
  {/if}
</main>

<style>
  main { display: flex; height: 100vh; width: 100vw; position: relative; }
  .orb-stage { flex: 1; display: flex; justify-content: center; align-items: center; position: relative; z-index: 5; }
  .search-container, .library-wrapper, .library-container, .settings-wrapper { width: 100%; height: 100%; }
  .search-container { display: flex; flex-direction: column; justify-content: center; align-items: center; }
  .library-container { transition: opacity 0.25s ease-out; }
  .library-container.fade-out { opacity: 0; }
</style>
