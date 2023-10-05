# Tauri + Solid + Typescript

This template should help get you started developing with Tauri, Solid and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

---

- Inspired by Alfred, Sublime Text, ScriptKit and Neovim.
- Task runner (Scripts, User defined workflows, applescript, AutoHotKey)
  - Tasks can be defined via a json file in ~/.config/swordfish, or an alfred-like GUI
  - there is a task std lib for js and go
- exposes internals as CLI api

  - cli autocompletes task names `swordfish run ta...`

- Search and preview files, browser history, docs etc
- Control media players
- Customizable UI via css

## Technical goals

- Performance is king. If we can't find a way to do it performantly, we shouldn't do it.
- JS should not do any operations that are worse than O(n) by default.
- The JS side should remain a dumb client that consumes data from the rust backend and displays it.
- The app should be configurable via javascript, much like Neovim is configurable via Lua.
- The defaults should be great, and easily overrideable

## Contribution guide

The Rules

1. Be patient. No matter what.
2. Don’t badmouth: Assign responsibility, not blame.
3. Never assume the motives of others are, to them, any less noble than yours are to you.
4. Expect no more of anyone than you can deliver yourself.
5. Laugh at yourself frequently.
6. Concern yourself with what is right rather than who is right.
7. Never forget that, no matter how certain, you might be wrong.
8. Praise at least as often as you disparage.
9. Admit your errors freely and soon.
