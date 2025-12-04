<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { fade, scale } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import { flip } from 'svelte/animate'; 

  // -- TYPES --
  type Book = {
    id: string;
    title: string;
    author: string;
    cover: string;
    cover_color: string; 
    status: string; 
    date_added?: string; 
  };

  // -- STATE --
  let books: Book[] = [];
  let activeFilter: string = 'All'; 
  let sortOption: string = 'Newest'; 
  let searchQuery: string = ""; 
  let selectedBook: Book | null = null;
  let showSortMenu = false; 
  let showDeleteConfirm = false;

  onMount(async () => {
    loadBooks();
  });

  // -- DATA FETCHING --
  async function loadBooks() {
    try {
      books = await invoke('get_books');
    } catch (err) {
      console.error("Failed to load books:", err);
    }
  }

  // -- DERIVED STATE (Filter & Sort) --
  
  // Combine Status Filter and Search Query
  $: filteredBooks = books.filter(book => {
    const matchesStatus = activeFilter === 'All' ? true : book.status.toLowerCase() === activeFilter.toLowerCase();
    const matchesSearch = searchQuery.trim() === "" 
      ? true 
      : book.title.toLowerCase().includes(searchQuery.toLowerCase()) || 
        book.author.toLowerCase().includes(searchQuery.toLowerCase());
    
    return matchesStatus && matchesSearch;
  });

  // Sort the filtered results
  $: sortedBooks = [...filteredBooks].sort((a, b) => {
    if (sortOption === 'Newest') {
      return (new Date(b.date_added || 0).getTime()) - (new Date(a.date_added || 0).getTime());
    } else if (sortOption === 'Oldest') {
      return (new Date(a.date_added || 0).getTime()) - (new Date(b.date_added || 0).getTime());
    } else if (sortOption === 'A-Z') {
      return a.title.localeCompare(b.title);
    }
    return 0;
  });

  // -- UI HANDLERS --

  function setFilter(filter: string) { activeFilter = filter; }

  // Dropdown Logic
  function toggleSortMenu() { showSortMenu = !showSortMenu; }
  function closeMenu() { showSortMenu = false; }
  function selectSort(option: string) {
    sortOption = option;
    showSortMenu = false;
  }

  // Modal Logic
  function openBook(book: Book) { selectedBook = book; }
  function closeBook() { 
    selectedBook = null; 
    showDeleteConfirm = false; 
  }
  
  // -- BOOK ACTIONS --

  // Updates book
  async function updateStatus(newStatus: string) {
    if (!selectedBook) return;
    const updatedBook = { ...selectedBook, status: newStatus };
    selectedBook = updatedBook; 
    books = books.map(b => b.id === updatedBook.id ? updatedBook : b);
    try {
      await invoke('update_book_status', { id: selectedBook.id, status: newStatus });
    } catch (err) {
      console.error("Failed to save status:", err);
      loadBooks(); 
    }
  }

  // Delete Flow
  function confirmDelete() { showDeleteConfirm = true; }
  function cancelDelete() { showDeleteConfirm = false; }

  async function deleteBook() {
    if (!selectedBook) return;
    try {
      await invoke('delete_book', { id: selectedBook.id });
      books = books.filter(b => b.id !== selectedBook?.id);
      showDeleteConfirm = false;
      closeBook();
    } catch (err) {
      console.error("Failed to delete book:", err);
    }
  }
</script>

