<script lang="ts">
  import { fade } from 'svelte/transition';

  // Props for two-way binding with parent
  export let activeFilter = 'All';
  export let sortOption = 'Newest';
  export let searchQuery = "";

  // Local state for the dropdown
  let showSortMenu = false;

  function setFilter(filter: string) {
    activeFilter = filter;
  }

  function toggleSortMenu() {
    showSortMenu = !showSortMenu;
  }

  function closeMenu() {
    showSortMenu = false;
  }

  function selectSort(option: string) {
    sortOption = option;
    showSortMenu = false;
  }
</script>

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

<style>
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
  .chevron { transition: transform 0.2s; }
  .chevron.rotated { transform: rotate(180deg); }
  
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
</style>
