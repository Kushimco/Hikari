<script lang="ts">
  // You can add props here later, e.g., export let books = [];
</script>

<div class="library-content">
  <h2>Library</h2>
  <div class="book-grid">
    <!-- Placeholders for books -->
    <div class="book-card"></div>
    <div class="book-card"></div>
    <div class="book-card"></div>
    <div class="book-card"></div>
    <div class="book-card"></div>
    <div class="book-card"></div>
  </div>
</div>

<style>
  .library-content {
    animation: fadeIn 0.5s ease 0.5s forwards;
    opacity: 0; 
    color: #5e4b4b;
    height: 100%;
    width: 100%;
    
    /* Fade-in content */
    animation: fadeIn 0.8s ease 0.4s forwards; 
    opacity: 0; 
  }

  h2 {
    margin-top: 0;
    font-size: 2rem;
    margin-bottom: 20px;
    font-weight: 600;
  }

  .book-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 20px;
    padding-bottom: 20px;
  }

  .book-card {
    background: rgba(255,255,255,0.4);
    height: 180px;
    border-radius: 12px;
    box-shadow: 0 4px 10px rgba(0,0,0,0.05);
    transition: transform 0.2s;
  }
  
  .book-card:hover {
    transform: translateY(-5px);
    background: rgba(255,255,255,0.6);
  }

  @keyframes fadeIn { to { opacity: 1; } }
</style>
