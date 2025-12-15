<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import type { Book } from './api';

  export let book: Book;

  function coverSrc(cover: string): string {
    if (!cover) return "";
    return cover.startsWith("http") ? cover : convertFileSrc(cover);
  }

  function getProgressPercent(b: Book): number {
    if (!b.total_pages) return 0;
    return Math.min(100, Math.max(0, (b.pages_read / b.total_pages) * 100));
  }
</script>

<div class="book-card" on:click on:keydown tabindex="0" role="button">
  <div class="cover" style="background: linear-gradient(135deg, {book.cover_color || '#FF9A9E'} 0%, white 200%);">
    
    {#if coverSrc(book.cover)}
      <img src={coverSrc(book.cover)} alt={book.title} class="cover-img" loading="lazy" />
    {/if}
    
    {#if book.total_pages > 0}
      <div class="mini-progress-bar">
        <div class="mini-progress-fill" style="width: {getProgressPercent(book)}%"></div>
      </div>
    {/if}

    <span class="status-dot {book.status ? book.status.toLowerCase().replace(' ', '-') : ''}"></span>
  </div>
  
  <div class="info">
    <h3 class="title">{book.title}</h3>
    <p class="author">{book.author}</p>
  </div>
</div>

<style>
  .book-card { display: flex; flex-direction: column; gap: 12px; cursor: pointer; transition: transform 0.2s; }
  .book-card:hover { transform: translateY(-8px); }

  .cover { 
    aspect-ratio: 2 / 3; width: 100%; border-radius: 12px; position: relative; 
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.05); overflow: hidden; 
  }
  .cover-img { width: 100%; height: 100%; object-fit: cover; display: block; position: relative; z-index: 1; }

  .status-dot { position: absolute; top: 10px; right: 10px; width: 8px; height: 8px; border-radius: 50%; z-index: 2; }
  .status-dot.reading { background: #47f386; }
  .status-dot.finished { background: #529ffd; }
  .status-dot.to-read { background: #ff4eaf; }

  .mini-progress-bar { position: absolute; bottom: 0; left: 0; right: 0; height: 4px; background: rgba(0,0,0,0.2); z-index: 3; }
  .mini-progress-fill { height: 100%; background: rgba(255,255,255,0.9); transition: width 0.3s ease; }

  .info { padding: 0 4px; }
  .title { font-family: 'Playfair Display', serif; font-size: 1.1rem; font-weight: 600; color: #2c1810; margin: 0 0 4px 0; }
  .author { font-size: 0.75rem; font-weight: 500; color: rgba(94, 75, 75, 0.65); text-transform: uppercase; }
</style>
