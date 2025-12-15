<script lang="ts">
  import { fade, scale } from 'svelte/transition';
  import { createEventDispatcher } from 'svelte';

  export let book: { 
    title: string; 
    author: string; 
    summary: string; 
    fullSummary?: string 
  };

  const dispatch = createEventDispatcher();

  function close() {
    dispatch('close');
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      close();
    } else if (event.key === "Escape") {
      close();
    }
  }
</script>

<div
  class="summary-overlay"
  role="button"
  tabindex="0"
  aria-label="Close description"
  on:click|self={close}
  on:keydown={handleKeydown}
  transition:fade={{ duration: 200 }}
>
  <div class="summary-dialog" transition:scale={{ duration: 300, start: 0.95 }}>
    <div class="summary-header">
      <h3>{book.title}</h3>
      <p class="summary-author">{book.author}</p>
    </div>
    
    <div class="summary-body">
      <p>{book.fullSummary ?? book.summary}</p>
    </div>
    
    <div class="summary-actions">
      <button class="pill-btn summary-close" on:click={close}>
        Close
      </button>
    </div>
  </div>
</div>

<style>
  /* Overlay */
  .summary-overlay {
    position: fixed;
    inset: 0;
    z-index: 900;
    background: rgba(34, 22, 19, 0.35);
    backdrop-filter: blur(14px);
    -webkit-backdrop-filter: blur(14px);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  /* Dialog Box */
  .summary-dialog {
    max-width: 620px;
    max-height: 70vh;
    width: 90%;
    background: rgba(255, 255, 255, 0.78);
    border-radius: 28px;
    padding: 24px 26px 20px;
    box-shadow:
      0 18px 50px rgba(0, 0, 0, 0.18),
      inset 0 0 18px rgba(255, 255, 255, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.9);
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  /* Header */
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

  /* Body (Scrollable) */
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

  /* Actions */
  .summary-actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 8px;
  }

  /* Button Style */
  .pill-btn {
    min-width: 90px;
    padding: 8px 20px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.7);
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    
    background: rgba(255, 255, 255, 0.4);
    color: #5b3b30;
    
    transition:
      background 0.18s ease,
      transform 0.12s ease,
      box-shadow 0.18s ease,
      color 0.2s ease;
  }

  .pill-btn:hover {
    background: rgba(255, 255, 255, 0.6);
    transform: translateY(-1px);
  }
</style>
