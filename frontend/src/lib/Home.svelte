<script lang="ts">
  import Sidebar from './Sidebar.svelte';
  import Library from './Library.svelte';
  import Background from './home-components/Background.svelte';
  import Orb from './home-components/Orb.svelte';
  import BookSearchModule from './home-components/BookSearchModule.svelte';

  // UI state
  let bookTitle = "";
  let isPulsing = false;
  let isFocused = false;
  let activeTab: "home" | "menu" = "home";
  let previousTab: "home" | "menu" = activeTab;

  // Return animation state (home bounce after leaving library)
  type ReturnStage = "idle" | "fading" | "bouncing_down" | "bouncing_up";
  let returnStage: ReturnStage = "idle";
  let isReturning = false;

  // Search / mock result state
  type SearchState = "idle" | "loading" | "result";
  let searchState: SearchState = "idle";

  type MockBook = {
    title: string;
    author: string;
    year: string;
    pages: string;
    summary: string;
  };

  let foundBook: MockBook | null = null;

  // "Add" and "Discard" confirmation animation states
  let isAdding = false;
  let isDiscarding = false;

  // Reference to the orb element for smooth float → center transition
  let orbElement: HTMLDivElement | null = null;

  // Tab change effects
  $: if (activeTab !== previousTab) {
    if (previousTab === "menu" && activeTab === "home") {
      triggerBounceSequence();
    }
    previousTab = activeTab;
  }

  async function triggerBounceSequence() {
    isReturning = true;

    returnStage = "fading";
    await wait(250);

    returnStage = "bouncing_down";
    await wait(700);

    returnStage = "bouncing_up";
    await wait(600);

    returnStage = "idle";
    isReturning = false;
  }

  function wait(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  // Derived visual states for Orb
  $: isGlowing =
    (isReturning && returnStage !== "idle") ||
    (isFocused && activeTab === "home") ||
    searchState === "loading" ||
    searchState === "result" ||
    isAdding;

  $: shouldScale = isFocused && activeTab === "home" && !isReturning;

  // Input interactions (BookSearchModule dispatches CustomEvents)
  function handleInput(_event: CustomEvent<Event>) {
    if (activeTab !== "home") return;
    isPulsing = true;
    setTimeout(() => (isPulsing = false), 100);
  }

  function handleFocus(_event: CustomEvent<FocusEvent>) {
    if (activeTab !== "home") return;
    isFocused = true;
    settleOrbToCenter();
  }

  function handleBlur(_event: CustomEvent<FocusEvent>) {
    isFocused = false;
    restoreOrbFloat();
  }

  async function handleKeydown(event: CustomEvent<KeyboardEvent>) {
    const e = event.detail;

    if (e.key === "Enter" && bookTitle.trim() && searchState === "idle") {
      const query = bookTitle.trim();
      bookTitle = "";
      searchState = "loading";

      // mock "search"
      await wait(1200);

      foundBook = {
        title: query,
        author: "Haruki Murakami",
        year: "2002",
        pages: "384",
        summary:
          "A quiet, introspective novel about memory, routine, and the small rituals that anchor a life."
      };

      searchState = "result";
    }
  }

  async function handleAdd() {
    if (!foundBook || isAdding || isDiscarding) return;
    console.log("Add book:", foundBook);

    isAdding = true;
    await wait(600);
    isAdding = false;
    resetSearch();
  }

  async function handleDiscard() {
    if (!foundBook || isAdding || isDiscarding) return;

    isDiscarding = true;
    await wait(450); // match discard animation duration
    isDiscarding = false;
    resetSearch();
  }

  function resetSearch() {
    searchState = "idle";
    foundBook = null;
    isAdding = false;
    isDiscarding = false;
  }

  function settleOrbToCenter() {
    if (!orbElement) return;

    const el = orbElement;
    const computed = getComputedStyle(el);
    const currentTransform =
      computed.transform === "none" ? "" : computed.transform;

    el.style.animation = "none";
    el.style.transform = currentTransform;

    requestAnimationFrame(() => {
      el.style.transition =
        "transform 0.6s cubic-bezier(0.25, 0.8, 0.25, 1)";
      el.style.transform = "translateY(0)";
    });
  }

  function restoreOrbFloat() {
    if (!orbElement) return;
    orbElement.style.transition = "";
    orbElement.style.transform = "";
    orbElement.style.animation = "";
  }
</script>

<main>
  <Background />

  <Sidebar bind:activeTab={activeTab} />

  <section class="orb-stage">
    <Orb
      bind:orbElement={orbElement}
      {activeTab}
      {isReturning}
      {returnStage}
      {isGlowing}
      {shouldScale}
      {isPulsing}
      {isAdding}
    >
      {#if activeTab === "home" && !isReturning}
        <BookSearchModule
          bind:bookTitle={bookTitle}
          {searchState}
          {foundBook}
          {isAdding}
          {isDiscarding}
          on:input={handleInput}
          on:keydown={handleKeydown}
          on:focus={handleFocus}
          on:blur={handleBlur}
          on:add={handleAdd}
          on:discard={handleDiscard}
        />
      {:else if activeTab === "menu" || (isReturning && returnStage === "fading")}
        <div class="library-container" class:fade-out={returnStage === "fading"}>
          <Library />
        </div>
      {/if}
    </Orb>
  </section>
</main>

<style>
  main {
    display: flex;
    height: 100vh;
    width: 100vw;
    position: relative;
  }

  .orb-stage {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
    z-index: 5;
  }

  .library-container {
    width: 100%;
    height: 100%;
    opacity: 1;
    transition: opacity 0.25s ease-out;
  }

  .library-container.fade-out {
    opacity: 0;
  }
</style>
