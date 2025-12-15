<script lang="ts">
  export let activeTab: string;
</script>

<aside class="sidebar">
  <div class="line"></div>
  
  <nav class="nav-menu">
    <!-- Home Button -->
    <button 
      class="nav-btn" 
      class:active={activeTab === 'home'} 
      on:click={() => activeTab = 'home'}
      aria-label="Home"
    >
      <svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
      </svg>
    </button>
    
    <!-- Library/Menu Button -->
    <button 
      class="nav-btn" 
      class:active={activeTab === 'menu'} 
      on:click={() => activeTab = 'menu'}
      aria-label="Library"
    >
      <svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <line x1="3" y1="12" x2="21" y2="12"></line>
        <line x1="3" y1="6" x2="21" y2="6"></line>
        <line x1="3" y1="18" x2="21" y2="18"></line>
      </svg>
    </button>

    <!-- Settings Button -->
    <button 
      class="nav-btn" 
      class:active={activeTab === 'settings'} 
      on:click={() => activeTab = 'settings'}
      aria-label="Settings"
    >
      <svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="2"></circle>
        <circle cx="19" cy="12" r="2"></circle>
        <circle cx="5" cy="12" r="2"></circle>
      </svg>
    </button>
  </nav>
</aside>

<style>
  .sidebar { 
    width: 80px; height: 100%; position: relative; z-index: 10; 
    display: flex; flex-direction: column; align-items: center; justify-content: center; 
  }

  .line { 
    position: absolute; right: 0; top: 10%; bottom: 10%; 
    width: 2px; background: rgba(41, 32, 27, 0.2); 
  }

  .nav-menu {
    display: flex; flex-direction: column; gap: 32px; 
    align-items: center; margin-right: 12px; 
  }

  .nav-btn {
    background: transparent; border: none; cursor: pointer;
    color: rgba(94, 75, 75, 0.4); padding: 8px; 
    transition: all 0.2s ease; display: flex; 
    align-items: center; justify-content: center;
  }

  .nav-btn:hover { color: rgba(94, 75, 75, 0.8); transform: scale(1.1); }
  .nav-btn.active { color: #5e4b4b; }
</style>
