# Vertica

Vertica is a local-first desktop application for managing structured personal projects.

The application is designed to bring project planning, progress tracking, schedules, activities, notes, metrics and specialized workflows into a single extensible environment.

A project in Vertica may represent:

* a study plan;
* preparation for an examination or certification;
* a software project;
* a health or fitness plan;
* a professional development plan;
* a personal goal;
* any structured initiative that requires planning and continuous tracking.

Vertica runs locally, stores its operational data in SQLite and does not depend on a remote service for its core functionality.

---

## Project status

Vertica is currently in its foundation phase.

The following capabilities are already implemented:

* Tauri desktop application;
* React and TypeScript frontend;
* Rust backend;
* typed IPC communication;
* local SQLite database;
* database migrations;
* database diagnostics;
* application information;
* persisted application settings;
* light, dark and system themes;
* English and Brazilian Portuguese interface support;
* repository and service abstractions;
* automated backend tests.

The next major milestone is the implementation of the project management domain.

The first product-oriented workflow will include:

1. listing projects;
2. creating a project;
3. opening a project;
4. editing project information;
5. archiving or deleting a project;
6. reopening the application with all data preserved.

---

## Product vision

Vertica is not intended to be a single-purpose tracker.

Its goal is to provide a robust project-centered platform in which each project can contain independent modules.

A possible project structure is:

```text
Project
├── Overview
├── Content
├── Activities
├── Schedule
├── Metrics
├── Notes
├── Files
└── Project settings
```

Different projects may use different combinations of modules.

For example, an examination preparation project may contain:

```text
Examination project
├── syllabus tracking
├── classes and materials
├── study sessions
├── revision cycles
├── examination schedule
├── performance metrics
└── health and physical preparation
```

A software project may instead contain:

```text
Software project
├── milestones
├── tasks
├── technical notes
├── activity history
├── deadlines
└── project metrics
```

The application architecture must remain independent from any single project type.

---

## Core principles

### Local-first

The SQLite database is the primary source of truth.

Core functionality must remain available without an internet connection.

### Project-centered

Operational data belongs to a project unless it is explicitly application-wide.

Examples of global data:

* application theme;
* interface language;
* application-level preferences.

Examples of project data:

* project metadata;
* project modules;
* content items;
* activities;
* schedules;
* notes;
* metrics.

### Modular

Features should be implemented as cohesive modules that can evolve independently.

Modules must not assume that every project is related to studying, examinations or health.

### Typed boundaries

Communication between React and Rust must use explicit request and response types.

IPC payloads should remain small, stable and validated.

### Layered architecture

Business rules must remain outside UI components, Tauri commands and database-specific implementations.

### Safe persistence

Database schema changes must be introduced through versioned migrations.

Existing user data must not be silently discarded.

---

## Technology stack

### Desktop runtime

* Tauri 2

### Frontend

* React 19
* TypeScript
* Vite
* CSS
* custom internationalization provider

### Backend

* Rust
* Tauri commands
* application services
* domain models
* repository abstractions

### Persistence

* SQLite
* versioned SQL migrations
* repository-based access

---

## Architecture

Vertica follows a layered architecture:

```text
React frontend
      │
      ▼
Typed IPC clients
      │
      ▼
Tauri commands
      │
      ▼
Application services
      │
      ▼
Domain models and repository contracts
      │
      ▼
Infrastructure repositories
      │
      ▼
SQLite
```

### Frontend

The frontend is responsible for:

* rendering the interface;
* collecting user input;
* managing view state;
* calling typed Tauri clients;
* displaying loading, success and error states;
* applying the selected interface language and theme.

The frontend must not contain persistence logic or authoritative business rules.

### Commands

Tauri commands are the IPC boundary.

Commands are responsible for:

* receiving frontend requests;
* validating transport-level input when necessary;
* creating or obtaining application dependencies;
* calling application services;
* returning serializable responses.

Commands should remain thin.

### Application layer

Application services coordinate use cases.

Examples include:

* creating a project;
* updating project metadata;
* listing projects;
* archiving a project;
* updating application settings.

### Domain layer

The domain layer contains:

* entities;
* value objects;
* domain validation;
* repository contracts;
* business invariants.

The domain must not depend on React, Tauri or SQLite.

### Infrastructure layer

The infrastructure layer contains technical implementations such as:

* SQLite repositories;
* database initialization;
* migrations;
* database path resolution;
* persistence error translation.

---

## Repository structure

```text
vertica/
├── docs/
│   ├── ADR/
│   ├── 01_SDD.md
│   ├── 02_AI_RULES.md
│   ├── 03_DEVELOPMENT_WORKFLOW.md
│   ├── 04_PROJECT_JOURNAL.md
│   ├── 05_BACKLOG.md
│   └── 06_DATA_SCHEMA.md
├── public/
├── scripts/
├── src/
│   ├── features/
│   ├── i18n/
│   ├── lib/
│   ├── App.css
│   ├── App.tsx
│   ├── index.css
│   └── main.tsx
├── src-tauri/
│   ├── capabilities/
│   ├── src/
│   │   ├── application/
│   │   ├── commands/
│   │   ├── domain/
│   │   └── infrastructure/
│   ├── Cargo.toml
│   └── tauri.conf.json
├── tests/
├── package.json
└── README.md
```

---

## Current implemented features

### Application information

The application exposes basic runtime information through a typed Tauri command.

### Database diagnostics

The application can inspect its current persistence state, including:

* database reachability;
* schema version;
* pending migrations;
* foreign-key enforcement;
* journal mode;
* database path.

### Application settings

The settings system currently supports:

