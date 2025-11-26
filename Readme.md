# OpenDevicePartnership.github.io

Website for the Open Device Partnership, built with Rust, Leptos, and Trunk.

## Prerequisites

Before running this project, ensure you have the following installed:

1. **Rust** - Install from [https://rustup.rs/](https://rustup.rs/)

2. **Trunk** - Build tool for Rust web applications

   ```cmd
   cargo binstall trunk or
   cargo install trunk
   ```

3. **WebAssembly target** - Required for compiling to WASM

   ```cmd
   rustup target add wasm32-unknown-unknown
   ```

## Development

To run the development server:

```cmd
trunk serve
```

This will:

- Build the project in debug mode
- Start a development server at <http://127.0.0.1:3000>
- Watch for file changes and auto-reload the browser

## Production Build

To build for production:

```cmd
trunk build --release
```

The optimized output will be generated in the `dist/` directory.

## Project Structure

- `src/` - Rust source code
  - `components/` - Reusable UI components
  - `pages/` - Page components
- `style/` - CSS files (Tailwind)
- `public/` - Static assets (images, etc.)
- `index.html` - Main HTML template
- `Trunk.toml` - Trunk configuration
- `Cargo.toml` - Rust dependencies

## Technology Stack

- **Leptos** - Reactive web framework for Rust
- **Trunk** - WASM web application bundler
- **Tailwind CSS** - Utility-first CSS framework
