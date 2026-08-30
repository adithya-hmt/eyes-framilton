import { useState } from "react";

type Platform = "Linux" | "Windows";

const platformNotes: Record<Platform, { label: string; copy: string; detail: string }> = {
  Linux: {
    label: "AT-SPI2",
    copy: "Reads the focused application through the desktop accessibility bus.",
    detail: "Works best on GNOME and KDE when accessibility is enabled.",
  },
  Windows: {
    label: "UI Automation",
    copy: "Reads the focused window through the native UI Automation tree.",
    detail: "No special permission prompt on supported Windows versions.",
  },
};

function Eye({ awake }: { awake: boolean }) {
  return (
    <div className={`eye-mark ${awake ? "eye-mark--awake" : ""}`} aria-hidden="true">
      <span className="eye-mark__lid" />
      <span className="eye-mark__iris" />
      <span className="eye-mark__glint" />
    </div>
  );
}

function StatusDot({ live = false }: { live?: boolean }) {
  return <span className={`status-dot ${live ? "status-dot--live" : ""}`} aria-hidden="true" />;
}

function App() {
  const [activePlatform, setActivePlatform] = useState<Platform>("Linux");
  const [demoAwake, setDemoAwake] = useState(true);

  return (
    <div className="site-shell">
      <header className="site-header">
        <a className="wordmark" href="#top" aria-label="Eyes home">
          <span className="wordmark__eye" aria-hidden="true"><i /></span>
          <span>eyes</span>
        </a>
        <nav className="site-nav" aria-label="Primary navigation">
          <a href="#how">How it works</a>
          <a href="#platforms">Platforms</a>
          <a href="#build">Build it</a>
        </nav>
        <a className="header-link" href="#build">See the build <span aria-hidden="true">↗</span></a>
      </header>

      <main id="top">
        <section className="hero section-grid">
          <div className="hero-copy">
            <p className="eyebrow"><StatusDot live /> Local work memory / 01</p>
            <h1>Remember what<br /><em>you were doing.</em></h1>
            <p className="hero-lede">Eyes quietly records the work in your focused window, then leaves it as plain Markdown on your own machine.</p>
            <div className="hero-actions">
              <a className="button button--ink" href="#build">Build Eyes <span aria-hidden="true">↗</span></a>
              <a className="quiet-link" href="#how">See the small version <span aria-hidden="true">↓</span></a>
            </div>
            <p className="hero-note"><StatusDot /> No screenshots · No account · No cloud</p>
          </div>

          <div className="hero-demo" id="demo">
            <div className="demo-topline"><span>eyes / desktop recorder</span><span>v0.1</span></div>
            <div className="demo-window">
              <div className="demo-window__bar"><span className="window-controls"><i /><i /><i /></span><span>Today’s context</span><span className="demo-live"><StatusDot live /> {demoAwake ? "watching" : "paused"}</span></div>
              <div className="demo-window__body">
                <div className="demo-eye-col">
                  <button className="eye-button" type="button" onClick={() => setDemoAwake((value) => !value)} aria-label={demoAwake ? "Pause demo recording" : "Resume demo recording"}>
                    <Eye awake={demoAwake} />
                  </button>
                  <span>{demoAwake ? "eye open" : "eye closed"}</span>
                </div>
                <div className="context-list">
                  <div className="context-entry context-entry--active"><span className="context-time">10:42–11:16</span><strong>VS Code</strong><p>reader/linux.rs · mapping the active window into a useful daily note</p></div>
                  <div className="context-entry"><span className="context-time">09:58–10:41</span><strong>Firefox</strong><p>AT-SPI accessibility overview · focused application patterns</p></div>
                  <div className="context-entry"><span className="context-time">09:20–09:57</span><strong>Terminal</strong><p>eyes / feature branch · cargo test</p></div>
                </div>
              </div>
              <div className="demo-window__footer"><span><StatusDot live /> {demoAwake ? "saving locally" : "not recording"}</span><span>~/Eyes / 3 blocks</span></div>
            </div>
            <p className="demo-caption">The useful part is boring: a timeline you can grep, move, or hand to your own agent.</p>
          </div>
        </section>

        <section className="signal-strip" aria-label="Product principles">
          <div><span className="signal-number">01</span><strong>Focused window only</strong></div>
          <div><span className="signal-number">02</span><strong>Redacted before disk</strong></div>
          <div><span className="signal-number">03</span><strong>One file per day</strong></div>
          <div><span className="signal-number">04</span><strong>Plain Markdown</strong></div>
        </section>

        <section className="explain section-grid" id="how">
          <div className="section-intro">
            <h2>One quiet eye between your work and your memory.</h2>
            <p>Eyes is not a screenshot archive or a cloud dashboard. It is a tiny local loop that turns the context you are already looking at into a readable day file.</p>
          </div>
          <div className="flow-line" aria-label="Eyes capture flow">
            <div className="flow-step"><span className="flow-index">01</span><strong>Focused window</strong><p>Ask the operating system which window is actually in front.</p></div>
            <div className="flow-arrow" aria-hidden="true">→</div>
            <div className="flow-step"><span className="flow-index">02</span><strong>Text tree</strong><p>Read accessible text, title, app, and document metadata.</p></div>
            <div className="flow-arrow" aria-hidden="true">→</div>
            <div className="flow-step"><span className="flow-index">03</span><strong>Day file</strong><p>Redact, deduplicate, and append a Markdown block locally.</p></div>
          </div>
        </section>

        <section className="platforms section-grid" id="platforms">
          <div className="section-intro section-intro--compact">
            <h2>Same promise.<br /><em>Native readers.</em></h2>
            <p>The UI and writer can stay shared. The part that must respect each desktop is the focused-window reader.</p>
          </div>
          <div className="platform-panel">
            <div className="platform-tabs" role="tablist" aria-label="Platform implementation details">
              {(Object.keys(platformNotes) as Platform[]).map((platform) => (
                <button key={platform} type="button" role="tab" aria-selected={activePlatform === platform} className={activePlatform === platform ? "is-selected" : ""} onClick={() => setActivePlatform(platform)}>{platform}<span aria-hidden="true">↗</span></button>
              ))}
            </div>
            <div className="platform-detail" role="tabpanel">
              <div className="platform-symbol" aria-hidden="true">{activePlatform === "Linux" ? "⌁" : "⊞"}</div>
              <div><span className="platform-meta">Native adapter / {activePlatform}</span><h3>{platformNotes[activePlatform].label}</h3><p>{platformNotes[activePlatform].copy}</p><small>{platformNotes[activePlatform].detail}</small></div>
            </div>
            <div className="platform-shared"><span>Shared Rust core</span><span>redaction</span><span>segmenting</span><span>Markdown writer</span></div>
          </div>
        </section>

        <section className="day-file section-grid">
          <div className="file-preview" aria-label="Example Eyes Markdown file">
            <div className="file-preview__top"><span>2026-08-30.md</span><span>plain text</span></div>
            <pre><code>{`---\ndate: 2026-08-30\ncaptured_by: Eyes 0.1.0\n---\n\n## 10:42–11:16 · VS Code\n\nreader/linux.rs · mapping the active window\ninto a useful daily note\n\nsource: file:///home/wk/eyes/src-tauri/src/reader/linux.rs`}</code></pre>
            <div className="file-preview__bottom"><span>3 blocks today</span><span>saved on this computer</span></div>
          </div>
          <div className="section-intro section-intro--file">
            <p className="eyebrow">Output, not a product silo</p>
            <h2>Your notes should belong to you.</h2>
            <p>Open them in an editor. Search them from a terminal. Give the folder to an LLM you trust. Eyes keeps the output legible because legible files outlive apps.</p>
            <a className="quiet-link" href="#build">See the build contract <span aria-hidden="true">↗</span></a>
          </div>
        </section>

        <section className="build section-grid" id="build">
          <div className="build-copy">
            <h2>Build once.<br /><em>Keep it local.</em></h2>
            <p>Use the Tauri shell for the tray and settings window. Plug in the platform reader, then let the shared Rust core handle the privacy boundary and daily file.</p>
          </div>
          <div className="build-card">
            <div className="build-card__head"><span>Terminal</span><span>Linux / Windows</span></div>
            <pre><code>{`npm install\nnpm run tauri build`}</code></pre>
            <div className="build-checks"><span><StatusDot live /> Shared Tauri 2 shell</span><span><StatusDot /> Linux AT-SPI2 adapter</span><span><StatusDot /> Windows UI Automation adapter</span></div>
            <a className="button button--paper" href="#top">Back to the top <span aria-hidden="true">↑</span></a>
          </div>
        </section>
      </main>

      <footer className="site-footer">
        <a className="wordmark" href="#top"><span className="wordmark__eye" aria-hidden="true"><i /></span><span>eyes</span></a>
        <p>eyes.framilton.com · a local work memory</p>
        <p>Nothing to sync.</p>
      </footer>
    </div>
  );
}

export default App;
