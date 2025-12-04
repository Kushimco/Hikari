<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { fade, scale } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';

  type Book = {
    id: string;
    title: string;
    author: string;
    cover: string;
    cover_color: string; 
    status: string; 
  };

  let books: Book[] = [];
  let activeFilter: string = 'All'; 
  let selectedBook: Book | null = null;

  onMount(async () => {
    loadBooks();
  });

  async function loadBooks() {
    try {
      books = await invoke('get_books');
    } catch (err) {
      console.error("Failed to load books:", err);
    }
  }

  $: filteredBooks = activeFilter === 'All' 
    ? books 
    : books.filter(book => book.status.toLowerCase() === activeFilter.toLowerCase());

  function setFilter(filter: string) {
    activeFilter = filter;
  }

  function openBook(book: Book) {
    selectedBook = book;
  }

  function closeBook() {
    selectedBook = null;
  }

  async function updateStatus(newStatus: string) {
    if (!selectedBook) return;

    // 1. Optimistic Update (UI changes instantly)
    const updatedBook = { ...selectedBook, status: newStatus };
    selectedBook = updatedBook; 
    books = books.map(b => b.id === updatedBook.id ? updatedBook : b);

    // 2. Persist to Backend
    try {
      await invoke('update_book_status', { id: selectedBook.id, status: newStatus });
    } catch (err) {
      console.error("Failed to save status:", err);
      loadBooks(); // Revert on failure
    }
  }
</script>

