default:
  just --list
  
run:
  cargo run

[working-directory: 'templates']
asset-serving:
    pnpm dlx @tailwindcss/cli -i styles/tailwind.css -o assets/main.css --watch

[working-directory: 'templates']
format:
    pnpm format

[working-directory: 'templates']
lint:
    pnpm lint