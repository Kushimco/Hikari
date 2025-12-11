<script lang="ts">
  import { invoke, convertFileSrc } from '@tauri-apps/api/core';
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
    total_pages: number;
    pages_read: number;
  };

  // -- STATE --
  let books: Book[] = [];
  let activeFilter: string = 'All';
  let sortOption: string = 'Newest';
  let searchQuery: string = "";
  let selectedBook: Book | null = null;
  let showSortMenu = false;
  let showDeleteConfirm = false;

  // Edit mode state for the progress number
  let isEditingProgress = false;

  onMount(async () => {
    await loadBooks();
  });

  // -- DATA FETCHING --
  async function loadBooks() {
    try {
      books = await invoke<Book[]>('get_books');
      console.log("Initial load:", books);
    } catch (err) {
      console.error("Failed to load books:", err);
    }
  }

  function coverSrc(book: Book | null): string {
    if (!book || !book.cover) return "";
    if (book.cover.startsWith("http://") || book.cover.startsWith("https://")) {
      return book.cover;
    }
    return convertFileSrc(book.cover);
  }

  // -- DERIVED STATE --
  $: filteredBooks = books.filter(book => {
    const matchesStatus =
      activeFilter === 'All'
        ? true
        : book.status.toLowerCase() === activeFilter.toLowerCase().replace(' ', '-');

    const q = searchQuery.trim().toLowerCase();
    const matchesSearch =
      q === ""
        ? true
        : book.title.toLowerCase().includes(q) ||
          book.author.toLowerCase().includes(q);

    return matchesStatus && matchesSearch;
  });

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

  function toggleSortMenu() { showSortMenu = !showSortMenu; }
  function closeMenu() { showSortMenu = false; }
  function selectSort(option: string) {
    sortOption = option;
    showSortMenu = false;
  }

  function openBook(book: Book) { 
    console.log("OPENING BOOK:", book);
    selectedBook = book; 
    isEditingProgress = false; // Reset edit mode on open
  }

  async function closeBook() {
    if (selectedBook) {
      const finalBookState = selectedBook; 
      console.log(`[CLOSE] Saving progress: ${finalBookState.pages_read} pages.`);

      try {
        // NOTE: 'pagesRead' must be camelCase for the Tauri command
        await invoke('update_book_progress', { 
          id: finalBookState.id, 
          pagesRead: finalBookState.pages_read 
        });
        
        await invoke('update_book_status', {
           id: finalBookState.id,
           status: finalBookState.status 
        });
        
        console.log("[CLOSE] Save successful.");
      } catch (err) {
        console.error("[CLOSE] Failed to save:", err);
      }
    }

    selectedBook = null;
    showDeleteConfirm = false;
    isEditingProgress = false;
  }

  // -- BOOK ACTIONS --
  async function updateStatus(newStatus: string) {
    if (!selectedBook) return;
    const s = newStatus.toLowerCase().replace(" ", "-");
    const updatedBook = { ...selectedBook, status: s };
    
    // Optimistic update
    selectedBook = updatedBook;
    books = books.map(b => b.id === updatedBook.id ? updatedBook : b);

    try {
      await invoke('update_book_status', { id: selectedBook.id, status: s });
    } catch (err) {
      console.error("Failed to save status:", err);
      loadBooks(); 
    }
  }

  // -- PROGRESS LOGIC --
  function updateProgress(delta: number) {
    if (!selectedBook) return;

    let newPages = selectedBook.pages_read + delta;
    
    if (newPages < 0) newPages = 0;
    if (selectedBook.total_pages > 0 && newPages > selectedBook.total_pages) {
      newPages = selectedBook.total_pages;
    }

    applyProgressChange(newPages);
  }

  // Centralized function to apply changes and update status
  function applyProgressChange(newPages: number) {
    if (!selectedBook) return;

    const updatedBook = { ...selectedBook, pages_read: newPages };

    // Auto-update status
    if (updatedBook.total_pages > 0 && newPages >= updatedBook.total_pages) {
      updatedBook.status = "finished";
    } else if (newPages > 0 && updatedBook.status === "to-read") {
      updatedBook.status = "reading";
    }

    selectedBook = updatedBook;
    books = books.map(b => b.id === updatedBook.id ? updatedBook : b);
  }

  // -- EDIT MODE LOGIC --
  function toggleEditProgress() {
    isEditingProgress = true;
  }

  function finishEditing() {
    isEditingProgress = false;
  }

  // Action to auto-focus the input when it appears
  function focusInput(node: HTMLElement) {
    node.focus();
  }

  // Blocks negative signs and 'e'
  function blockInvalidChars(e: KeyboardEvent) {
    if (e.key === '-' || e.key === 'e' || e.key === '+') {
      e.preventDefault();
    }
    if (e.key === 'Enter') {
      finishEditing();
    }
  }

  // Handles typing numbers directly
  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    let val = parseInt(target.value);

    if (isNaN(val)) val = 0;
    if (val < 0) val = 0; // Double safety

    // Cap at max pages
    if (selectedBook && selectedBook.total_pages > 0 && val > selectedBook.total_pages) {
      val = selectedBook.total_pages;
      target.value = val.toString(); // Update input visual to reflect cap
    }

    applyProgressChange(val);
  }

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

  function getProgressPercent(book: Book): number {
    if (!book.total_pages || book.total_pages === 0) return 0;
    return Math.min(100, Math.max(0, (book.pages_read / book.total_pages) * 100));
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

        <!-- SORT DROPDOWN -->
        <div class="sort-controls">
          <div class="custom-select-wrapper">
            <button class="custom-select-trigger" on:click|stopPropagation={toggleSortMenu}>
              {sortOption}
              <svg class="chevron" class:rotated={showSortMenu} width="10" height="6" viewBox="0 0 10 6" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <path d="M1 1L5 5L9 1" />
              </svg>
            </button>

            {#if showSortMenu}
              <div class="custom-options" transition:fade={{ duration: 100 }}>
                {#each ['Newest', 'Oldest', 'A-Z'] as option}
                  <button
                    class="custom-option"
                    class:selected={sortOption === option}
                    on:click={() => selectSort(option)}
                  >
                    {option}
                  </button>
                {/each}
              </div>
            {/if}
          </div>

          {#if showSortMenu}
            <div
              class="menu-overlay"
              on:click={closeMenu}
              role="button"
              tabindex="0"
              on:keydown={closeMenu}
            ></div>
          {/if}
        </div>
      </div>
    </div>

    <!-- FILTER TABS -->
    <div class="filters">
      {#each ['All', 'Reading', 'To Read', 'Finished'] as filter}
        <button
          class="filter-pill"
          class:active={activeFilter === filter}
          on:click={() => setFilter(filter)}
        >
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
        <div
          class="cover"
          style="background: linear-gradient(135deg, {book.cover_color || '#FF9A9E'} 0%, white 200%);"
        >
          {#if coverSrc(book)}
            <img
              src={coverSrc(book)}
              alt={`Cover of ${book.title}`}
              class="cover-img"
              loading="lazy"
            />
          {/if}
          
          {#if book.total_pages > 0}
            <div class="mini-progress-bar">
                <div class="mini-progress-fill" style="width: {getProgressPercent(book)}%"></div>
            </div>
          {/if}

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
    <div
      class="modal-card"
      role="dialog"
      aria-modal="true"
      transition:scale={{ duration: 300, easing: cubicOut, start: 0.95 }}
    >
      <div
        class="modal-cover"
        style="background: linear-gradient(135deg, {selectedBook.cover_color || '#FF9A9E'} 0%, white 200%);"
      >
        {#if coverSrc(selectedBook)}
          <img
            src={coverSrc(selectedBook)}
            alt={`Cover of ${selectedBook.title}`}
            class="modal-cover-img"
          />
        {/if}
        <span class="status-badge {selectedBook.status.toLowerCase().replace(' ', '-')}"
          >{selectedBook.status}</span
        >
      </div>

      <div class="modal-content">
        <div class="modal-header">
          <h3>{selectedBook.title}</h3>
          <p class="modal-author">{selectedBook.author}</p>
          
          <!-- PROGRESS CONTROL -->
          <div class="progress-section">
             <div class="progress-info">
                 <span class="progress-label">Progress</span>
                 <div class="progress-edit-wrapper">
                    {#if isEditingProgress}
                      <input 
                        type="number" 
                        class="progress-input"
                        value={selectedBook.pages_read}
                        use:focusInput
                        on:input={handleInput}
                        on:keydown={blockInvalidChars}
                        on:blur={finishEditing}
                      />
                    {:else}
                      <button class="progress-text clickable" on:click={toggleEditProgress} title="Click to edit">
                        {selectedBook.pages_read}
                      </button>
                    {/if}
                    <span class="progress-total"> / {selectedBook.total_pages} pages</span>
                 </div>
             </div>
             
             <!-- Visual Bar -->
             <div class="modal-progress-track">
                 <div class="modal-progress-fill" style="width: {getProgressPercent(selectedBook)}%"></div>
             </div>

             <!-- Controls -->
             <div class="progress-controls">
                 <button class="prog-btn" on:click={() => updateProgress(-10)}>-10</button>
                 <button class="prog-btn" on:click={() => updateProgress(-1)}>-</button>
                 <button class="prog-btn" on:click={() => updateProgress(1)}>+</button>
                 <button class="prog-btn" on:click={() => updateProgress(10)}>+10</button>
             </div>
          </div>
        </div>

        <div class="status-actions">
          <p class="label">Change Status</p>
          <div class="action-buttons">
            {#each ['Reading', 'To Read', 'Finished'] as status}
              <button
                class="status-btn {status.toLowerCase().replace(' ', '-')}"
                class:active={selectedBook.status === status.toLowerCase().replace(' ', '-')}
                on:click={() => updateStatus(status)}
              >
                {status}
              </button>
            {/each}
          </div>
        </div>

        <div class="modal-footer">
          <button class="delete-btn" on:click={confirmDelete} title="Delete Book">
            <svg
              width="19"
              height="19"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <polyline points="3 6 5 6 21 6"></polyline>
              <path
                d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
              ></path>
            </svg>
          </button>
          <button class="close-btn" on:click={closeBook}>Close</button>
        </div>
      </div>

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
  /* Base Layout */
  .library-content { transition: filter 0.3s ease; height: 100%; width: 100%; padding-bottom: 40px; animation: fadeIn 0.6s ease 0.6s forwards; opacity: 0; }
  .library-content.blurred { filter: blur(8px); pointer-events: none; }

  .lib-header { display: flex; flex-direction: column; gap: 16px; margin-bottom: 25px; align-items: flex-start; }
  .title-group { display: flex; justify-content: space-between; width: 100%; align-items: center; }
  h2 { margin: 0; font-family: 'Playfair Display', serif; font-size: 2.5rem; font-weight: 600; color: #4a3b3b; }
  .header-controls { display: flex; gap: 12px; align-items: center; }

  /* Search & Sort */
  .search-wrapper { position: relative; display: flex; align-items: center; }
  .search-icon { position: absolute; left: 12px; color: rgba(94, 75, 75, 0.4); pointer-events: none; }
  .search-wrapper input {
    background: rgba(255, 255, 255, 0.4);
    border: 1px solid rgba(255, 255, 255, 0.6);
    border-radius: 12px;
    padding: 6px 12px 6px 36px;
    font-size: 0.85rem;
    color: #5e4b4b;
    font-family: 'Inter', sans-serif;
    width: 180px;
    outline: none;
    transition: all 0.2s;
  }
  .search-wrapper input:focus { background: rgba(255, 255, 255, 0.6); border-color: rgba(255, 255, 255, 0.9); width: 220px; }
  .search-wrapper input::placeholder { color: rgba(94, 75, 75, 0.4); }

  .sort-controls { display: flex; align-items: center; gap: 8px; font-family: 'Inter', sans-serif; position: relative; z-index: 50; }
  .custom-select-wrapper { position: relative; }
  .custom-select-trigger { background: rgba(255, 255, 255, 0.4); border: 1px solid rgba(255, 255, 255, 0.6); border-radius: 12px; padding: 6px 12px; font-size: 0.85rem; color: #5e4b4b; cursor: pointer; display: flex; align-items: center; gap: 8px; font-weight: 600; font-family: 'Inter', sans-serif; transition: all 0.2s; min-width: 90px; justify-content: space-between; }
  .custom-select-trigger:hover { background: rgba(255, 255, 255, 0.6); }
  .custom-options {
    position: absolute;
    top: calc(100% + 6px);
    right: 0;
    left: 0;
    background: #fff6f0;
    border: 1px solid rgba(255, 255, 255, 0.8);
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(94, 75, 75, 0.1);
    padding: 4px;
    display: flex;
    flex-direction: column;
    min-width: 100px;
    backdrop-filter: blur(10px);
  }
  .custom-option {
    background: transparent;
    border: none;
    text-align: left;
    padding: 8px 12px;
    font-size: 0.85rem;
    color: rgba(94, 75, 75, 0.8);
    cursor: pointer;
    border-radius: 8px;
    transition: all 0.15s;
    font-weight: 500;
  }
  .custom-option:hover { background: rgba(94, 75, 75, 0.08); color: #5e4b4b; }
  .custom-option.selected { background: rgba(94, 75, 75, 0.15); color: #5e4b4b; font-weight: 600; }
  .menu-overlay { position: fixed; inset: 0; z-index: -1; cursor: default; }

  /* Filters */
  .filters { display: flex; gap: 8px; }
  .filter-pill {
    background: rgba(255, 255, 255, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.5);
    padding: 6px 16px;
    border-radius: 20px;
    font-size: 0.85rem;
    color: #6b5b5b;
    cursor: pointer;
    transition: all 0.2s;
  }
  .filter-pill.active { background: #5e4b4b; color: white; border-color: transparent; }

  /* Book Grid */
  .book-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 24px;
    padding-right: 10px;
  }
  .book-card { display: flex; flex-direction: column; gap: 12px; cursor: pointer; transition: transform 0.2s; }
  .book-card:hover { transform: translateY(-8px); }
  
  .cover {
    aspect-ratio: 2 / 3;
    width: 100%;
    border-radius: 12px;
    position: relative;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.05);
    overflow: hidden;
  }
  .cover-img { width: 100%; height: 100%; object-fit: cover; display: block; position: relative; z-index: 1; }
  
  .status-dot {
    position: absolute;
    top: 10px;
    right: 10px;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    z-index: 2;
  }
  .status-dot.reading { background: #47f386; }
  .status-dot.finished { background: #529ffd; }
  .status-dot.to-read { background: #ff4eaf; }
  
  .mini-progress-bar {
      position: absolute;
      bottom: 0;
      left: 0;
      right: 0;
      height: 4px;
      background: rgba(0,0,0,0.2);
      z-index: 3;
  }
  .mini-progress-fill {
      height: 100%;
      background: rgba(255,255,255,0.9);
      transition: width 0.3s ease;
  }

  .info { padding: 0 4px; }
  .title { font-family: 'Playfair Display', serif; font-size: 1.1rem; font-weight: 600; color: #2c1810; margin: 0 0 4px 0; }
  .author { font-size: 0.75rem; font-weight: 500; color: rgba(94, 75, 75, 0.65); text-transform: uppercase; }
  .empty-state { grid-column: 1 / -1; text-align: center; padding: 40px; color: rgba(94, 75, 75, 0.5); font-style: italic; }

  /* Modal */
  .modal-overlay { position: fixed; inset: 0; background: rgba(255, 240, 230, 0.4); display: flex; justify-content: center; align-items: center; z-index: 100; backdrop-filter: blur(4px); }
  .modal-card { position: relative; background: #fbddc8; backdrop-filter: blur(20px); width: 600px; height: 380px; border-radius: 24px; box-shadow: 0 20px 60px rgba(94, 75, 75, 0.15); display: flex; overflow: hidden; border: 1px solid rgba(255, 255, 255, 0.8); }
  
  .modal-cover { width: 240px; height: 100%; position: relative; overflow: hidden; }
  .modal-cover-img { width: 100%; height: 100%; object-fit: cover; display: block; }
  
  .status-badge { position: absolute; top: 16px; left: 16px; padding: 6px 12px; border-radius: 20px; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; background: white; box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1); }
  .status-badge.reading { color: #47f386; }
  .status-badge.finished { color: #529ffd; }
  .status-badge.to-read { color: #ff4eaf; }

  .modal-content { flex: 1; padding: 32px; display: flex; flex-direction: column; justify-content: space-between; }
  .modal-header h3 { font-family: 'Playfair Display', serif; font-size: 2rem; margin: 0 0 8px 0; color: #2c1810; line-height: 1.1; }
  .modal-author { font-size: 0.9rem; color: rgba(94, 75, 75, 0.7); text-transform: uppercase; margin: 0; }
  
  /* PROGRESS SECTION */
  .progress-section { margin-top: 20px; }
  .progress-info { display: flex; justify-content: space-between; font-size: 0.8rem; color: #5e4b4b; margin-bottom: 6px; align-items: center; }
  .progress-label { text-transform: uppercase; font-weight: 600; opacity: 0.6; }
  
  .progress-edit-wrapper { display: flex; align-items: baseline; gap: 4px; font-weight: 600; }
  .progress-text.clickable { 
      background: none; 
      border: none; 
      padding: 0; 
      font-size: 0.8rem; 
      font-weight: 700; 
      color: #5e4b4b; 
      cursor: pointer; 
      border-bottom: 1px dashed rgba(94,75,75,0.4); 
      transition: color 0.2s;
  }
  .progress-text.clickable:hover { color: #2c1810; border-bottom-color: #2c1810; }
  
  .progress-input {
      background: transparent;
      border: none;
      border-bottom: 2px solid #5e4b4b;
      width: 40px;
      font-family: inherit;
      font-size: 0.8rem;
      font-weight: 700;
      color: #2c1810;
      text-align: right;
      padding: 0;
      outline: none;
      -moz-appearance: textfield; /* Firefox: remove spinner */
      appearance: textfield;
  }
  /* Remove spinners in Chrome/Safari/Edge */
  .progress-input::-webkit-outer-spin-button,
  .progress-input::-webkit-inner-spin-button {
      -webkit-appearance: none;
      margin: 0;
  }
  
  .modal-progress-track {
      height: 8px;
      width: 100%;
      background: rgba(255,255,255,0.4);
      border-radius: 4px;
      overflow: hidden;
      margin-bottom: 8px;
  }
  .modal-progress-fill {
      height: 100%;
      background: #5e4b4b;
      transition: width 0.3s ease;
  }
  .progress-controls { display: flex; gap: 6px; }
  .prog-btn {
      background: rgba(255,255,255,0.4);
      border: 1px solid rgba(255,255,255,0.6);
      border-radius: 6px;
      padding: 4px 10px;
      font-size: 0.75rem;
      color: #5e4b4b;
      cursor: pointer;
      font-weight: 600;
      transition: all 0.2s;
  }
  .prog-btn:hover { background: rgba(255,255,255,0.7); transform: translateY(-1px); }

  .label { font-size: 0.8rem; font-weight: 600; color: rgba(94, 75, 75, 0.5); text-transform: uppercase; margin-bottom: 12px; }
  .action-buttons { display: flex; gap: 10px; }
  .status-btn { flex: 1; padding: 12px; border: none; border-radius: 12px; font-size: 0.85rem; font-weight: 600; cursor: pointer; transition: all 0.2s ease; background: #5e4b4b; color: rgba(255, 255, 255, 0.9); opacity: 0.9; }
  .status-btn:hover { opacity: 1; transform: translateY(-1px); }
  .status-btn.reading:hover, .status-btn.reading.active { background: #47f386; color: white; box-shadow: 0 4px 12px rgba(71, 243, 134, 0.3); }
  .status-btn.to-read:hover, .status-btn.to-read.active { background: #ff4eaf; color: white; box-shadow: 0 4px 12px rgba(255, 78, 175, 0.3); }
  .status-btn.finished:hover, .status-btn.finished.active { background: #529ffd; color: white; box-shadow: 0 4px 12px rgba(82, 159, 253, 0.3); }

  .modal-footer { display: flex; justify-content: space-between; align-items: center; width: 100%; }
  .delete-btn { background: transparent; border: none; color: rgba(255, 98, 98, 0.7); cursor: pointer; transition: all 0.2s; padding: 8px; border-radius: 8px; display: flex; align-items: center; justify-content: center; }
  .delete-btn:hover { background: rgba(200, 50, 50, 0.1); color: rgba(200, 50, 50, 0.8); transform: scale(1.09); }
  .close-btn { background: transparent; border: 1.5px solid #5e4b4b; padding: 8px 24px; border-radius: 24px; color: #5e4b4b; font-weight: 600; cursor: pointer; font-size: 0.85rem; transition: all 0.2s; opacity: 0.8; }
  .close-btn:hover { background: #5e4b4b; color: white; opacity: 1; }

  /* Delete Confirm Styles */
  .delete-confirm-overlay { position: absolute; inset: 0; background: rgba(255, 234, 219, 0.685); backdrop-filter: blur(5px); display: flex; justify-content: center; align-items: center; z-index: 50; border-radius: 24px; }
  .delete-confirm-card { background: #ffd3d3ce; padding: 24px; border-radius: 16px; box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1); text-align: center; border: 1px solid rgba(200, 50, 50, 0.1); width: 80%; }
  .delete-confirm-card h4 { margin: 0 0 8px 0; color: #4a3b3b; font-size: 1.1rem; }
  .delete-confirm-card p { margin: 0 0 20px 0; font-size: 0.85rem; color: rgba(94, 75, 75, 0.6); }
  .confirm-actions { display: flex; gap: 12px; justify-content: center; }
  .cancel-btn { background: rgba(94, 75, 75, 0.1); color: #5e4b4b; border: none; padding: 8px 16px; border-radius: 8px; font-size: 0.8rem; font-weight: 600; cursor: pointer; }
  .confirm-delete-btn { background: #ff4e4e; color: white; border: none; padding: 8px 16px; border-radius: 8px; font-size: 0.8rem; font-weight: 600; cursor: pointer; box-shadow: 0 2px 8px rgba(255, 78, 78, 0.3); }

  @keyframes fadeIn { to { opacity: 1; } }
</style>
