<script lang="ts">
  // Mock Data for UI development
  let books = [
    { id: 1, title: "The Design of Everyday Things", author: "Don Norman", coverColor: "#FF9A9E", status: "reading" },
    { id: 2, title: "Clean Code", author: "Robert C. Martin", coverColor: "#A18CD1", status: "finished" },
    { id: 3, title: "Dune", author: "Frank Herbert", coverColor: "#FBC2EB", status: "to-read" },
    { id: 4, title: "Atomic Habits", author: "James Clear", coverColor: "#84FAB0", status: "reading" },
    { id: 5, title: "Neuromancer", author: "William Gibson", coverColor: "#E0C3FC", status: "finished" },
    { id: 6, title: "Sapiens", author: "Yuval Noah Harari", coverColor: "#4FACFE", status: "to-read" },
  ];
</script>

<div class="library-content">
  <header class="lib-header">
    <h2>Library</h2>
    <div class="filters">
      <button class="filter-pill active">All</button>
      <button class="filter-pill">Reading</button>
      <button class="filter-pill">Finished</button>
    </div>
  </header>

  <div class="book-grid">
    {#each books as book}
      <div class="book-card">
        <!-- Book Cover Placeholder -->
        <div class="cover" style="background: linear-gradient(135deg, {book.coverColor} 0%, white 200%);">
          <span class="status-dot {book.status}"></span>
        </div>
        
        <!-- Book Info -->
        <div class="info">
          <h3 class="title">{book.title}</h3>
          <p class="author">{book.author}</p>
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  /* Import the Playfair Display font */
  @import url('https://fonts.googleapis.com/css2?family=Playfair+Display:ital,wght@0,400;0,600;1,400&display=swap');

  .library-content {
    /* Animation delay to sync with Orb expansion */
    animation: fadeIn 0.6s ease 0.6s forwards; 
    opacity: 0; 
    color: #5e4b4b;
    height: 100%;
    width: 100%;
    padding-bottom: 40px; /* Space for scrolling */
  }

  /* --- Header & Filters --- */
  .lib-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 25px;
  }

  h2 {
    margin: 0;
    font-family: 'Playfair Display', serif; /* NEW FONT */
    font-size: 2.5rem; /* Larger for impact */
    font-weight: 600;
    letter-spacing: -0.5px;
    color: #4a3b3b; /* Dark, warm brown */
  }

  .filters {
    display: flex;
    gap: 8px;
  }

  .filter-pill {
    background: rgba(255, 255, 255, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.5);
    padding: 6px 16px;
    border-radius: 20px;
    font-size: 0.85rem;
    font-family: 'Inter', sans-serif;
    font-weight: 500;
    color: #6b5b5b;
    cursor: pointer;
    transition: all 0.2s;
  }

  .filter-pill:hover { background: rgba(255, 255, 255, 0.5); }
  .filter-pill.active { 
    background: #5e4b4b; 
    color: white; 
    border-color: transparent;
  }

  /* --- Grid Layout --- */
  .book-grid {
    display: grid;
    /* Responsive columns: roughly 140px wide min */
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 24px;
    padding-right: 10px; /* Avoid scrollbar overlap */
  }

  /* --- Card Styles --- */
  .book-card {
    display: flex;
    flex-direction: column;
    gap: 12px;
    cursor: pointer;
    transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .book-card:hover {
    transform: translateY(-8px);
  }

  .cover {
    aspect-ratio: 2 / 3; /* Standard book ratio */
    width: 100%;
    border-radius: 12px;
    position: relative;
    box-shadow: 
      0 4px 10px rgba(0,0,0,0.05),
      0 10px 20px rgba(0,0,0,0.03),
      inset 0 0 0 1px rgba(255,255,255,0.2); /* Subtle inner border */
    overflow: hidden;
  }

  /* Status Dot (Reading/Finished) */
  .status-dot {
    position: absolute;
    top: 10px; right: 10px;
    width: 8px; height: 8px;
    border-radius: 50%;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  .status-dot.reading { background: #4ade80; } /* Green */
  .status-dot.finished { background: #60a5fa; } /* Blue */
  .status-dot.to-read { background: #cbd5e1; } /* Grey */

  /* Text Info */
  .info { padding: 0 4px; }

  .title {
    font-family: 'Playfair Display', serif; /* NEW FONT */
    font-size: 1.1rem;
    font-weight: 600;
    color: #2c1810;
    margin: 0 0 4px 0;
    line-height: 1.2;
    /* Truncate text after 2 lines */
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .author {
    font-family: 'Inter', sans-serif;
    font-size: 0.75rem;
    font-weight: 500;
    color: rgba(94, 75, 75, 0.65);
    letter-spacing: 0.03em;
    text-transform: uppercase; 
    margin: 0;
  }

  @keyframes fadeIn { to { opacity: 1; } }
</style>
