<script lang="ts">
  import { createEventDispatcher } from "svelte";

  type SearchState = "idle" | "loading" | "result";

  type MockBook = {
    title: string;
    author: string;
    year: string;
    pages: string;
    summary: string;       // truncated
    fullSummary?: string;  // full text for modal
    coverUrl?: string | null;
  };

  export let bookTitle = "";
  export let searchState: SearchState = "idle";
  export let books: MockBook[] = [];
  export let isAdding = false;
  export let isDiscarding = false;

  const dispatch = createEventDispatcher<{
    input: Event;
    keydown: KeyboardEvent;
    focus: FocusEvent;
    blur: FocusEvent;
    add: MockBook;
    done: void;
    openSummary: MockBook;
  }>();

  let currentIndex = 0;
  let direction: "next" | "prev" = "next";

  // Reset to first book when new results arrive
  $: if (books && books.length > 0 && currentIndex >= books.length) {
    currentIndex = 0;
  }

  $: currentBook = books.length ? books[currentIndex] : null;

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

  function handleAdd(book: MockBook) {
    dispatch("add", book);
  }

  function handleOpenSummary(book: MockBook) {
    dispatch("openSummary", book);
  }

  function handleDone() {
    dispatch("done");
  }

  function goNext() {
    if (!books.length) return;
    direction = "next";
    currentIndex = (currentIndex + 1) % books.length;
  }

  function goPrev() {
    if (!books.length) return;
    direction = "prev";
    currentIndex = (currentIndex - 1 + books.length) % books.length;
  }
</script>

