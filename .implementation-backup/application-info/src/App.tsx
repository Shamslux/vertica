import "./App.css";

function App() {
  return (
    <main className="app-shell">
      <section className="welcome-panel" aria-labelledby="application-title">
        <header className="application-header">
          <span className="environment-badge">Development</span>

          <h1 id="application-title">Vertica</h1>

          <p>
            The desktop application foundation is initialized and ready for
            incremental development.
          </p>
        </header>

        <dl className="status-list">
          <div className="status-item">
            <dt>Frontend</dt>
            <dd>Ready</dd>
          </div>

          <div className="status-item">
            <dt>Desktop runtime</dt>
            <dd>Ready</dd>
          </div>

          <div className="status-item">
            <dt>Application version</dt>
            <dd>0.1.0</dd>
          </div>
        </dl>
      </section>
    </main>
  );
}

export default App;