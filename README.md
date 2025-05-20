# Swordfish

> What is Swordfish?

Swordfish is a toolbelt for builders. The fast. The precise. The ones who are never quite satisfied.

It's for people who think faster than their tools.
Whether you code, sell, plan, or ship, Swordfish is made for builders who expect their tools to move as fast as they do.

## Technical Ethos (for devs and users alike)

### 🧠 Prioritize clarity over cleverness
Readable code is resilient code. If it takes more than a minute to understand what something does, rewrite it or document why it exists.

### ✂️ Refactor early, not endlessly
Cut complexity at the source. A small well-named file beats a giant magic one. Structure pays dividends in speed, trust, and onboarding.

### 📚 Docs are part of the interface
If we can't make something obvious, we make it documented. A one-line "why this exists" can unblock the next dev (even if it's you).

### 💡 Defaults are important
Good defaults are opinionated and extensible. Every pre-set should feel like a well-reasoned choice made with your best interest in mind, but you can always swap it out.

### 🚀 Developer Experience is User Experience
Clunky DX leaks into the product. Smooth scaffolds, rich hints, and fast feedback loops aren't a luxury—they're how we move fast and stay sane.

### 🔐 Trust through Type Safety
We fail early, loudly, and with context. Zod schemas and compile-time guards catch errors before they hit the user—or your logs.

### 🧭 Be fast, or don't bother
Performance isn't a feature, it's a foundation. If it's slow, we rethink it. Tools fail not because they can't do things—but because they can't do them fast enough to be useful.

## Technical stack

### Client
- Tauri
- React
- Typescript
- TailwindCSS
- Zod
- Zustand

### Backend
- Tauri
- Rust

## Planned features

- Task runner (Scripts, User defined workflows, applescript, AutoHotKey)
  - there is a task std lib for javascript
  - bring your own runtime (bash, python, node, etc)
- exposes internals through a clean CLI api and MCP server
- Search and preview files, docs etc
- first-class support for AI models and agents
  - bring your own LLM
  - expose the LLM to tasks
- Provides OS access to agents, allowing them to:
  - use pre-defined scripts
  - apps
  - applescript, accessbility api, etc
- themable UI via css variables

## Technical goals

- Performance is king. If we can't find a way to do it performantly, we probably shouldn't do it.
  - Intellij is a good example of a failure in this regard. Large projects become unusable and slow.
- The defaults should be great, and easily overrideable
  - Opinionated *and* Configurable
  - Ship defaults that feel like best-practice
  - Guide users towards success
- Fail Fast, embrace Type-Safety
  - Schemas (Zod) guard runtime and compile-time. Invalid inputs throw early in dev
  - Bugs surface at dev-time, not in production dashboards. |
- DX is UX
  - If the dev experience is clunky, the end-user experience decays. CLI scaffolds, exhaustive hints, and helpful error messages are first-class.


## Guiding Ethos

1. Be patient. No matter what.
2. Don't badmouth: Assign responsibility, not blame.
3. Never assume the motives of others are, to them, any less noble than yours are to you.
4. Expect no more of anyone than you can deliver yourself.
5. Concern yourself with what is right rather than who is right.
6. Never forget that, no matter how certain, you might be wrong.
7. Praise at least as often as you disparage.
8. Admit your errors freely and soon.

When in doubt, use good judgement.

