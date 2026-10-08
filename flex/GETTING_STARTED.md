# Monolith in Flexscript

This is the first native, database-backed Monolith prototype. Its application
compiler, HTTP server, schema tooling, UI renderer, diff engine, SQLite adapter,
and integration harness are written in current Flexscript. The browser uses a
small JavaScript DOM adapter bundled into the executable by the build tool.

The original Rust prototype remains in the repository for reference. This
implementation lives entirely under `flex/` and has no source imports from a
Flexscript checkout. Supply a compiler executable when building.

## Build and run

Requirements: Linux x86-64, a Flexscript compiler with imports and C FFI
(released 0.0.5 or newer), glibc, and `libsqlite3.so.0`. No Rust, Cargo, Node,
Python, JavaScript bundler, or C compiler is needed for the app or native tests.

From this directory, substituting your compiler's absolute path:

```sh
/path/to/flex tools/build.flex -o build/build
build/build /path/to/flex
build/todo 8080 build/todo.db
```

Open <http://127.0.0.1:8080/> in two windows. Add, complete, and delete tasks in
one; the other updates automatically. Typing, focus, and selection remain local.
The database survives server restarts. The server binds only to loopback.

To rebuild just the app, including its schema, stylesheet and browser runtime:

```sh
build/monolith build examples/todo/app.flex \
  --schema examples/todo/schema.mono --compiler /path/to/flex -o build/todo
```

The compiler writes `build/todo.generated.flex` and compiles a temporary binary,
then replaces the executable atomically. Rebuilding is allowed while the old
binary runs; restart the server to use the new code. Invalid schema or compiler
errors leave the previous executable intact.

## Application sources

- `examples/todo/schema.mono`: one model with an automatic integer primary key,
  bounded text, and boolean fields. Generates DDL and parameterized CRUD queries.
- `examples/todo/app.flex`: declarative UI builders and transactional handlers.
- `src/ui.flex`: stable keys, HTML rendering, and property/insert/remove/move patches.
- `src/server.flex`: HTTP requests, action validation, SSE subscriptions, reconnect
  snapshots, and database invalidation.
- `src/db.flex`: SQLite FFI and persistent schema metadata.
- `src/client.js`: delegated browser events, local drafts, keyed patch application.
- `tools/monolith.flex`: application compiler and asset embedding.
- `tests/integration.flex`: real compiler/server/database integration tests.

The builder API is valid Flexscript today. Records, closures, typed model access,
and a richer declarative syntax are future language/compiler work.

## Tests

```sh
build/test-integration /path/to/flex
build/test-browser build/todo /usr/bin/chromium
```

The harness builds isolated binaries/databases, obtains a temporary loopback
port, and tests schema errors, SSR, real SQLite storage, two simultaneous live
clients, precise patches, external database writes, UTF-8, HTML escaping, SQL
injection, rejected actions/requests, reconnect snapshots, restart persistence,
and schema drift. Child servers terminate when the harness exits.

The optional browser harness is also written in Flexscript. It starts an
isolated headless Chromium profile and temporary Todo server, drives two tabs
through Chromium's DevTools protocol, and verifies live changes, a server
restart, changed database contents, draft/focus/selection preservation, and mobile
layout. It saves `build/todo-desktop.png` and `build/todo-mobile.png`. Supply the
path to Chromium or Google Chrome; browser tests also need `libcrypto.so.3`.
The `Flexscript Monolith` GitHub Actions workflow runs both harnesses on pushes
and pull requests to `trunk`, and uploads the binary and screenshots.

## Current boundaries

This is a shared Todo demo without accounts or per-user authorization. It has
500-task, 64-connection, 32-live-client, 8-KiB-header and 4-KiB-body limits.
Requests time out after two seconds; stalled live clients are disconnected.
The server accepts one request per ordinary HTTP connection. SSE streams stay
open, with heartbeats and full keyed snapshots on reconnect. HTTPS termination,
authentication, deployment, native/mobile renderers, and offline writes are not
implemented here.

Actions require a random process token and reject foreign Origin and Host
headers. The token rotates on restart and reconnect updates it automatically.
SQL values are bound parameters; HTML and patch strings are escaped. This demo's
schema metadata refuses implicit migrations or adoption of an existing table.
An explicit migration engine is future work.

Committed application writes invalidate the shared view immediately. Changes
made through other SQLite connections are detected with `PRAGMA data_version`
once per second. Dependency tracking is coarse at this stage: the view is
recomputed, then keyed diffs minimize browser updates.

The compiler currently embeds the complete small DOM adapter and application
assets. It does not yet specialize UI render functions, eliminate unused client
features, transpile local Flexscript handlers, or support per-client data views.
