# Amico AI Agent Framework

Amico is the next generation Autonomous AI Agent Framework tailored for embedded AI devices and multi-agent systems.

## Getting Started

**This is only the prototype version of Amico.** The Amico runtime is currently a command-line chatbot for testing our SDK. The integration of the Engine Layer is still in progress, but refactorings and integrations of the Agent Layer and Interaction Layer are already in progress.

### Clone the Repository

```bash
git clone https://github.com/AIMOverse/amico.git
cd amico
```

### Run the Runtime

```bash
export OPENAI_API_KEY=your_api_key

# If you want to configure a custom base URL for OpenAI
# export OPENAI_BASE_URL=your_base_url

# Use a helius api key for Solana actions.
# We recommend you to use Helius API for on-chain actions.
# The default Solana RPC is not stable enough.
# Check out https://helius.dev for more information.
export HELIUS_API_KEY=your_api_key

cargo run -p amico
```

This will create a wallet for the agent. The bip39 seed phrase will be saved in the `agent_wallet.txt` file.

Now you will get a command-line chatbot interface. You can develop your plugins or expand Amico's functionality and test them in the runtime.

```txt
$ cargo run -p amico
   Compiling amico-sdk v0.0.2 (/home/.../amico/amico-sdk)
   Compiling amico-plugins v0.1.1 (/home/.../amico/amico-plugins)
   Compiling amico v0.0.1 (/home/.../amico/amico)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 08s
     Running `target/debug/amico`
This is only a PROTOTYPE VERSION of Amico.
Check out our docs for more information:
https://www.amico.dev

Found OPENAI_API_KEY
Found OPENAI_BASE_URL
Found HELIUS_API_KEY

Agent wallet addresses:
- Solana: F1t15xYmLqrALX45p9XVpvwxcRsjXKq676xDWEFazUYD
- Ethereum: 0xA7283cb5A2Fc2766674325FA6a9f5711aC8f1b63

Using service plugin: StdInMemoryService
Tools enabled:
- create_asset: Create a NFT on Solana representing yourself
- buy_solana_token: Buy a Solana token
- check_ethereum_balance: Check ETH balance on Ethereum in your own wallet
- check_solana_balance: Check SOL balance on Solana in your own wallet
- search_for_jokes: Search for jokes


I'm Amico, your personal AI assistant. How can I assist you today?
--------------------
Enter your message
>
```

## Architecture Overview

### Framework Modules

![Framework](https://raw.githubusercontent.com/AIMOverse/amico/refs/heads/main/images/framework-v2.png)

- **Framework Layers**

  - The **Interaction Layer** manages the communication between agents and the environment. In this layer, **sensors** are used to acquire the current state of the environment, and **effectors** are used to execute actions. The environment the layer interacts to is not only real-world but also virtual environments like the Internet or a block chain. The drivers for real-world hardware sensors and effectors are implemented in `amico-firmware` crate. **In the future, we aims to further decouple the current interaction layer into:**
    - The **Environment Layer**, which passively receive/respond to the environment all the time without inter needed.
    - The **Interaction Layer**, which actively receive/respond to users/agents' actions and state.
  - The **Agent Layer** encapsulates the core logic of the agent, including state management, decision-making, and action execution. The concrete **LLM Providers** and **RAG Systems** are implemented in plugins. The framework provides several **Task execution model** (see the _Model-Based Agents_ section below) implementations in the `amico-std` crate, but you can also write your own implementations in plugins.
  - The **Engine Layer** implements the core logic of task scheduling, event generation and action selection based on events. The framework provides an implementation of **Action Selector** based on mapping in the `amico-std` crate, but you can also write your own implementations in plugins.

- **Plugins**

  - **Effectors**: Perform actions like hardware module control, transaction execution, content posting, sending messages to other agents, etc.
  - **Sensors**: Acquire the current state of the environment like sensor reading, social media content reading, receiving messages from other agents, etc.
  - **LLM Providers**: Providing API access to LLM services like OpenAI, DeepSeek, etc.
  - **Firmware Drivers**: Providing a low-level interface for interacting with embedded devices.

- **Low-Level Plugins**

  - **RAG Systems**: Providing a retrieval-augmented generation system.
  - **Task Executors**: Providing a task execution workflow, like Model-Based Agents described below.
  - **Action Selectors**: Providing an action selection algorithm, to select the most appropriate action given the current state and the available actions.

### Model-Based Agents

![Basic Design](https://raw.githubusercontent.com/AIMOverse/amico/refs/heads/main/images/model_based.png)

- **State Representation**: The state agent acquires the current state of the environment through sensors and represents it. This state describes the specific situation of the current world, such as the attributes of location, resources, or objects.
- **World Evolution**: Predicts the impact of actions.
- **Condition-Action Rules**: Module for decision-making.

### Task Execution Workflow

![Task Execution Workflow](https://raw.githubusercontent.com/AIMOverse/amico/refs/heads/main/images/task_exec.png)

- **Event-Triggered Task**

  - Tasks are triggered by various "events", such as timers, major on-chain or off-chain events, or signals from other agents.
  - Each event carries context, the information of the event in natural language, which is then used as an additional knowledge source when the agent gathers information.

- **Knowledge Acquisition**

  - The agent collects relevant knowledge from its internal knowledge base as well as the context of the event.
  - If needed, the agent can also acquire real-time data sources from both on-chain and off-chain environments.
  - The agent synthesizes all these informations into a comprehensive report to guide its decision-making process.

- **Decision Making**

  - Using the knowledge report, the agent evaluates possible actions and makes fully-informed decisions.
  - The agent can either respond to the user, execute a task, or do both.
  - For critical decisions, the agent may optionally seek consensus from other agents to ensure the reliability of the decision in a SWARM-system environment.

- **Execution of Decision**

  - The agent carries out the chosen action, which could range from executing a transaction to posting content (e.g., a tweet).
  - If the action requires consensus, the agent will optinally first communicate with other agents before proceeding.

- **Agent Response**

  - Following execution, the agent can provide feedback to the user in human-readable way.
  - This response could also include instructions (tool calls) for embedded devices, such as triggering a motor or adjusting the environment in some way.

## Modules

1. **`amico-core`**: Engine layer interfaces and workflows.
2. **`amico-sdk`**: AI Agent layer and Interaction layer interfaces and workflows.
3. **`amico-mods`**: Pluggable implementation modules that actually implement the interfaces.
4. **`amico-hal`**: Hardware abstraction layer.

## Future Improvements

- **Enhanced decision logic**: Investigate support for reinforcement learning-based decision-making within
  `ActionSelector`.
- **Plugin security**: Strengthen security for dynamically loaded plugins using WebAssembly (WASM) or sandboxing
  techniques.

## License

AMICO is released under the [MIT License](https://raw.githubusercontent.com/AIMOverse/amico/main/LICENSE).

## Contributing

Contributions are welcome! Please read
our [contributing guidelines](https://raw.githubusercontent.com/AIMOverse/amico/main/CONTRIBUTING.md) before submitting
a pull request.
