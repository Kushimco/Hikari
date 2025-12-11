<script lang="ts">
  import { fade } from 'svelte/transition';
  import { createEventDispatcher } from 'svelte';

  // Props
  export let book: { 
    title: string; 
    author: string; 
    coverUrl?: string | null; 
    pages?: string 
  };

  const dispatch = createEventDispatcher();

  // Local State
  let statusInput: "to-read" | "reading" | "finished" = "reading";
  let pagesReadInput = "0";
  let totalPagesInput = "0";

  // Initialize total pages from book data
  $: if (book) {
    const p = parseInt(book.pages || "0", 10);
    totalPagesInput = isNaN(p) ? "0" : String(p);
  }

  // Auto-fill pages if status is 'finished'
  $: if (statusInput === "finished") {
    pagesReadInput = totalPagesInput;
  }

  function increment() {
    if (statusInput === "finished") return;
    const current = parseInt(pagesReadInput || "0");
    const total = parseInt(totalPagesInput || "0");
    
    // Don't exceed total if total is set
    if (total > 0 && current >= total) {
       pagesReadInput = String(total);
       return;
    }
    pagesReadInput = String(current + 1);
  }

  function decrement() {
    if (statusInput === "finished") return;
    const current = parseInt(pagesReadInput || "0");
    pagesReadInput = String(Math.max(0, current - 1));
  }
  
  function handlePageInput() {
      const current = parseInt(pagesReadInput);
      const total = parseInt(totalPagesInput);
      if (!isNaN(total) && total > 0 && current > total) {
          pagesReadInput = String(total);
      }
  }

  function handleSave() {
    dispatch('save', {
      status: statusInput,
      pagesRead: parseInt(pagesReadInput || "0"),
      totalPages: parseInt(totalPagesInput || "0"),
      book // Pass back the original book data for reference
    });
  }
</script>

<div class="add-overlay" transition:fade={{ duration: 250 }}>
  <div class="add-dialog">
    <div class="add-header">
      <h3>ADD TO LIBRARY</h3>
      <p>{book.title}</p>
      <span class="add-author">{book.author}</span>
    </div>

    <div class="add-body">
      <!-- STATUS SELECTOR -->
      <label class="field">
        <span>Status</span>
        <div class="status-row">
          <button
            type="button"
            class="status-pill"
            class:status-pill-active={statusInput === "to-read"}
            on:click={() => (statusInput = "to-read")}
          >
            To read
          </button>
          <button
            type="button"
            class="status-pill"
            class:status-pill-active={statusInput === "reading"}
            on:click={() => (statusInput = "reading")}
          >
            Reading
          </button>
          <button
            type="button"
            class="status-pill"
            class:status-pill-active={statusInput === "finished"}
            on:click={() => (statusInput = "finished")}
          >
            Finished
          </button>
        </div>
      </label>

      <!-- PAGE COUNTER -->
      <label class="field field-center">
        <span>Pages read / {totalPagesInput}</span>
        <div class="pages-row" class:disabled-row={statusInput === "finished"}>
          <button
            type="button"
            class="pages-arrow pages-arrow-left"
            on:click={decrement}
            disabled={statusInput === "finished"}
            aria-label="Decrease pages read"
          >
            –
          </button>
          <input
            type="number"
            min="0"
            bind:value={pagesReadInput}
            on:input={handlePageInput}
            disabled={statusInput === "finished"}
            class="pages-input no-spin"
          />
          <button
            type="button"
            class="pages-arrow pages-arrow-right"
            on:click={increment}
            disabled={statusInput === "finished"}
            aria-label="Increase pages read"
          >
            +
          </button>
        </div>
      </label>
    </div>

    <!-- ACTIONS -->
    <div class="add-actions">
      <button
        type="button"
        class="pill-btn pill-secondary"
        on:click={() => dispatch('cancel')}
      >
        Cancel
      </button>
      <button
        type="button"
        class="pill-btn pill-primary"
        on:click={handleSave}
      >
        Save
      </button>
    </div>
  </div>
</div>

