<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { fade, scale } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import { createEventDispatcher } from 'svelte';

  /* 
     ================================================================
     SECTION 1: TYPES & PROPS
     ================================================================
  */
  // {
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

    export let book: Book;

    const dispatch = createEventDispatcher();

    let isEditingProgress = false;
    let showDeleteConfirm = false;

    $: isProgressDisabled = book.total_pages === 0;
  // }


  /* 
     ================================================================
     SECTION 2: HELPERS (Formatting & Visuals)
     ================================================================
  */
  // {
    function titleSize(title: string): string {
      const len = title.length;
      if (len > 60) return "1.15rem";
      if (len > 35) return "1.4rem";
      return "2rem";
    }

    function authorSize(author: string): string {
      return author.length > 40 ? "0.75rem" : "0.9rem";
    }

    function coverSrc(cover: string): string {
      if (!cover) return "";
      if (cover.startsWith("http://") || cover.startsWith("https://")) {
        return cover;
      }
      return convertFileSrc(cover);
    }

    function getProgressPercent(b: Book): number {
      if (!b.total_pages || b.total_pages === 0) return 0;
      return Math.min(100, Math.max(0, (b.pages_read / b.total_pages) * 100));
    }
  // }


  /* 
     ================================================================
     SECTION 3: LOGIC & HANDLERS
     ================================================================
  */
  // {
    function emitUpdate(changes: Partial<Book>) {
      dispatch('update', { ...book, ...changes });
    }

    function updateStatus(newStatus: string) {
      const s = newStatus.toLowerCase().replace(" ", "-");
      emitUpdate({ status: s });
    }

    function updateProgress(delta: number) {
      if (isProgressDisabled) return;
      let newPages = book.pages_read + delta;
      
      if (newPages < 0) newPages = 0;
      if (book.total_pages > 0 && newPages > book.total_pages) {
        newPages = book.total_pages;
      }

      // Auto-update status logic
      let newStatus = book.status;
      if (book.total_pages > 0 && newPages >= book.total_pages) {
        newStatus = "finished";
      } else if (newPages > 0 && book.status === "to-read") {
        newStatus = "reading";
      }

      emitUpdate({ pages_read: newPages, status: newStatus });
    }

    function handleInput(e: Event) {
      if (isProgressDisabled) return;
      const target = e.target as HTMLInputElement;
      let val = parseInt(target.value);

      if (isNaN(val)) val = 0;
      if (val < 0) val = 0;

      if (book.total_pages > 0 && val > book.total_pages) {
        val = book.total_pages;
        target.value = val.toString();
      }

      let newStatus = book.status;
      if (book.total_pages > 0 && val >= book.total_pages) {
        newStatus = "finished";
      } else if (val > 0 && book.status === "to-read") {
        newStatus = "reading";
      }
      
      emitUpdate({ pages_read: val, status: newStatus });
    }

    function toggleEditProgress() { 
      if (!isProgressDisabled) isEditingProgress = true; 
    }
    
    function finishEditing() { isEditingProgress = false; }
    function focusInput(node: HTMLElement) { node.focus(); }

    function blockInvalidChars(e: KeyboardEvent) {
      if (['-', 'e', '+'].includes(e.key)) e.preventDefault();
      if (e.key === 'Enter') finishEditing();
    }

    function handleDelete() { dispatch('delete', book.id); }
    function close() { dispatch('close'); }
  // }
</script>

