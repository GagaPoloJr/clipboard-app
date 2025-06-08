# Clipboard App ✂️📋

This is a clipboard manager app built using **Tauri**, **Vue 3**, **TypeScript**, and **Rust**.

## Why this app?

macOS does not come with built-in clipboard history functionality like Windows (e.g. `Win + V`). That’s why I created this simple app to keep a local clipboard history.

While building this, I'm also exploring and learning how Rust works (although I still don’t fully understand it yet 😅: skill issue). This app is part of my journey to understand system-level programming and native app development using Rust and Tauri.

## Features (still develop)

- Users can save up to 20 clipboard entries (stored locally).
- Use `Cmd + Shift + V` to trigger a popup window to select and paste a previously copied text.
- A simple main window interface with a search bar to filter clipboard entries.

## Next Features to Implement

- Persist clipboard data even after the app is closed.
- Show the light popup window near the user's current cursor location on the screen.
- Optimize for lightweight bundle size.
- Add option to run in the background automatically when the system starts.
- Support for pinning frequently used text items.
