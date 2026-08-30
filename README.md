# Eyes

Eyes is a small, local-first desktop recorder for Linux and Windows. It reads
the focused window through the platform accessibility API, removes obvious
secrets before writing, and appends readable Markdown to one file per day.

Landing page: https://eyes-framilton.vercel.app

This repository intentionally does not clone Ambient Context. It keeps the
product idea and rebuilds the shell, privacy pipeline, and site around Eyes.

## Current slice

- Vite/React landing page for `eyes.framilton.com`
- Tauri 2 desktop shell with a tray menu
- Shared redaction and Markdown writer with Rust tests
- Explicit reader seams for Linux AT-SPI2 and Windows UI Automation

The native readers are the only unfinished part of the product. Linux needs a
long-lived AT-SPI2 D-Bus connection and focused-tree traversal. Windows needs a
COM UI Automation thread using `GetFocusedElement`. Details and acceptance
checks live in [`docs/PLATFORM.md`](docs/PLATFORM.md).

## Run the site

```bash
npm install
npm run dev
```

## Build the desktop shell

Install Node, Rust, and the platform dependencies for Tauri 2, then run:

```bash
npm run tauri dev
npm run tauri build
```

## Privacy boundary

No screenshots, OCR, network sync, telemetry, or bundled model. A reader must
return only the focused window's accessible text and metadata. The shared
privacy module drops password managers and private browsing windows and
scrubs common credentials before the writer sees them.