* theme preference:

  * system;
  * light;
  * dark;
* interface language:

  * English;
  * Brazilian Portuguese.

Settings are stored in SQLite and restored when the application starts.

---

## Planned domain model

The central aggregate of Vertica will be `Project`.

An initial project model is expected to contain fields such as:

```text
Project
├── id
├── name
├── description
├── status
├── color or visual identity
├── start date
├── target date
├── created at
├── updated at
└── archived at
```

Later iterations may introduce project modules such as:

```text
ProjectModule
├── id
├── project id
├── module type
├── position
├── enabled
├── configuration
├── created at
└── updated at
```

The final schema must be defined in `docs/06_DATA_SCHEMA.md` before implementation.

---

## Development prerequisites

The project requires:

* Node.js;
* pnpm;
* Rust;
* Cargo;
* Tauri system dependencies for the host operating system.

The repository includes PowerShell scripts to help initialize and validate the local environment.

---

## Installing dependencies

```bash
pnpm install
```

---

## Running the frontend

```bash
pnpm dev
```

This starts the Vite development server without launching the desktop runtime.

---

## Running the desktop application

```bash
pnpm tauri dev
```

This starts the React frontend and the Tauri desktop application.

---

## Building the application

```bash
pnpm tauri build
```

The generated installer or executable depends on the current operating system and Tauri configuration.

---

## Running frontend checks

Use the scripts defined in `package.json`.

Typical commands may include:

```bash
pnpm build
```

```bash
pnpm lint
```

The exact available commands should be verified in `package.json`.

---

## Running Rust tests

From the repository root:

```bash
cd src-tauri
cargo test
```

The backend test suite currently covers the implemented settings and persistence behavior.

---

## Database migrations

SQL migrations are stored in:

```text
src-tauri/src/infrastructure/persistence/migrations/
```

Migration files use a sequential naming convention:

```text
0001_initialize_persistence_foundation.sql
0002_create_application_settings.sql
0003_create_projects.sql
```

The third migration is planned and has not yet been implemented.

Migration rules:

* never modify an already released migration to change existing behavior;
* introduce schema changes through a new migration;
* keep migrations deterministic;
* preserve existing user data whenever possible;
* enable and respect foreign-key constraints.

---

## Internationalization

The frontend uses a custom internationalization provider.

Current supported languages:

* `en`
* `pt-BR`

Translation dictionaries are located in:

```text
src/i18n/translations.ts
```

Visible interface text should use translation keys rather than inline language conditionals.

---

## Themes

The application supports:

* system theme;
* light theme;
* dark theme.

The selected theme is persisted in SQLite and applied to the document root.

---

## Documentation

The main project documents are:

```text
docs/01_SDD.md
```

Software design description and product definition.

```text
docs/02_AI_RULES.md
```

Rules for AI-assisted implementation.

```text
docs/03_DEVELOPMENT_WORKFLOW.md
```

Development, validation and delivery workflow.

```text
docs/04_PROJECT_JOURNAL.md
```

Chronological record of project decisions and completed work.

```text
docs/05_BACKLOG.md
```

Prioritized implementation backlog.

```text
docs/06_DATA_SCHEMA.md
```

Database entities, columns, constraints and relationships.

```text
docs/ADR/
```

Architectural decision records.

---

## Architectural decision records

Current ADRs:

```text
ADR-0001-architecture-overview.md
ADR-0002-technology-stack.md
ADR-0003-repository-structure.md
ADR-0004-persistence-strategy.md
```

The project-centered domain decision should be documented in:

```text
ADR-0005-project-centric-domain-model.md
```

This ADR should define:

* `Project` as the central aggregate;
* separation between global settings and project-owned data;
* modular project capabilities;
* independence from study-specific or examination-specific concepts;
* incremental migration of existing prototypes into domain modules.

---

## Near-term roadmap

### Foundation

* [x] Scaffold Tauri, React and TypeScript application
* [x] Establish layered backend architecture
* [x] Configure SQLite
* [x] Implement migration infrastructure
* [x] Add application information
* [x] Add database diagnostics
* [x] Add persisted application settings
* [x] Add theme support
* [x] Add English and Brazilian Portuguese support

### Project management

* [ ] Define the `Project` domain model
* [ ] Define the projects database schema
* [ ] Create the projects migration
* [ ] Implement the project repository contract
* [ ] Implement the SQLite project repository
* [ ] Implement project application services
* [ ] Expose project Tauri commands
* [ ] Add typed TypeScript clients
* [ ] Create the project list screen
* [ ] Create the project form
* [ ] Add project details and editing
* [ ] Add archive and delete workflows
* [ ] Add project tests

### First project module

* [ ] Define the content and progress module
* [ ] Add project categories
* [ ] Add content items or topics
* [ ] Track completion stages
* [ ] Store notes
* [ ] Calculate project progress
* [ ] Display module metrics

### Future modules

* [ ] Activity tracking
* [ ] Schedules and milestones
* [ ] Metrics and dashboards
* [ ] Notes
* [ ] Files and attachments
* [ ] Health and physical-performance tracking
* [ ] Import and export
* [ ] Backup and restore

---

## Contribution guidelines

Before implementing a feature:

1. review the relevant design documentation;
2. verify whether an ADR is required;
3. update the data schema when persistence changes;
4. implement the domain and application behavior before UI-specific logic;
5. add or update tests;
6. run frontend and backend validation;
7. update the project journal and backlog.

Implementation should preserve the architectural boundaries described in this repository.

---

## License

No license has been defined yet.

Until a license is added, the repository should be treated as privately owned and not automatically reusable or redistributable.
