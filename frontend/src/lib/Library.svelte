<script lang="ts">
  import { onMount } from 'svelte';
  import { flip } from 'svelte/animate';
  
  // Imports
  import * as API from './library-components/api';
  import type { Book } from './library-components/api';
  import BookCard from './library-components/BookCard.svelte';
  import BookDetailModal from './library-components/BookDetailModal.svelte';
  import LibraryHeader from './library-components/LibraryHeader.svelte'; 

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
      } catch (err) {
        console.error("Failed to load books:", err);
      }
    }

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
