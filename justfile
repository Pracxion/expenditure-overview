default:
  just --list

run:
  cargo run

watch:
  RUST_LOG=info cargo watch -x run

format-rust:
  cargo +nightly fmt

[working-directory: 'templates']
asset-serving:
    pnpm dlx @tailwindcss/cli -i styles/tailwind.css -o assets/main.css --watch

[working-directory: 'templates']
format-templates:
    pnpm format

[working-directory: 'templates']
lint:
    pnpm lint

[working-directory: 'templates']
organize:
    rustywind --write .

[working-directory: 'templates']
organize-all:
    @organize
    @format-templates
    @format-rust
    @lint
