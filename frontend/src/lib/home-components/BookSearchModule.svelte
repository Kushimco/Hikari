<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  type SearchState = "idle" | "loading" | "result";

  type MockBook = {
    title: string;
    author: string;
    year: string;
    pages: string;
    summary: string;
  };

  // bind:bookTitle in parent
  export let bookTitle = "";

  // current search state + found book (provided by Home.svelte)
  export let searchState: SearchState = "idle";
  export let foundBook: MockBook | null = null;

  // whether the "add" / "discard" animations are playing
  export let isAdding = false;
  export let isDiscarding = false;

  const dispatch = createEventDispatcher<{
    input: Event;
    keydown: KeyboardEvent;
    focus: FocusEvent;
    blur: FocusEvent;
    add: void;
    discard: void;
  }>();

  function handleInput(e: Event) {
    dispatch("input", e);
  }

  function handleKeydown(e: KeyboardEvent) {
    dispatch("keydown", e);
  }

  function handleFocus(e: FocusEvent) {
    dispatch("focus", e);
  }

  function handleBlur(e: FocusEvent) {
    dispatch("blur", e);
  }

  function handleAdd() {
    dispatch("add");
  }

  function handleDiscard() {
    dispatch("discard");
  }
</script>

{#if searchState === "result" && foundBook}
  <div
    class="book-result"
    class:book-result-adding={isAdding}
    class:book-result-discarding={isDiscarding}
  >
    <div class="book-card">
      <div class="book-cover"></div>
      <div class="book-info">
        <h2>{foundBook.title}</h2>
        <p class="book-meta">
          {foundBook.author} · {foundBook.year} · {foundBook.pages} pages
        </p>
        <p class="book-summary">{foundBook.summary}</p>
      </div>
    </div>
    <div class="book-actions">
      <button class="pill-btn pill-add" on:click={handleAdd}>
        Add
      </button>
      <button class="pill-btn pill-discard" on:click={handleDiscard}>
        Discard
      </button>
    </div>
  </div>
{:else}
  <div
    class="glass-capsule"
    class:fade-in-delayed={searchState === "idle"}
    class:glass-capsule-loading={searchState === "loading"}
  >
    {#if searchState === "idle"}
      <input
        type="text"
        bind:value={bookTitle}
        placeholder="What are you reading?"
        on:input={handleInput}
        on:keydown={handleKeydown}
        on:focus={handleFocus}
        on:blur={handleBlur}
      />
    {:else if searchState === "loading"}
      <div class="loading-circle"></div>
    {/if}
  </div>
{/if}

<style>
  /* Glass input capsule / morphing shell */
  .glass-capsule {
    background: rgba(255, 255, 255, 0.3);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    padding: 18px 36px;
    border-radius: 100px;
    border: 1px solid rgba(255, 255, 255, 0.5);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.05);
    width: 340px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition:
      width 0.35s cubic-bezier(0.25, 1, 0.5, 1),
      height 0.35s cubic-bezier(0.25, 1, 0.5, 1),
      padding 0.35s cubic-bezier(0.25, 1, 0.5, 1),
      border-radius 0.35s cubic-bezier(0.25, 1, 0.5, 1),
      box-shadow 0.35s ease,
      background 0.35s ease;
  }

  .glass-capsule.fade-in-delayed {
    animation: fadeIn 0.8s ease forwards;
    opacity: 0;
  }

  /* Morph state: pill → circle for loader */
  .glass-capsule-loading {
    width: 72px;
    height: 72px;
    padding: 0;
    border-radius: 999px;
    box-shadow: 0 14px 36px rgba(181, 119, 83, 0.35);
    background: rgba(255, 255, 255, 0.45);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .glass-capsule:focus-within {
    transform: scale(1.03);
    background: rgba(255, 255, 255, 0.45);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.08);
  }

  /* Loading circle inside morphing capsule */
  .loading-circle {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    border: 3px solid rgba(255, 255, 255, 0.6);
    border-top-color: rgba(205, 132, 94, 1);
    box-shadow: 0 0 24px rgba(205, 132, 94, 0.4);
    animation: spin 0.9s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Mock book result card; expands in smoothly */
  .book-result {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 24px;
    max-width: 640px;
    transform-origin: center center;
    animation: resultIn 0.4s cubic-bezier(0.25, 0.9, 0.3, 1) forwards;
  }

  .book-result-adding {
    animation: resultAddOut 0.55s cubic-bezier(0.15, 0.9, 0.3, 1) forwards;
  }

  .book-result-discarding {
    animation: resultDiscardOut 0.45s cubic-bezier(0.25, 0.7, 0.35, 1) forwards;
  }

  @keyframes resultIn {
    from {
      opacity: 0;
      transform: scale(0.9) translateY(8px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }

  @keyframes resultAddOut {
    0% {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
    45% {
      opacity: 1;
      transform: scale(1.03) translateY(-2px);
    }
    100% {
      opacity: 0;
      transform: scale(0.92) translateY(-8px);
    }
  }

  @keyframes resultDiscardOut {
    0% {
      opacity: 1;
      transform: scale(1) translateY(0) rotate(0deg);
      filter: blur(0px);
    }
    35% {
      opacity: 1;
      transform: scale(0.98) translateY(4px) rotate(-1.5deg);
      filter: blur(0px);
    }
    100% {
      opacity: 0;
      transform: scale(0.9) translateY(18px) rotate(-3deg);
      filter: blur(2px);
    }
  }

  .book-card {
    display: flex;
    gap: 24px;
    align-items: stretch;
    background: rgba(255, 255, 255, 0.3);
    border-radius: 24px;
    padding: 22px 26px;
    border: 1px solid rgba(255, 255, 255, 0.6);
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.06);
  }

  .book-cover {
    width: 120px;
    min-width: 120px;
    height: 150px;
    border-radius: 18px;
    background: linear-gradient(145deg, #f0c3a3, #f7e4d3);
    box-shadow:
      0 10px 25px rgba(181, 119, 83, 0.35),
      inset 0 0 12px rgba(255, 255, 255, 0.7);
  }

  .book-info {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .book-info h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #4b332e;
  }

  .book-meta {
    margin: 0;
    font-size: 0.9rem;
    color: rgba(75, 51, 46, 0.7);
  }

  .book-summary {
    margin: 6px 0 0;
    font-size: 0.95rem;
    line-height: 1.4;
    color: rgba(75, 51, 46, 0.8);
  }

  .book-actions {
    display: flex;
    gap: 12px;
    justify-content: center;
  }

  .pill-btn {
    min-width: 110px;
    padding: 9px 22px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.7);
    background: rgba(255, 255, 255, 0.22);
    color: #5b3b30;
    font-size: 0.9rem;
    font-weight: 500;
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    cursor: pointer;
    transition:
      background 0.2s ease,
      transform 0.15s ease,
      box-shadow 0.2s ease;
  }

  .pill-btn:hover {
    background: rgba(255, 255, 255, 0.35);
    transform: translateY(-1px);
    box-shadow: 0 10px 25px rgba(181, 119, 83, 0.25);
  }

  .pill-add {
    border-color: rgba(255, 255, 255, 0.9);
  }

  .pill-discard {
    border-color: rgba(255, 255, 255, 0.6);
    background: rgba(255, 255, 255, 0.16);
  }

  /* Text input */
  input {
    width: 100%;
    background: transparent;
    border: none;
    outline: none;
    font-size: 1.2rem;
    color: #5e4b4b;
    text-align: center;
    font-weight: 500;
    font-family: "Inter", sans-serif;
  }

  input::placeholder {
    color: rgba(94, 75, 75, 0.45);
    font-weight: 400;
    transition: opacity 0.15s ease-out;
  }

  input:focus::placeholder {
    opacity: 0;
  }
</style>