<div class="library-content" class:blurred={selectedBook !== null}>
  <header class="lib-header">
    <div class="title-group">
      <h2>Library</h2>
      
      <div class="header-controls">
        <!-- SEARCH BAR -->
        <div class="search-wrapper">
          <svg class="search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="11" cy="11" r="8"></circle>
            <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
          </svg>
          <input 
            type="text" 
            placeholder="Search books..." 
            bind:value={searchQuery}
          />
        </div>

        <!-- SORT DROPDOWN (CUSTOM) -->
        <div class="sort-controls">
          <div class="custom-select-wrapper">
              <!-- Trigger Button -->
              <button class="custom-select-trigger" on:click|stopPropagation={toggleSortMenu}>
                  {sortOption}
                  <svg class="chevron" class:rotated={showSortMenu} width="10" height="6" viewBox="0 0 10 6" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M1 1L5 5L9 1"/>
                  </svg>
              </button>
              
              <!-- Dropdown Options -->
              {#if showSortMenu}
                  <div class="custom-options" transition:fade={{ duration: 100 }}>
                      {#each ['Newest', 'Oldest', 'A-Z'] as option}
                          <button class="custom-option" class:selected={sortOption === option} on:click={() => selectSort(option)}>
                              {option}
                          </button>
                      {/each}
                  </div>
              {/if}
          </div>
          <!-- Click-outside Overlay -->
          {#if showSortMenu}
              <div class="menu-overlay" on:click={closeMenu} role="button" tabindex="0" on:keydown={closeMenu}></div>
          {/if}
        </div>
      </div>
    </div>

    <!-- FILTER TABS -->
    <div class="filters">
      {#each ['All', 'Reading', 'To Read', 'Finished'] as filter}
        <button class="filter-pill" class:active={activeFilter === filter} on:click={() => setFilter(filter)}>
            {filter}
        </button>
      {/each}
    </div>
  </header>

  <!-- BOOK GRID -->
  <div class="book-grid">
    {#each sortedBooks as book (book.id)}
      <div 
        class="book-card" 
        on:click={() => openBook(book)} 
        on:keydown={(e) => e.key === 'Enter' && openBook(book)} 
        tabindex="0" 
        role="button" 
        animate:flip={{ duration: 300 }}
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
    
    {#if sortedBooks.length === 0}
      <div class="empty-state">
        <p>{searchQuery ? `No books match "${searchQuery}"` : "No books found."}</p>
      </div>
    {/if}
  </div>
</div>

<!-- DETAIL MODAL -->
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
      
      <!-- Large Cover -->
      <div class="modal-cover" style="background: linear-gradient(135deg, {selectedBook.cover_color || '#FF9A9E'} 0%, white 200%);">
        <span class="status-badge {selectedBook.status.toLowerCase().replace(' ', '-')}">{selectedBook.status}</span>
      </div>

      <!-- Info & Controls -->
      <div class="modal-content">
        <div class="modal-header">
          <h3>{selectedBook.title}</h3>
          <p class="modal-author">{selectedBook.author}</p>
          {#if selectedBook.date_added}
             <p class="modal-date">Added: {new Date(selectedBook.date_added).toLocaleDateString()}</p>
          {/if}
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

        <div class="modal-footer">
            <button class="delete-btn" on:click={confirmDelete} title="Delete Book">
                <!-- Trash Icon -->
                <svg width="19" height="19" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="3 6 5 6 21 6"></polyline>
                    <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
                </svg>
            </button>
            <button class="close-btn" on:click={closeBook}>Close</button>
        </div>
      </div>
      
      <!-- NESTED DELETE CONFIRMATION -->
      {#if showDeleteConfirm}
        <div class="delete-confirm-overlay" transition:fade={{ duration: 150 }}>
            <div class="delete-confirm-card" transition:scale={{ duration: 200, start: 0.9 }}>
                <h4>Delete this book?</h4>
                <p>This action cannot be undone.</p>
                <div class="confirm-actions">
                    <button class="cancel-btn" on:click={cancelDelete}>Cancel</button>
                    <button class="confirm-delete-btn" on:click={deleteBook}>Delete</button>
                </div>
            </div>
        </div>
      {/if}

    </div>
  </div>
{/if}

<style>
  @import url('https://fonts.googleapis.com/css2?family=Playfair+Display:ital,wght@0,400;0,600;1,400&display=swap');

  /* -- BASE LAYOUT -- */
  .library-content { transition: filter 0.3s ease; height: 100%; width: 100%; padding-bottom: 40px; animation: fadeIn 0.6s ease 0.6s forwards; opacity: 0; }
  .library-content.blurred { filter: blur(8px); pointer-events: none; }

  /* -- HEADER -- */
  .lib-header { display: flex; flex-direction: column; gap: 16px; margin-bottom: 25px; align-items: flex-start; }
  .title-group { display: flex; justify-content: space-between; width: 100%; align-items: center; }
  h2 { margin: 0; font-family: 'Playfair Display', serif; font-size: 2.5rem; font-weight: 600; color: #4a3b3b; }

  /* -- CONTROLS (Search + Sort) -- */
  .header-controls { display: flex; gap: 12px; align-items: center; }

  /* SEARCH */
  .search-wrapper { position: relative; display: flex; align-items: center; }
  .search-icon { position: absolute; left: 12px; color: rgba(94, 75, 75, 0.4); pointer-events: none; }
  .search-wrapper input {
    background: rgba(255, 255, 255, 0.4);
    border: 1px solid rgba(255, 255, 255, 0.6);
    border-radius: 12px;
    padding: 6px 12px 6px 36px; /* Left padding for icon */
    font-size: 0.85rem;
    color: #5e4b4b;
    font-family: 'Inter', sans-serif;
    width: 180px;
    outline: none;
    transition: all 0.2s;
  }
  .search-wrapper input:focus { background: rgba(255, 255, 255, 0.6); border-color: rgba(255, 255, 255, 0.9); width: 220px; }
  .search-wrapper input::placeholder { color: rgba(94, 75, 75, 0.4); }

  /* DROPDOWN */
  .sort-controls { display: flex; align-items: center; gap: 8px; font-family: 'Inter', sans-serif; position: relative; z-index: 50; }
  .custom-select-wrapper { position: relative; }
  .custom-select-trigger { background: rgba(255, 255, 255, 0.4); border: 1px solid rgba(255, 255, 255, 0.6); border-radius: 12px; padding: 6px 12px; font-size: 0.85rem; color: #5e4b4b; cursor: pointer; display: flex; align-items: center; gap: 8px; font-weight: 600; font-family: 'Inter', sans-serif; transition: all 0.2s; min-width: 90px; justify-content: space-between; }
  .custom-select-trigger:hover { background: rgba(255, 255, 255, 0.6); }
  .custom-select-trigger:focus { outline: none; }
  .chevron { transition: transform 0.2s; opacity: 0.6; }
  .chevron.rotated { transform: rotate(180deg); }
  .custom-options { position: absolute; top: calc(100% + 6px); right: 0; left: 0; background: #fff6f0; border: 1px solid rgba(255, 255, 255, 0.8); border-radius: 12px; box-shadow: 0 4px 12px rgba(94, 75, 75, 0.1); padding: 4px; display: flex; flex-direction: column; min-width: 100px; backdrop-filter: blur(10px); }
  .custom-option { background: transparent; border: none; text-align: left; padding: 8px 12px; font-size: 0.85rem; color: rgba(94, 75, 75, 0.8); cursor: pointer; border-radius: 8px; transition: all 0.15s; font-weight: 500; }
  .custom-option:hover { background: rgba(94, 75, 75, 0.08); color: #5e4b4b; }
  .custom-option.selected { background: rgba(94, 75, 75, 0.15); color: #5e4b4b; font-weight: 600; }
  .custom-option:focus { outline: none; }
  .menu-overlay { position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: -1; cursor: default; }

  /* -- FILTERS -- */
  .filters { display: flex; gap: 8px; }
  .filter-pill { background: rgba(255, 255, 255, 0.3); border: 1px solid rgba(255, 255, 255, 0.5); padding: 6px 16px; border-radius: 20px; font-size: 0.85rem; color: #6b5b5b; cursor: pointer; transition: all 0.2s; }
  .filter-pill.active { background: #5e4b4b; color: white; border-color: transparent; }
  .filter-pill:focus { outline: none; }

  /* -- GRID -- */
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

  /* -- MODAL -- */
  .modal-overlay { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(255, 240, 230, 0.4); display: flex; justify-content: center; align-items: center; z-index: 100; backdrop-filter: blur(4px); }
  .modal-overlay:focus { outline: none; }
  .modal-card { position: relative; background: #fbddc8; backdrop-filter: blur(20px); width: 600px; height: 350px; border-radius: 24px; box-shadow: 0 20px 60px rgba(94, 75, 75, 0.15); display: flex; overflow: hidden; border: 1px solid rgba(255, 255, 255, 0.8); }
  .modal-cover { width: 240px; height: 100%; position: relative; }
  .status-badge { position: absolute; top: 16px; left: 16px; padding: 6px 12px; border-radius: 20px; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; background: white; box-shadow: 0 4px 10px rgba(0,0,0,0.1); }
  .status-badge.reading { color: #47f386; }
  .status-badge.finished { color: #529ffd; }
  .status-badge.to-read { color: #ff4eaf; }
  .modal-content { flex: 1; padding: 32px; display: flex; flex-direction: column; justify-content: space-between; }
  .modal-header h3 { font-family: 'Playfair Display', serif; font-size: 2rem; margin: 0 0 8px 0; color: #2c1810; line-height: 1.1; }
  .modal-author { font-size: 0.9rem; color: rgba(94, 75, 75, 0.7); text-transform: uppercase; margin: 0; }
  .modal-date { font-size: 0.7rem; color: rgba(94, 75, 75, 0.4); margin-top: 4px; }
  .label { font-size: 0.8rem; font-weight: 600; color: rgba(94, 75, 75, 0.5); text-transform: uppercase; margin-bottom: 12px; }
  .action-buttons { display: flex; gap: 10px; }
  
  /* BUTTONS */
  .status-btn { flex: 1; padding: 12px; border: none; border-radius: 12px; font-size: 0.85rem; font-weight: 600; cursor: pointer; transition: all 0.2s ease; background: #5e4b4b; color: rgba(255, 255, 255, 0.9); opacity: 0.9; }
  .status-btn:hover { opacity: 1; transform: translateY(-1px); }
  .status-btn:focus { outline: none; }
  .status-btn.reading:hover, .status-btn.reading.active { background: #47f386; color: white; box-shadow: 0 4px 12px rgba(71, 243, 134, 0.3); }
  .status-btn.to-read:hover, .status-btn.to-read.active { background: #ff4eaf; color: white; box-shadow: 0 4px 12px rgba(255, 78, 175, 0.3); }
  .status-btn.finished:hover, .status-btn.finished.active { background: #529ffd; color: white; box-shadow: 0 4px 12px rgba(82, 159, 253, 0.3); }

  .modal-footer { display: flex; justify-content: space-between; align-items: center; width: 100%; }
  .delete-btn { background: transparent; border: none; color: rgba(255, 98, 98, 0.7); cursor: pointer; transition: all 0.2s; padding: 8px; border-radius: 8px; display: flex; align-items: center; justify-content: center; }
  .delete-btn:hover { background: rgba(200, 50, 50, 0.1); color: rgba(200, 50, 50, 0.8); transform: scale(1.09); }
  .delete-btn:focus { outline: none; }
  .close-btn { background: transparent; border: 1.5px solid #5e4b4b; padding: 8px 24px; border-radius: 24px; color: #5e4b4b; font-weight: 600; cursor: pointer; font-size: 0.85rem; transition: all 0.2s; opacity: 0.8; }
  .close-btn:hover { background: #5e4b4b; color: white; opacity: 1; }
  .close-btn:focus { outline: none; }

  /* -- DELETE CONFIRMATION -- */
  .delete-confirm-overlay { position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: rgba(255, 234, 219, 0.685); backdrop-filter: blur(5px); display: flex; justify-content: center; align-items: center; z-index: 50; border-radius: 24px; }
  .delete-confirm-card { background: #ffd3d3ce; padding: 24px; border-radius: 16px; box-shadow: 0 10px 30px rgba(0,0,0,0.1); text-align: center; border: 1px solid rgba(200, 50, 50, 0.1); width: 80%; }
  .delete-confirm-card h4 { margin: 0 0 8px 0; color: #4a3b3b; font-size: 1.1rem; }
  .delete-confirm-card p { margin: 0 0 20px 0; font-size: 0.85rem; color: rgba(94, 75, 75, 0.6); }
  .confirm-actions { display: flex; gap: 12px; justify-content: center; }
  .cancel-btn { background: rgba(94, 75, 75, 0.1); color: #5e4b4b; border: none; padding: 8px 16px; border-radius: 8px; font-size: 0.8rem; font-weight: 600; cursor: pointer; }
  .cancel-btn:hover { background: rgba(94, 75, 75, 0.2); }
  .confirm-delete-btn { background: #ff4e4e; color: white; border: none; padding: 8px 16px; border-radius: 8px; font-size: 0.8rem; font-weight: 600; cursor: pointer; box-shadow: 0 2px 8px rgba(255, 78, 78, 0.3); }
  .confirm-delete-btn:hover { background: #d93636; }

  @keyframes fadeIn { to { opacity: 1; } }
</style>
