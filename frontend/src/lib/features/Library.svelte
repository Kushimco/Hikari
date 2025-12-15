<script lang="ts">
  import { onMount } from 'svelte';
  import { flip } from 'svelte/animate';
  import { invoke } from '@tauri-apps/api/core'; // Needed for cover update command
  
  // Imports
  import * as API from '$lib/library-components/api';
  import type { Book } from '$lib/library-components/api';
  import BookCard from '$lib/library-components/BookCard.svelte';
  import BookDetailModal from '$lib/library-components/BookDetailModal.svelte';
  import LibraryHeader from '$lib/library-components/LibraryHeader.svelte'; 

  /* 
      ================================================================
      SECTION 1: STATE MANAGEMENT
      ================================================================
  */
  // {
    let books: Book[] = [];
    let activeFilter = 'All';
    let sortOption = 'Newest';
    let searchQuery = "";
    let selectedBook: Book | null = null;
  // }

  /* 
      ================================================================
      SECTION 2: COMPUTED & LIFECYCLE
      ================================================================
  */
  // {
    onMount(loadBooks);

    async function loadBooks() {
      try {
        books = await API.getBooks();
        // Run auto-fix silently after loading
        fixMissingCovers();
      } catch (err) {
        console.error("Failed to load books:", err);
      }
    }

    // --- AUTO-FIX MISSING COVERS LOGIC ---
    async function fixMissingCovers() {
        const missing = books.filter(b => !b.cover || b.cover.trim() === "");
        if (missing.length === 0) return;

        console.log(`Fixing ${missing.length} missing covers...`);

        for (const book of missing) {
            let newCover = await searchOpenLibraryCover(book.title, book.author);
            
            if (!newCover) {
                newCover = await searchAniListCover(book.title);
            }

            if (newCover) {
                // 1. Update UI Instantly
                books = books.map(b => b.id === book.id ? { ...b, cover: newCover! } : b);
                
                // 2. Save to Backend (using invoke directly or adding to your API class)
                try {
                    // Make sure 'update_book_cover' is registered in Rust!
                    await invoke('update_book_cover', { id: book.id, cover: newCover });
                    console.log(`Updated cover for: ${book.title}`);
                } catch (err) {
                    console.error(`Failed to save cover for ${book.title}`, err);
                }
            }
        }
    }

    async function searchOpenLibraryCover(title: string, author: string): Promise<string | null> {
        try {
            const q = encodeURIComponent(`${title} ${author}`);
            const res = await fetch(`https://openlibrary.org/search.json?q=${q}&limit=1&fields=cover_i`);
            const data = await res.json();
            if (data.docs?.[0]?.cover_i) {
                return `https://covers.openlibrary.org/b/id/${data.docs[0].cover_i}-L.jpg`;
            }
        } catch (e) { return null; }
        return null;
    }

    async function searchAniListCover(title: string): Promise<string | null> {
        try {
            const query = `query ($search: String) { Media(search: $search, type: MANGA) { coverImage { large } } }`;
            const res = await fetch('https://graphql.anilist.co', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ query, variables: { search: title } })
            });
            const data = await res.json();
            return data.data?.Media?.coverImage?.large || null;
        } catch (e) { return null; }
    }
    // -------------------------------------


    // Filter Logic
    $: filteredBooks = books.filter((book: Book) => {
      const matchesStatus = activeFilter === 'All' ? true : book.status.toLowerCase() === activeFilter.toLowerCase().replace(' ', '-');
      const q = searchQuery.trim().toLowerCase();
      const matchesSearch = q === "" ? true : book.title.toLowerCase().includes(q) || book.author.toLowerCase().includes(q);
      return matchesStatus && matchesSearch;
    });

    // Sort Logic
    $: sortedBooks = [...filteredBooks].sort((a, b) => {
      if (sortOption === 'Newest') return (new Date(b.date_added || 0).getTime()) - (new Date(a.date_added || 0).getTime());
      if (sortOption === 'Oldest') return (new Date(a.date_added || 0).getTime()) - (new Date(b.date_added || 0).getTime());
      if (sortOption === 'A-Z') return a.title.localeCompare(b.title);
      return 0;
    });
  // }

  /* 
      ================================================================
      SECTION 3: ACTION HANDLERS
      ================================================================
  */
  // {
    async function handleBookUpdate(event: CustomEvent<Book>) {
      const updatedBook = event.detail;
      books = books.map(b => b.id === updatedBook.id ? updatedBook : b);
      if (selectedBook && selectedBook.id === updatedBook.id) selectedBook = updatedBook;

      try {
        if (updatedBook.pages_read !== undefined) await API.updateBookProgress(updatedBook.id, updatedBook.pages_read);
        if (updatedBook.status) await API.updateBookStatus(updatedBook.id, updatedBook.status);
      } catch (err) {
        console.error("Sync failed", err);
        loadBooks();
      }
    }

    async function handleBookDelete(event: CustomEvent<string>) {
      const id = event.detail;
      try {
        await API.deleteBook(id);
        books = books.filter(b => b.id !== id);
        selectedBook = null;
      } catch (err) {
        console.error("Delete failed", err);
      }
    }
  // }
</script>

<!-- Main Wrapper -->
<div class="library-wrapper">
  
  <!-- Scrollable Content Area -->
  <div class="library-scroll-area" class:noscroll={selectedBook !== null} class:blurred={selectedBook !== null}>
    
    <div class="header-container">
      <LibraryHeader 
        bind:activeFilter 
        bind:sortOption 
        bind:searchQuery 
      />
    </div>

    <div class="book-grid">
      {#each sortedBooks as book (book.id)}
        <div animate:flip={{ duration: 300 }}>
          <BookCard {book} on:click={() => selectedBook = book} />
        </div>
      {/each}

      {#if sortedBooks.length === 0}
        <div class="empty-state">
          <p>{searchQuery ? `No books match "${searchQuery}"` : "No books found."}</p>
        </div>
      {/if}
    </div>
  </div>

  <!-- Detail Modal -->
  {#if selectedBook}
    <div class="modal-overlay">
      <BookDetailModal 
        book={selectedBook} 
        on:close={() => selectedBook = null}
        on:update={handleBookUpdate}
        on:delete={handleBookDelete}
      />
    </div>
  {/if}

</div>

<style>
  /* --- LAYOUT CONTAINERS --- */
  .library-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .library-scroll-area {
    width: 100%;
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
    padding-bottom: 40px;
    transition: filter 0.2s ease;
    -ms-overflow-style: none;
    scrollbar-width: none; 
  }

  .library-scroll-area::-webkit-scrollbar {
    display: none;
  }
  
  /* Modal States */
  .library-scroll-area.noscroll { overflow: hidden; }
  .library-scroll-area.blurred { filter: blur(5px); pointer-events: none; }

  /* --- CONTENT STYLES --- */
  .header-container {
    padding: 0 10px;
  }

  .book-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 24px;
    padding: 0 20px;
    margin-top: 10px;
  }

  .empty-state {
    grid-column: 1 / -1;
    text-align: center;
    padding: 40px;
    color: rgba(94, 75, 75, 0.5);
    font-style: italic;
  }

  /* --- MODAL OVERLAY --- */
  .modal-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 100;
  }
</style>