<!-- Main Content -->
<div class="library-content" class:blurred={selectedBook !== null}>
  <header class="lib-header">
    <h2>Library</h2>
    <div class="filters">
      <button class="filter-pill" class:active={activeFilter === 'All'} on:click={() => setFilter('All')}>All</button>
      <button class="filter-pill" class:active={activeFilter === 'Reading'} on:click={() => setFilter('Reading')}>Reading</button>
      <button class="filter-pill" class:active={activeFilter === 'To Read'} on:click={() => setFilter('To Read')}>To Read</button>
      <button class="filter-pill" class:active={activeFilter === 'Finished'} on:click={() => setFilter('Finished')}>Finished</button>
    </div>
  </header>

  <div class="book-grid">
    {#each filteredBooks as book}
      <div 
        class="book-card" 
        on:click={() => openBook(book)} 
        on:keydown={(e) => e.key === 'Enter' && openBook(book)} 
        tabindex="0" 
        role="button"
      >
        <div class="cover" style="background: linear-gradient(135deg, {book.cover_color || '#FF9A9E'} 0%, white 200%);">
          <span class="status-dot {book.status ? book.status.toLowerCase().replace(' ', '-') : ''}"></span>
        </div>
        <div class="info">
          <h3 class="title">{book.title}</h3>
          <p class="author">{book.author}</p>
        </div>
      </div>
    {/each}
    
    {#if filteredBooks.length === 0}
      <div class="empty-state"><p>No books found.</p></div>
    {/if}
  </div>
</div>

<!-- Details Modal -->
{#if selectedBook}
  <div 
    class="modal-overlay" 
    on:click|self={closeBook} 
    on:keydown={(e) => e.key === 'Escape' && closeBook()}
    tabindex="0"
    role="button"
    aria-label="Close modal"
    transition:fade={{ duration: 200 }}
  >
    <div class="modal-card" role="dialog" aria-modal="true" transition:scale={{ duration: 300, easing: cubicOut, start: 0.95 }}>
      <div class="modal-cover" style="background: linear-gradient(135deg, {selectedBook.cover_color || '#FF9A9E'} 0%, white 200%);">
        <span class="status-badge {selectedBook.status.toLowerCase().replace(' ', '-')}">
          {selectedBook.status}
        </span>
      </div>

      <div class="modal-content">
        <div class="modal-header">
          <h3>{selectedBook.title}</h3>
          <p class="modal-author">{selectedBook.author}</p>
        </div>

        <div class="status-actions">
          <p class="label">Change Status</p>
          <div class="action-buttons">
            {#each ['Reading', 'To Read', 'Finished'] as status}
              <button 
                class="status-btn {status.toLowerCase().replace(' ', '-')}" 
                class:active={selectedBook.status === status}
                on:click={() => updateStatus(status)}
              >
                {status}
              </button>
            {/each}
          </div>
        </div>

        <button class="close-btn" on:click={closeBook}>Close</button>
      </div>
    </div>
  </div>
{/if}

<style>
  @import url('https://fonts.googleapis.com/css2?family=Playfair+Display:ital,wght@0,400;0,600;1,400&display=swap');

  .library-content { transition: filter 0.3s ease; height: 100%; width: 100%; padding-bottom: 40px; animation: fadeIn 0.6s ease 0.6s forwards; opacity: 0; }
  .library-content.blurred { filter: blur(8px); pointer-events: none; }

  /* Modal Styles */
  .modal-overlay { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(255, 255, 255, 0.6); display: flex; justify-content: center; align-items: center; z-index: 100; backdrop-filter: blur(4px); }
  .modal-card { background: rgba(255, 255, 255, 0.85); backdrop-filter: blur(20px); width: 600px; height: 350px; border-radius: 24px; box-shadow: 0 20px 60px rgba(94, 75, 75, 0.15); display: flex; overflow: hidden; border: 1px solid rgba(255, 255, 255, 0.8); }
  .modal-cover { width: 240px; height: 100%; position: relative; }
  
  .status-badge { position: absolute; top: 16px; left: 16px; padding: 6px 12px; border-radius: 20px; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; background: white; box-shadow: 0 4px 10px rgba(0,0,0,0.1); }
  .status-badge.reading { color: #47f386; }
  .status-badge.finished { color: #529ffd; }
  .status-badge.to-read { color: #ff4eaf; }

  .modal-content { flex: 1; padding: 32px; display: flex; flex-direction: column; justify-content: space-between; }
  .modal-header h3 { font-family: 'Playfair Display', serif; font-size: 2rem; margin: 0 0 8px 0; color: #2c1810; line-height: 1.1; }
  .modal-author { font-size: 0.9rem; color: rgba(94, 75, 75, 0.7); text-transform: uppercase; margin: 0; }

  .label { font-size: 0.8rem; font-weight: 600; color: rgba(94, 75, 75, 0.5); text-transform: uppercase; margin-bottom: 12px; }
  .action-buttons { display: flex; gap: 10px; }
  
  .status-btn { flex: 1; padding: 10px; border: none; border-radius: 12px; font-size: 0.85rem; font-weight: 600; cursor: pointer; transition: all 0.2s; background: rgba(94, 75, 75, 0.05); color: rgba(94, 75, 75, 0.6); }
  .status-btn:hover { transform: translateY(-2px); }
  .status-btn.reading:hover, .status-btn.reading.active { background: #47f386; color: white; }
  .status-btn.to-read:hover, .status-btn.to-read.active { background: #ff4eaf; color: white; }
  .status-btn.finished:hover, .status-btn.finished.active { background: #529ffd; color: white; }
  .status-btn.active { box-shadow: 0 4px 12px rgba(0,0,0,0.15); }

  .close-btn { align-self: flex-end; background: transparent; border: 1px solid rgba(94, 75, 75, 0.2); padding: 8px 20px; border-radius: 20px; color: rgba(94, 75, 75, 0.6); cursor: pointer; transition: all 0.2s; }
  .close-btn:hover { background: rgba(94, 75, 75, 0.05); color: #5e4b4b; }

  /* Grid & Filters */
  .lib-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 25px; }
  h2 { margin: 0; font-family: 'Playfair Display', serif; font-size: 2.5rem; font-weight: 600; color: #4a3b3b; }
  .filters { display: flex; gap: 8px; }
  .filter-pill { background: rgba(255, 255, 255, 0.3); border: 1px solid rgba(255, 255, 255, 0.5); padding: 6px 16px; border-radius: 20px; font-size: 0.85rem; color: #6b5b5b; cursor: pointer; transition: all 0.2s; }
  .filter-pill.active { background: #5e4b4b; color: white; border-color: transparent; }
  
  .book-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); gap: 24px; padding-right: 10px; }
  .book-card { display: flex; flex-direction: column; gap: 12px; cursor: pointer; transition: transform 0.2s; }
  .book-card:hover { transform: translateY(-8px); }
  .cover { aspect-ratio: 2 / 3; width: 100%; border-radius: 12px; position: relative; box-shadow: 0 4px 10px rgba(0,0,0,0.05); overflow: hidden; }
  
  .status-dot { position: absolute; top: 10px; right: 10px; width: 8px; height: 8px; border-radius: 50%; }
  .status-dot.reading { background: #47f386; }
  .status-dot.finished { background: #529ffd; }
  .status-dot.to-read { background: #ff4eaf; }
  
  .info { padding: 0 4px; }
  .title { font-family: 'Playfair Display', serif; font-size: 1.1rem; font-weight: 600; color: #2c1810; margin: 0 0 4px 0; }
  .author { font-size: 0.75rem; font-weight: 500; color: rgba(94, 75, 75, 0.65); text-transform: uppercase; }
  .empty-state { grid-column: 1 / -1; text-align: center; padding: 40px; color: rgba(94, 75, 75, 0.5); font-style: italic; }
  @keyframes fadeIn { to { opacity: 1; } }
</style>
