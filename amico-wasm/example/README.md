# Amico WASM Example

This example demonstrates how to use the Amico WASM package in a React + Vite application.

## Features

- Integration with Amico WASM package
- Button to call the WASM `start()` function
- Simple UI showing WASM module loading status

## Getting Started

- Make sure the WASM package is built in the `amico-wasm/pkg` directory

```bash
# in amico project root
wasm-pack build amico-wasm
cd amico-wasm/examples
```

- Install dependencies:

```bash
# in amico-wasm/examples
pnpm install
```

- Run the development server:

```bash
# in amico-wasm/examples
pnpm dev
```

- Open your browser at the provided URL.

## How It Works

The example imports the WASM module from the local package and provides a button to call the `start()` function. The UI shows the loading status of the WASM module and prevents calling the function until the module is fully loaded.

Check the browser console for logs about the WASM module initialization and function calls.

## Technical Notes

- The WASM package is imported directly from the local `../pkg` directory
- The example uses React hooks to manage the WASM loading state
- Error handling is implemented for both loading and function calls
- Uses `vite-plugin-wasm` and `vite-plugin-top-level-await` to handle WebAssembly integration
- Dynamic imports are used to properly load the WASM module asynchronously

## Vite Configuration

The `vite.config.js` file has been configured with the necessary plugins:

```javascript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    react()
  ],
})
```
