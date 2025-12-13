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
  // When modal emits 'update', we update local state and backend
  async function handleBookUpdate(event: CustomEvent<Book>) {
    const updatedBook = event.detail;
    
    // Optimistic UI update
    books = books.map(b => b.id === updatedBook.id ? updatedBook : b);
    if (selectedBook && selectedBook.id === updatedBook.id) selectedBook = updatedBook;

    // Backend updates
    try {
      if (updatedBook.pages_read !== undefined) {
        await API.updateBookProgress(updatedBook.id, updatedBook.pages_read);
      }
      if (updatedBook.status) {
        await API.updateBookStatus(updatedBook.id, updatedBook.status);
      }
    } catch (err) {
      console.error("Sync failed", err);
      loadBooks(); // Revert on error
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

<div class="library-content" class:blurred={selectedBook !== null}>
  
  <!-- Header handles its own UI, just binds variables -->
  <LibraryHeader 
    bind:activeFilter 
    bind:sortOption 
    bind:searchQuery 
  />

  <div class="book-grid">
    {#each sortedBooks as book (book.id)}
      <div animate:flip={{ duration: 300 }}>
        <!-- Clean Card Component -->
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

<!-- Modal handles its own logic, emits events up -->
{#if selectedBook}
  <BookDetailModal 
    book={selectedBook} 
    on:close={() => selectedBook = null}
    on:update={handleBookUpdate}
    on:delete={handleBookDelete}
  />
{/if}

<style>
  /* Only layout styles remain here */
  .library-content { height: 100%; width: 100%; padding-bottom: 40px; }
  .library-content.blurred { filter: blur(8px); }
  .book-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); gap: 24px; padding-right: 10px; }
  .empty-state { grid-column: 1 / -1; text-align: center; padding: 40px; color: rgba(94, 75, 75, 0.5); font-style: italic; }
</style>
