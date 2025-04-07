# Amico WASM

The WASM library of the Amico AI Agent Framework

## Getting Started

### Installing via npm

```bash
npm install @aimoverse/amico-wasm
```

### Using in a nodejs project

```javascript
import {
    createAmico,
    createProvider,
    createBalanceSensor,
    createTradeEffector,
    loadWallet,
    AMICO_DEFAULT_SYS_PROMPT,
} from "@aimoverse/amico-wasm";

// The agent instance
let agent;

// Setup the agent
async function setup() {
    // Create the provider
    const provider = createProvider({
        base_url: "<BASE_URL>",
        api_key: "<API_KEY>",
    });
    // Create the wallet
    const wallet = await loadWallet();
    // Create the balance sensor
    const balanceSensor = createBalanceSensor({ wallet });
    // Create the trade effector
    const tradeEffector = createTradeEffector({ wallet });
    // Create the agent
    agent = await createAmico({
        provider,
        model: "gpt-4o",
        system_prompt: AMICO_DEFAULT_SYS_PROMPT,
        temperature: 0.2,
        max_tokens: 1000,
        tools: [
            balanceSensor.tool(),
            tradeEffector.tool(),
        ],
    });
}

// Interact with the agent
async function interact() {
    const response = await agent.chat("Hello, how are you");
    console.log(response);
}
```

## License

MIT
