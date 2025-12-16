# 🏮 Hikari

**A minimal, animated library for your digital bookshelf.**

Hikari is a desktop application built to track the books you are reading. It was created as a personal project to explore modern desktop development, combining the performance of **Rust** with the fluidity of **Svelte**.

This documentation is divided into two sections:

1. **App Showcase**: A visual tour of the interface, animations, and features.
2. **Developer Guide**: Instructions on how to set up, run, and build the project yourself.

---

## ✨ Showcase

Hikari features a unique "Orb" interface that morphs and adapts as you navigate.

### 1. Home & Search

*The app rests as a breathing orb. Typing instantly expands it into a seamless search bar powered by OpenLibrary.*

<p align="center">
  <img src="https://github.com/user-attachments/assets/3c81cc42-fec4-4a30-8b89-37cfb1d8aa55" alt="Search Animation" width="600">
</p>

### 2. Your Library

*The orb expands into a full window to display your collection. Track your progress, filter by status, and see your reading stats.*

<p align="center">
  <img src="https://github.com/user-attachments/assets/2756a89d-a60b-4dcc-9a9f-b49faf660f2f" alt="Library Transition" width="600">
</p>

---

## 🛠️ Developer Guide

It is built on the **Tauri v2** stack.

### 📂 Project Structure

The project is split into two distinct parts: the **Frontend** (UI) and the **Core** (Backend).
```
hikari/
├── frontend/           # The User Interface (Svelte 5 + TypeScript)
│   ├── src/            # All UI components, animations, and pages
│   └── package.json    # Frontend dependencies
├── src-tauri/          # The Backend (Rust)
│   ├── src/            # Rust code for windowing and file system access
│   ├── capabilities/   # Permission settings (http, fs, shell)
│   └── tauri.conf.json # Main configuration (App name, version, icons)
├── package.json        # Root scripts to run the app
└── README.md           # Documentation
```

### 🚀 How to Run Locally

Follow these steps to get Hikari running on your machine.

#### 1. Clone the Repository

Download the source code to your local machine.

```
git clone https://github.com/your-username/hikari.git
cd hikari
```

#### 2. Install Dependencies

Hikari requires **Node.js** (for the UI) and **Rust** (for the backend).

> [!IMPORTANT]
> Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```ruby
npm install
```

#### 3. Run in Development Mode

This command starts two processes: a local Svelte server (with hot-reload) and the Tauri window that wraps it.

```ruby
npx tauri dev
```

#### 4. Build for Production

When you are ready to create a standalone `.exe` (Windows) or `.app` (macOS):

```ruby
npx tauri build
```

> The final installer will be located in: `src-tauri/target/release/bundle/nsis/`

### ⚡ Tech Stack

- **Core**: Rust (Tauri v2)
- **Frontend**: Svelte 5 + TypeScript
- **Styling**: CSS (Custom animations & Glassmorphism)
- **Data**: Local JSON file

---

## 🗺️ Roadmap

- [x] **Core Architecture:** Project setup with Tauri 2.0 & Svelte.
- [x] **Basic CRUD:** Add, remove, and list books locally.
- [x] **UI Polishing:** Custom animations and glassmorphism theme.
- [x] **Online Metadata:** Auto-fetch book covers and details from open APIs (e.g., Open Library).
- [x] **Advanced Sorting:** Filter by genre, author, or read status.
- [ ] **Mobile Support:** Explore Android/iOS builds using Tauri's cross-platform capabilities.

