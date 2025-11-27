<script lang="ts">
  let bookTitle = "";
  let books = [
    {
      title: "The Alchemist",
      author: "Paulo Coelho",
      cover: "https://covers.openlibrary.org/b/id/10520157-L.jpg",
      summary: "A journey of self-discovery and following one's dreams."
    }
  ];

  function addBook() {
    if (bookTitle.trim()) {
      books = [
        ...books,
        {
          title: bookTitle,
          author: "Loading...",
          cover: "https://via.placeholder.com/120x180?text=?",
          summary: "Fetching summary..."
        }
      ];
      bookTitle = "";
    }
  }
</script>

<main>
  <!-- Background elements to make the glass effect pop -->
  <div class="ambient-light one"></div>
  <div class="ambient-light two"></div>

  <aside class="sidebar glass">
    <div class="brand">
      <h2>Hikari</h2>
    </div>
    <nav>
      <a class="active"><span class="icon">📚</span> Library</a>
      <a><span class="icon">✨</span> Wishlist</a>
      <a><span class="icon">📈</span> Stats</a>
      <div class="spacer"></div>
      <a><span class="icon">⚙️</span> Settings</a>
    </nav>
  </aside>

  <section class="content">
    <header class="glass-header">
      <h1>My Library</h1>
      <div class="user-profile">
        <div class="avatar">K</div>
      </div>
    </header>

    <div class="scroll-container">
      <div class="add-book-container glass">
        <input
          type="text"
          bind:value={bookTitle}
          placeholder="Type a book title to add..."
          on:keydown={(e) => e.key === "Enter" && addBook()}
        />
        <button on:click={addBook}>Add</button>
      </div>

      <div class="grid">
        {#each books as book}
          <div class="book-card glass">
            <div class="cover-wrapper">
              <img src={book.cover} alt={book.title} />
            </div>
            <div class="info">
              <h3>{book.title}</h3>
              <p class="author">{book.author}</p>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </section>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: 'Segoe UI', 'Inter', sans-serif;
    background-color: #fcfbf9; /* Creamy base */
    color: #4a4a4a;
    overflow: hidden; /* Prevent body scroll */
  }

  main {
    display: flex;
    height: 100vh;
    width: 100vw;
    position: relative;
  }

  /* --- Ambient Background Blobs (Essential for Glass Effect) --- */
  .ambient-light {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    z-index: -1;
    opacity: 0.6;
    animation: float 20s infinite ease-in-out alternate;
  }
  .one {
    width: 500px;
    height: 500px;
    background: #e8d5b5; /* Warm beige/sand */
    top: -100px;
    left: -100px;
  }
  .two {
    width: 600px;
    height: 600px;
    background: #dbece5; /* Soft sage/mint */
    bottom: -150px;
    right: -100px;
    animation-delay: -5s;
  }

  @keyframes float {
    0% { transform: translate(0, 0); }
    100% { transform: translate(30px, 50px); }
  }

  /* --- Glassmorphism Base Class --- */
  .glass {
    background: rgba(255, 255, 255, 0.45); /* Very translucent white */
    backdrop-filter: blur(16px) saturate(180%); /* The frost effect */
    -webkit-backdrop-filter: blur(16px) saturate(180%);
    border: 1px solid rgba(255, 255, 255, 0.6); /* Subtle highlight border */
    box-shadow: 0 8px 32px 0 rgba(156, 156, 156, 0.1); /* Soft shadow */
  }

  /* --- Sidebar --- */
  .sidebar {
    width: 240px;
    display: flex;
    flex-direction: column;
    padding: 2rem;
    border-radius: 0 24px 24px 0; /* Rounded only on right */
    z-index: 10;
  }
  .brand h2 {
    font-size: 1.8rem;
    color: #333;
    margin-bottom: 3rem;
    font-weight: 700;
    letter-spacing: -0.5px;
  }
  nav a {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    margin-bottom: 8px;
    border-radius: 12px;
    cursor: pointer;
    color: #555;
    font-weight: 500;
    transition: all 0.2s ease;
  }
  nav a:hover {
    background: rgba(255, 255, 255, 0.6);
    transform: translateX(5px);
  }
  nav a.active {
    background: rgba(255, 255, 255, 0.8);
    color: #000;
    box-shadow: 0 4px 12px rgba(0,0,0,0.05);
  }
  .spacer { flex: 1; }

  /* --- Main Content --- */
  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 0 2rem 2rem 2rem;
  }

  .glass-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 0;
    margin-bottom: 1rem;
  }
  .glass-header h1 {
    font-size: 1.6rem;
    font-weight: 600;
    color: #333;
  }
  .avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: #333;
    color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
  }

  .scroll-container {
    flex: 1;
    overflow-y: auto;
    padding-right: 10px;
  }

  /* --- Add Book Section --- */
  .add-book-container {
    display: flex;
    gap: 10px;
    padding: 1.2rem;
    border-radius: 20px;
    margin-bottom: 2.5rem;
  }
  input {
    flex: 1;
    border: none;
    background: rgba(255, 255, 255, 0.5);
    padding: 12px 16px;
    border-radius: 12px;
    font-size: 1rem;
    outline: none;
    color: #333;
    transition: background 0.2s;
  }
  input:focus {
    background: rgba(255, 255, 255, 0.8);
    box-shadow: 0 0 0 2px rgba(0,0,0,0.05);
  }
  button {
    padding: 10px 24px;
    border-radius: 12px;
    border: none;
    background: #333;
    color: #fff;
    font-weight: 600;
    cursor: pointer;
    transition: transform 0.1s;
  }
  button:active { transform: scale(0.96); }

  /* --- Grid --- */
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 24px;
  }
  
  .book-card {
    padding: 16px;
    border-radius: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    transition: transform 0.2s, box-shadow 0.2s;
  }
  .book-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 12px 40px rgba(0,0,0,0.08);
    background: rgba(255, 255, 255, 0.65);
  }
  .cover-wrapper {
    width: 120px;
    height: 180px;
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 8px 16px rgba(0,0,0,0.1);
    margin-bottom: 16px;
  }
  .cover-wrapper img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .book-card h3 {
    margin: 0.5rem 0 0.2rem 0;
    font-size: 1rem;
    color: #222;
  }
  .book-card .author {
    margin: 0;
    font-size: 0.85rem;
    color: #777;
  }
</style>
