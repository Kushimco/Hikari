<script lang="ts">
  import { onMount } from 'svelte';
  import { flip } from 'svelte/animate';
  
  // Imports
  import * as API from './library-components/api';
  import type { Book } from './library-components/api';
  import BookCard from './library-components/BookCard.svelte';
  import BookDetailModal from './library-components/BookDetailModal.svelte';
  import LibraryHeader from './library-components/LibraryHeader.svelte'; 

  // -- STATE --
  let books: Book[] = [];
  let activeFilter = 'All';
  let sortOption = 'Newest';
  let searchQuery = "";
  let selectedBook: Book | null = null;

  onMount(loadBooks);

  async function loadBooks() {
    try {
      books = await API.getBooks();
    } catch (err) {
      console.error("Failed to load books:", err);
    }
  }

  // -- DERIVED STATE --
  $: filteredBooks = books.filter((book: Book) => {
    const matchesStatus = activeFilter === 'All' ? true : book.status.toLowerCase() === activeFilter.toLowerCase().replace(' ', '-');
    const q = searchQuery.trim().toLowerCase();
    const matchesSearch = q === "" ? true : book.title.toLowerCase().includes(q) || book.author.toLowerCase().includes(q);
    return matchesStatus && matchesSearch;
  });

  $: sortedBooks = [...filteredBooks].sort((a, b) => {
    if (sortOption === 'Newest') return (new Date(b.date_added || 0).getTime()) - (new Date(a.date_added || 0).getTime());
    if (sortOption === 'Oldest') return (new Date(a.date_added || 0).getTime()) - (new Date(b.date_added || 0).getTime());
    if (sortOption === 'A-Z') return a.title.localeCompare(b.title);
    return 0;
  });

  // -- HANDLERS --
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
</script>

<!-- Main Wrapper: Takes full height, handles positioning -->
<div class="library-wrapper">
  
  <!-- Content Area: Scrolls independently -->
  <!-- We add 'noscroll' class when a book is selected to lock the background -->
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

  <!-- Modal: Sits OUTSIDE the scroll area, fixed to the wrapper -->
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
  /* 1. Ensure the wrapper fills the parent and creates a positioning context */
  .library-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden; /* Prevents double scrollbars */
  }

  /* 2. The scrollable area handles the overflowing content */
  .library-scroll-area {
    width: 100%;
    height: 100%;
    overflow-y: auto; /* This is the ONLY thing that scrolls */
    overflow-x: hidden;
    padding-bottom: 40px;
    transition: filter 0.2s ease;
    -ms-overflow-style: none;  /* IE and Edge */
    scrollbar-width: none; 
  }

  .library-scroll-area::-webkit-scrollbar {
    display: none;
  }
  
  /* 3. Locking scroll when modal is open */
  .library-scroll-area.noscroll {
    overflow: hidden;
  }

  .library-scroll-area.blurred {
    filter: blur(5px);
    pointer-events: none; /* Prevents clicking background books */
  }

  /* 4. Layout for content */
  .header-container {
    padding: 0 10px; /* Match grid padding mostly */
  }

  .book-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 24px;
    padding: 0 20px; /* Added padding so cards don't touch edges */
    margin-top: 10px;
  }

  .empty-state {
    grid-column: 1 / -1;
    text-align: center;
    padding: 40px;
    color: rgba(94, 75, 75, 0.5);
    font-style: italic;
  }

  /* 5. Modal Overlay: Fixed relative to the Wrapper (which doesn't scroll) */
  .modal-overlay {
    position: absolute; /* Absolute to the wrapper */
    inset: 0; /* Cover the whole area */
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 100; /* Ensure it's on top */
    /* Note: Background dimming is usually handled inside BookDetailModal's backdrop, 
       but if not, add background: rgba(0,0,0,0.5) here */
  }
</style>
