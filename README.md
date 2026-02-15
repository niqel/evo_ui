# evo_ui

## Manual tests

1. Proyecto vacío (sin `ui.toml`) y fallback del engine:

```bash
cargo run -- create --name demo_empty --output /tmp --empty
cargo run -- run /tmp/demo_empty
```

Esperado: abre la ventana en modo fallback (fondo negro) sin crashear.

2. Proyecto con fondo verde mínimo:

```bash
cargo run -- create --name demo_green --output /tmp --bg-green
cargo run -- run /tmp/demo_green
```

Esperado: abre una ventana con solo el fondo verde.