<style>
  /* Overlay & Dialog Container */
  .add-overlay {
    position: fixed;
    inset: 0;
    z-index: 950;
    display: flex;
    align-items: center;
    justify-content: center;
    background:
      radial-gradient(circle at top left, rgba(255, 190, 150, 0.4), transparent 55%),
      radial-gradient(circle at bottom right, rgba(255, 140, 180, 0.35), transparent 60%),
      rgba(255, 220, 200, 0.35);
    backdrop-filter: blur(22px) saturate(1.25);
    -webkit-backdrop-filter: blur(22px) saturate(1.25);
  }

  .add-dialog {
    width: 420px;
    max-width: 90vw;
    background: rgba(255, 255, 255, 0.68);
    border-radius: 26px;
    padding: 20px 22px 18px;
    box-shadow:
      0 20px 40px rgba(180, 110, 80, 0.35),
      inset 0 0 18px rgba(255, 255, 255, 0.9);
    border: 1px solid rgba(255, 245, 240, 0.95);
    display: flex;
    flex-direction: column;
    gap: 16px;
    transform-origin: center;
  }

  /* Header Styles */
  .add-header h3 {
    margin: 0;
    font-size: 0.95rem;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: rgba(131, 91, 74, 0.9);
    text-align: center;
  }

  .add-header p {
    margin: 4px 0 0;
    font-size: 1.05rem;
    font-weight: 600;
    color: #4b332e;
    text-align: center;
  }

  .add-author {
    font-size: 0.85rem;
    color: rgba(75, 51, 46, 0.7);
    text-align: center;
    display: block; /* Ensure it takes a line */
  }

  /* Body & Fields */
  .add-body {
    display: flex;
    flex-direction: column;
    gap: 14px;
    margin-top: 10px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-size: 0.85rem;
    color: rgba(75, 51, 46, 0.8);
  }

  .field-center {
    align-items: center;
    text-align: center;
  }

  .field-center > span {
    width: 100%;
    text-align: center;
  }

  /* Status Pills */
  .status-row {
    display: flex;
    gap: 8px;
  }

  .status-pill {
    flex: 1;
    padding: 7px 10px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.5);
    color: #5b3b30;
    font-size: 0.8rem;
    cursor: pointer;
    transition:
      background 0.18s ease,
      box-shadow 0.18s ease,
      transform 0.12s ease;
  }

  .status-pill-active {
    background: linear-gradient(135deg, #ffcf9f, #f8a3b0);
    box-shadow: 0 8px 18px rgba(200, 120, 90, 0.3);
    transform: translateY(-1px);
  }

  /* Pages Counter */
  .pages-row {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    transition: opacity 0.2s ease, filter 0.2s ease;
  }
  
  .pages-row.disabled-row {
    opacity: 0.5;
    filter: grayscale(0.5);
    pointer-events: none;
  }

  .pages-input {
    width: 90px;
    padding: 7px 10px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.55);
    box-shadow: inset 0 0 10px rgba(255, 255, 255, 0.9);
    color: #4b332e;
    font-size: 0.9rem;
    text-align: center;
    outline: none;
  }

  .no-spin {
    -moz-appearance: textfield;
    appearance: textfield;
  }
  .no-spin::-webkit-outer-spin-button,
  .no-spin::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .pages-arrow {
    width: 28px;
    height: 28px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.42);
    color: #5b3b30;
    font-size: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow:
      0 6px 14px rgba(200, 120, 90, 0.25),
      inset 0 0 8px rgba(255, 255, 255, 0.7);
    transition:
      background 0.15s ease,
      transform 0.12s ease,
      box-shadow 0.15s ease;
  }

  .pages-arrow:hover {
    background: rgba(255, 255, 255, 0.65);
    transform: translateY(-1px);
  }
  
  .pages-arrow:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }
  
  .pages-arrow-left, .pages-arrow-right {
      padding-bottom: 1px;
  }

  /* Action Buttons */
  .add-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 6px;
  }

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
    transition:
      background 0.18s ease,
      transform 0.12s ease,
      box-shadow 0.18s ease,
      color 0.2s ease;
  }

  .pill-secondary {
    background: rgba(255, 255, 255, 0.4);
    color: #5b3b30;
  }
  
  .pill-primary {
    background: rgba(255, 255, 255, 0.5); 
    color: #4b332e;
    box-shadow: 0 4px 12px rgba(200, 120, 90, 0.15);
  }

  .pill-primary:hover {
    background: rgba(255, 255, 255, 0.75);
    transform: translateY(-1px);
    box-shadow: 0 8px 20px rgba(200, 120, 90, 0.2);
  }

  .pill-primary:active {
    background: linear-gradient(135deg, #ffcf9f, #f8a3b0);
    color: #2c1810;
    transform: translateY(1px);
    box-shadow: 0 2px 8px rgba(200, 120, 90, 0.3);
  }

  .pill-secondary:hover {
    background: rgba(255, 255, 255, 0.6);
    transform: translateY(-1px);
  }
</style>
