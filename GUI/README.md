# Chip Layout Viewer — Dioxus 0.7.9

Web-frontend displaying placed `cells` from a chip layout JSON. Coordinates `x/y`
mean the lower-left corner of each cell, in the chip's lower-left coordinate system.
The app converts to SVG's top-left coordinate system.

## Requirements

- Rust, rustup and `wasm32-unknown-unknown` target
- Dioxus CLI (`dx`) compatible with 0.7.x

## Start backend

From project root:

```bash
cargo run --manifest-path server/Cargo.toml
```

Actix Web listens on `127.0.0.1:3001`. It exposes **one API path**, `/api/layout`, with two methods:

- `GET /api/layout` — return the last submitted layout (example preloaded on startup)
- `POST /api/layout` — replace it with JSON, return HTTP 204

The backend is a development/demo implementation. Data is only stored in memory:
restarting the backend resets the example. It checks only that `design` is an object
and `cells` is an array; the frontend does typed JSON deserialization.

## Start frontend

In another terminal, from project root:

```bash
rustup target add wasm32-unknown-unknown
dx serve --web
```

Open the local URL printed by `dx` (commonly http://127.0.0.1:8080).
Dioxus development proxy configured in `Dioxus.toml` forwards `/api/*`
to `http://127.0.0.1:3001/api/*`, avoiding cross-origin browser requests.

## Submit JSON from your placement algorithm

```bash
curl -i -X POST http://127.0.0.1:3001/api/layout \
  -H 'Content-Type: application/json' \
  --data-binary @examples/layout.json
```

Now click **«Обновить данные»** in the web interface. Updates are manual; there
is no WebSocket/polling in this starter implementation.

## Important schema detail

`cells[]` in the original input only include `id`, `width`, `height`, `type`,
`priority`. You must add `x` and `y` to each placed cell; otherwise the renderer
intentionally skips it. The fields are `Option<f64>` to keep the original JSON
compatible (unplaced cells are counted, but not painted).

```json
{"id":"C1", "x":150, "y":80, "width":30, "height":40, "type":"NAND2", "priority":3}
```

Cell coordinates: `(x,y)` **lower-left corner**. SVG's y-axis points downward,
so a rectangle's top-left y is computed as
`svg_y = chip_height - cell_y - cell_height`.

The UI renders only `cells`; `rows`, `blockages`, `nets`, `pad_locations`, and
`tetris_params` are deserialized, not rendered. Grid/background and chip bounds
are visualization guides, not algorithm entities.

## Project layout

```text
.
├── Cargo.toml
├── Dioxus.toml
├── assets/main.css
├── src/
│   ├── main.rs                   # API loader and app shell
│   ├── model.rs                  # Deserializable layout structures
│   └── components/
│       ├── mod.rs
│       ├── canvas.rs             # SVG canvas, zoom, pan, cell rectangles
│       └── inspector.rs          # Cell properties and summary
├── examples/layout.json        # Demonstration input
└── server/
    ├── Cargo.toml
    └── src/main.rs              # Actix Web GET and POST on one path
```

## Production considerations

- Serve compiled frontend assets and `/api/layout` via the same origin, e.g. a
  reverse proxy, or configure an explicit CORS policy and API URL.
- Add backend payload schema validation, size limits / authentication as needed.
- For live updates without clicking Refresh, add SSE/WebSocket/polling.
- Browser SVG can be slow with a very large number of cells; consider canvas/WebGL
  if the design reaches hundreds of thousands of elements.

Note: Source files have not been cargo-compiled in this artifact generation environment;
please verify with `cargo check` / `dx serve` in your Rust development environment.
