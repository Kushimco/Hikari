// src/lib/api.ts
import { invoke } from '@tauri-apps/api/core';

export type Book = {
  id: string;
  title: string;
  author: string;
  cover: string;
  cover_color: string;
  status: string;
  date_added?: string;
  total_pages: number;
  pages_read: number;
};

export async function getBooks(): Promise<Book[]> {
  return await invoke('get_books');
}

export async function updateBookProgress(id: string, pagesRead: number) {
  return await invoke('update_book_progress', { id, pagesRead });
}

export async function updateBookStatus(id: string, status: string) {
  return await invoke('update_book_status', { id, status });
}

export async function deleteBook(id: string) {
  return await invoke('delete_book', { id });
}