{#if searchState === "result" && currentBook}
  <div class="carousel-shell">
    <div class="carousel-row">
      {#if books.length > 1}
        <button
          type="button"
          class="nav-bubble nav-bubble-left"
          on:click={goPrev}
          aria-label="Previous result"
        >
          ‹
        </button>
      {/if}

      {#key currentBook.title + currentIndex}
        <div
          class="book-card-wrapper"
          class:slide-next={direction === "next"}
          class:slide-prev={direction === "prev"}
        >
          <div
            class="book-card"
            class:book-card-adding={isAdding}
            class:book-card-discarding={isDiscarding}
          >
            <div class="book-cover">
              {#if currentBook.coverUrl}
                <img
                  src={currentBook.coverUrl}
                  alt={`Cover of ${currentBook.title}`}
                  loading="lazy"
                />
              {/if}
            </div>
            <div class="book-info">
              <h2>{currentBook.title}</h2>
              <p class="book-meta">
                {currentBook.author} · {currentBook.year} · {currentBook.pages} pages
              </p>
              <button
                type="button"
                class="book-summary-button"
                on:click={() => handleOpenSummary(currentBook)}
                aria-label={`Read full description of ${currentBook.title}`}
              >
                {currentBook.summary}
              </button>
            </div>
          </div>

          {#if books.length > 1}
            <div class="dot-row">
              {#each books as _b, i}
                <span
                  class="dot"
                  class:dot-active={i === currentIndex}
                ></span>
              {/each}
            </div>
          {/if}
        </div>
      {/key}

      {#if books.length > 1}
        <button
          type="button"
          class="nav-bubble nav-bubble-right"
          on:click={goNext}
          aria-label="Next result"
        >
          ›
        </button>
      {/if}
    </div>

    <div class="carousel-footer">
      <button
        type="button"
        class="pill-btn pill-done"
        on:click={handleDone}
      >
        Done
      </button>
      <button
        type="button"
        class="pill-btn pill-add"
        on:click={() => handleAdd(currentBook)}
      >
        Add
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
  /* Carousel shell sized to sit nicely inside orb */
  .carousel-shell {
    width: 420px;
    max-width: 100%;
    display: flex;
    flex-direction: column;
    gap: 18px;
    align-items: center;
    justify-content: center;
  }

  .carousel-row {
    width: 100%;
    display: grid;
    grid-template-columns: auto 1fr auto;
    align-items: center;
    column-gap: 10px;
  }

  .book-card-wrapper {
    position: relative;
  }

  /* Navigation bubbles */
  .nav-bubble {
    width: 40px;
    height: 40px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.7);
    background: rgba(255, 255, 255, 0.2);
    color: #5b3b30;
    font-size: 1.4rem;
    display: flex;
    align-items: center;
    justify-content: center;
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    cursor: pointer;
    box-shadow: 0 10px 25px rgba(181, 119, 83, 0.25);
    transition:
      background 0.2s ease,
      transform 0.15s ease,
      box-shadow 0.2s ease;
  }

  .nav-bubble:hover {
    background: rgba(255, 255, 255, 0.32);
    transform: translateY(-1px);
  }

  .nav-bubble-left {
    justify-self: flex-start;
  }

  .nav-bubble-right {
    justify-self: flex-end;
  }

  /* Card */
  .book-card {
    display: flex;
    gap: 20px;
    align-items: center; /* center cover + text vertically */
    background: rgba(255, 255, 255, 0.3);
    border-radius: 24px;
    padding: 22px 24px;  /* a bit more vertical padding */
    border: 1px solid rgba(255, 255, 255, 0.6);
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.06);
  }

  .book-card-adding {
    animation: resultAddOut 0.55s cubic-bezier(0.15, 0.9, 0.3, 1) forwards;
  }

  .book-card-discarding {
    animation: resultDiscardOut 0.45s cubic-bezier(0.25, 0.7, 0.35, 1) forwards;
  }

  .book-cover {
    width: 96px;
    min-width: 96px;
    height: 128px;
    border-radius: 18px;
    background: linear-gradient(145deg, #f0c3a3, #f7e4d3);
    box-shadow:
      0 10px 25px rgba(181, 119, 83, 0.35),
      inset 0 0 12px rgba(255, 255, 255, 0.7);
    overflow: hidden;
    flex-shrink: 0;
    display: flex;
    align-items: stretch;
    justify-content: center;
  }

  .book-cover img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .book-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .book-info h2 {
    margin: 0;
    font-size: 1.05rem;
    font-weight: 600;
    color: #4b332e;
    text-align: left;
  }

  .book-meta {
    margin: 0;
    font-size: 0.85rem;
    color: rgba(75, 51, 46, 0.7);
  }

  /* Description: clamp to 2 lines so layout stays stable */
  .book-summary-button {
    margin: 6px 0 0;
    padding: 0;
    border: none;
    background: none;
    font: inherit;
    text-align: left;
    cursor: pointer;
    font-size: 0.92rem;
    line-height: 1.4;
    color: rgba(75, 51, 46, 0.8);

    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2; /* WebKit / legacy Safari */
    line-clamp: 2;          /* standard property */
    overflow: hidden;
  }

  .book-summary-button:hover,
  .book-summary-button:focus-visible {
    color: rgba(75, 51, 46, 0.95);
    outline: none;
  }

  /* Little dots under card */
  .dot-row {
    display: flex;
    justify-content: center;
    gap: 6px;
    margin-top: 10px;
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.5);
  }

  .dot-active {
    background: rgba(181, 119, 83, 0.95);
    box-shadow: 0 0 10px rgba(181, 119, 83, 0.7);
  }

  /* Footer buttons */
  .carousel-footer {
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

  .pill-done {
    border-color: rgba(255, 255, 255, 0.6);
    background: rgba(255, 255, 255, 0.16);
  }

  /* Slide animations for next/prev */
  .slide-next {
    animation: slideNext 0.25s ease-out;
  }

  .slide-prev {
    animation: slidePrev 0.25s ease-out;
  }

  @keyframes slideNext {
    from {
      opacity: 0;
      transform: translateX(12px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  @keyframes slidePrev {
    from {
      opacity: 0;
      transform: translateX(-12px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
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

  /* Existing glass / loader styles */
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