<div
  class="modal-overlay"
  on:click|self={close}
  on:keydown={(e) => e.key === 'Escape' && close()}
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
    
    <!-- LEFT SIDE: COVER -->
    <div
      class="modal-cover"
      style="background: linear-gradient(135deg, {book.cover_color || '#FF9A9E'} 0%, white 200%);"
    >
      {#if coverSrc(book.cover)}
        <img
          src={coverSrc(book.cover)}
          alt={`Cover of ${book.title}`}
          class="modal-cover-img"
        />
      {/if}
      <span class="status-badge {book.status.toLowerCase().replace(' ', '-')}">
        {book.status}
      </span>
    </div>

    <!-- RIGHT SIDE: CONTENT -->
    <div class="modal-content">
      <div class="modal-header">
        <h3 style="font-size: {titleSize(book.title)}">{book.title}</h3>
        <p class="modal-author" style="font-size: {authorSize(book.author)}">{book.author}</p>
        
        <!-- PROGRESS CONTROL -->
        <div class="progress-section">
           <div class="progress-info">
               <span class="progress-label">Progress</span>
               
               {#if isEditingProgress}
                 <!-- EDIT MODE -->
                 <div class="progress-edit-wrapper" role="group" aria-label="Edit pages read">
                   <input 
                     type="number" 
                     class="progress-input"
                     value={book.pages_read}
                     use:focusInput
                     on:input={handleInput}
                     on:keydown={blockInvalidChars}
                     on:blur={finishEditing}
                     disabled={isProgressDisabled}
                   />
                   <span class="progress-total"> / {book.total_pages} pages</span>
                 </div>
               {:else}
                 <!-- VIEW MODE -->
                 <button 
                   class="progress-edit-wrapper" 
                   on:click={toggleEditProgress}
                   disabled={isProgressDisabled}
                   aria-label="Change pages read"
                 >
                   <span class="progress-text clickable" title="Click to edit">
                     {book.pages_read}
                   </span>
                   <span class="progress-total"> / {book.total_pages} pages</span>
                 </button>
               {/if}
           </div>
           
           <div class="modal-progress-track">
               <div class="modal-progress-fill" style="width: {getProgressPercent(book)}%"></div>
           </div>

           <div class="progress-controls">
               <button class="prog-btn" on:click={() => updateProgress(-10)} disabled={isProgressDisabled}>-10</button>
               <button class="prog-btn" on:click={() => updateProgress(-1)} disabled={isProgressDisabled}>-</button>
               <button class="prog-btn" on:click={() => updateProgress(1)} disabled={isProgressDisabled}>+</button>
               <button class="prog-btn" on:click={() => updateProgress(10)} disabled={isProgressDisabled}>+10</button>
           </div>
        </div>
      </div>

      <div class="status-actions">
        <p class="label">Change Status</p>
        <div class="action-buttons">
          {#each ['Reading', 'To Read', 'Finished'] as status}
            <button
              class="status-btn {status.toLowerCase().replace(' ', '-')}"
              class:active={book.status === status.toLowerCase().replace(' ', '-')}
              on:click={() => updateStatus(status)}
            >
              {status}
            </button>
          {/each}
        </div>
      </div>

      <div class="modal-footer">
        <button class="delete-btn" on:click={() => showDeleteConfirm = true} title="Delete Book">
          <svg width="19" height="19" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="3 6 5 6 21 6"></polyline>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
          </svg>
        </button>
        <button class="close-btn" on:click={close}>Close</button>
      </div>
    </div>

    {#if showDeleteConfirm}
      <div class="delete-confirm-overlay" transition:fade={{ duration: 150 }}>
        <div class="delete-confirm-card" transition:scale={{ duration: 200, start: 0.9 }}>
          <h4>Delete this book?</h4>
          <p>This action cannot be undone.</p>
          <div class="confirm-actions">
            <button class="cancel-btn" on:click={() => showDeleteConfirm = false}>Cancel</button>
            <button class="confirm-delete-btn" on:click={handleDelete}>Delete</button>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  /* #region --- MODAL SHELL --- */
  .modal-overlay { 
    position: fixed; inset: 0; background: rgba(255, 240, 230, 0.4); 
    display: flex; justify-content: center; align-items: center; 
    z-index: 100; backdrop-filter: blur(4px); 
  }
  
  .modal-card { 
    position: relative; background: #fbddc8; backdrop-filter: blur(20px); 
    width: 600px; height: 380px; border-radius: 24px; 
    box-shadow: 0 20px 60px rgba(94, 75, 75, 0.15); 
    display: flex; overflow: hidden; border: 1px solid rgba(255, 255, 255, 0.8); 
  }
  /* #endregion */

  /* #region --- COVER SIDE --- */
  .modal-cover { width: 240px; height: 100%; position: relative; overflow: hidden; }
  .modal-cover-img { width: 100%; height: 100%; object-fit: cover; display: block; }
  
  .status-badge { 
    position: absolute; top: 16px; left: 16px; padding: 6px 12px; 
    border-radius: 20px; font-size: 0.75rem; font-weight: 600; 
    text-transform: uppercase; background: white; 
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1); 
  }
  .status-badge.reading { color: #47f386; }
  .status-badge.finished { color: #529ffd; }
  .status-badge.to-read { color: #ff4eaf; }
  /* #endregion */

  /* #region --- CONTENT SIDE --- */
  .modal-content { flex: 1; padding: 32px; display: flex; flex-direction: column; justify-content: space-between; }
  
  .modal-header h3 { 
    font-family: 'Playfair Display', serif; font-size: 2rem; 
    margin: 0 0 8px 0; color: #2c1810; line-height: 1.1; 
    display: -webkit-box; -webkit-line-clamp: 3; line-clamp: 3; 
    -webkit-box-orient: vertical; overflow: hidden;
  }
  
  .modal-author { 
    font-size: 0.9rem; color: rgba(94, 75, 75, 0.7); 
    text-transform: uppercase; margin: 0; 
  }
  
  /* Progress Section */
  .progress-section { margin-top: 20px; }
  .progress-info { 
    display: flex; justify-content: space-between; font-size: 0.8rem; 
    color: #5e4b4b; margin-bottom: 6px; align-items: center; 
  }
  .progress-label { text-transform: uppercase; font-weight: 600; opacity: 0.6; }
  
  /* Edit/View Pill */
  .progress-edit-wrapper { 
    display: flex; align-items: center; gap: 8px; font-weight: 600; 
    cursor: pointer; background: rgba(255, 255, 255, 0.4); 
    padding: 4px 10px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.6); 
    transition: all 0.2s ease; font-family: inherit; font-size: inherit; color: inherit;
  }
  .progress-edit-wrapper:hover { 
    background: rgba(255, 255, 255, 0.6); border-color: rgba(255, 255, 255, 0.9); 
    transform: translateY(-1px); box-shadow: 0 2px 6px rgba(94, 75, 75, 0.05); 
  }

  .progress-text.clickable { 
    background: none; border: none; padding: 0; font-size: 0.9rem; 
    font-weight: 700; color: #2c1810; cursor: pointer; transition: color 0.2s;
  }
  
  /* Input Field */
  .progress-input {
    background: rgba(255, 255, 255, 0.5); border: 1px solid rgba(94, 75, 75, 0.3); 
    border-radius: 8px; width: 50px; font-family: inherit; font-size: 0.9rem; 
    font-weight: 700; color: #4a3b3b; text-align: center; padding: 2px 4px; 
    outline: none; -moz-appearance: textfield; appearance: textfield; 
    box-shadow: inset 0 1px 4px rgba(94, 75, 75, 0.1); transition: all 0.2s ease;
  }
  .progress-input:focus { 
    background: rgba(255, 255, 255, 0.8); border-color: #5e4b4b; 
    box-shadow: inset 0 1px 4px rgba(94, 75, 75, 0.05), 0 0 0 2px rgba(94, 75, 75, 0.1); 
  }
  .progress-input::-webkit-outer-spin-button, .progress-input::-webkit-inner-spin-button { -webkit-appearance: none; margin: 0; }
  
  .progress-total { font-size: 0.85rem; color: rgba(94, 75, 75, 0.6); font-weight: 500; }

  /* Bars & Controls */
  .modal-progress-track { 
    height: 8px; width: 100%; background: rgba(255,255,255,0.4); 
    border-radius: 4px; overflow: hidden; margin-bottom: 8px; 
  }
  .modal-progress-fill { height: 100%; background: #5e4b4b; transition: width 0.3s ease; }
  
  .progress-controls { display: flex; gap: 6px; }
  .prog-btn { 
    background: rgba(255,255,255,0.4); border: 1px solid rgba(255,255,255,0.6); 
    border-radius: 6px; padding: 4px 10px; font-size: 0.75rem; color: #5e4b4b; 
    cursor: pointer; font-weight: 600; transition: all 0.2s; 
  }
  .prog-btn:hover { background: rgba(255,255,255,0.7); transform: translateY(-1px); }

  /* Actions */
  .label { 
    font-size: 0.8rem; font-weight: 600; color: rgba(94, 75, 75, 0.5); 
    text-transform: uppercase; margin-bottom: 12px; 
  }
  .action-buttons { display: flex; gap: 10px; }
  .status-btn { 
    flex: 1; padding: 12px; border: none; border-radius: 12px; 
    font-size: 0.85rem; font-weight: 600; cursor: pointer; transition: all 0.2s ease; 
    background: #5e4b4b; color: rgba(255, 255, 255, 0.9); opacity: 0.9; 
  }
  .status-btn:hover { opacity: 1; transform: translateY(-1px); }
  .status-btn.reading:hover, .status-btn.reading.active { background: #47f386; color: white; box-shadow: 0 4px 12px rgba(71, 243, 134, 0.3); }
  .status-btn.to-read:hover, .status-btn.to-read.active { background: #ff4eaf; color: white; box-shadow: 0 4px 12px rgba(255, 78, 175, 0.3); }
  .status-btn.finished:hover, .status-btn.finished.active { background: #529ffd; color: white; box-shadow: 0 4px 12px rgba(82, 159, 253, 0.3); }

  /* Footer */
  .modal-footer { display: flex; justify-content: space-between; align-items: center; width: 100%; }
  .delete-btn { 
    background: transparent; border: none; color: rgba(255, 98, 98, 0.7); 
    cursor: pointer; transition: all 0.2s; padding: 8px; border-radius: 8px; 
    display: flex; align-items: center; justify-content: center; 
  }
  .delete-btn:hover { background: rgba(200, 50, 50, 0.1); color: rgba(200, 50, 50, 0.8); transform: scale(1.09); }
  .close-btn { 
    background: transparent; border: 1.5px solid #5e4b4b; padding: 8px 24px; 
    border-radius: 24px; color: #5e4b4b; font-weight: 600; cursor: pointer; 
    font-size: 0.85rem; transition: all 0.2s; opacity: 0.8; 
  }
  .close-btn:hover { background: #5e4b4b; color: white; opacity: 1; }
  /* #endregion */

  /* #region --- DELETE CONFIRM --- */
  .delete-confirm-overlay { 
    position: absolute; inset: 0; background: rgba(255, 234, 219, 0.685); 
    backdrop-filter: blur(5px); display: flex; justify-content: center; 
    align-items: center; z-index: 50; border-radius: 24px; 
  }
  .delete-confirm-card { 
    background: #ffd3d3ce; padding: 24px; border-radius: 16px; 
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1); text-align: center; 
    border: 1px solid rgba(200, 50, 50, 0.1); width: 80%; 
  }
  .delete-confirm-card h4 { margin: 0 0 8px 0; color: #4a3b3b; font-size: 1.1rem; }
  .confirm-actions { display: flex; gap: 12px; justify-content: center; }
  .cancel-btn { 
    background: rgba(94, 75, 75, 0.1); color: #5e4b4b; border: none; 
    padding: 8px 16px; border-radius: 8px; font-size: 0.8rem; font-weight: 600; cursor: pointer; 
  }
  .confirm-delete-btn { 
    background: #ff4e4e; color: white; border: none; padding: 8px 16px; 
    border-radius: 8px; font-size: 0.8rem; font-weight: 600; cursor: pointer; 
    box-shadow: 0 2px 8px rgba(255, 78, 78, 0.3); 
  }
  /* #endregion */
</style>
