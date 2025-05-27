import { useState, useEffect } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
  const [count, setCount] = useState(0)
  const [wasmLoaded, setWasmLoaded] = useState(false)
  const [wasmModule, setWasmModule] = useState(null)

  useEffect(() => {
    const initWasm = async () => {
      try {
        // Dynamic import of WASM module
        const wasm = await import('amico-wasm')
        console.log('WASM module loaded:', wasm)
        setWasmModule(wasm)
        setWasmLoaded(true)
      } catch (error) {
        console.error('Failed to initialize WASM:', error)
      }
    }

    initWasm()
  }, [])

  const handleStartWasm = () => {
    if (!wasmModule) return

    try {
      wasmModule.start()
      console.log('WASM start function called successfully')
    } catch (error) {
      console.error('Error calling WASM start function:', error)
    }
  }

  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Amico WASM Example</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>

        <div style={{ marginTop: '20px' }}>
          <button
            onClick={handleStartWasm}
            disabled={!wasmLoaded}
            style={{
              backgroundColor: wasmLoaded ? '#646cff' : '#888',
              padding: '10px 20px',
              borderRadius: '8px',
              color: 'white',
              cursor: wasmLoaded ? 'pointer' : 'not-allowed'
            }}
          >
            {wasmLoaded ? 'Start WASM' : 'Loading WASM...'}
          </button>
          <p>
            {wasmLoaded
              ? 'WASM loaded successfully! Click the button to call the start() function.'
              : 'Loading WASM module...'}
          </p>
        </div>
      </div>
      <p className="read-the-docs">
        Amico WASM integration example
      </p>
    </>
  )
}

export default App
