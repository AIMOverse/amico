# Amico AI Agent Framework

Amico is a next-generation Autonomous AI Agent Framework designed for embedded AI devices and multi-agent systems.

## Detailed Documentation

See our [Document Site](https://www.amico.dev) for more information.

## Modules

1. **`amico`**: The main executable crate.
2. **`amico-core`**: Interfaces and workflows for the Engine Layer.
3. **`amico-sdk`**: Interfaces and workflows for the AI Agent and Interaction Layers.
4. **`amico-mods`**: Pluggable implementation modules.
5. **`amico-hal`**: Hardware Abstraction Layer.

## Development Plans

- **Model Context Protocol (MCP) Integration**: Improves environmental awareness and control.
- **Agent Networking**: Supports peer-to-peer networking using Web3 technologies.
- **WASM Support**: Supports WASM-based AI Agent runtime.

## Architecture Overview

### Core Concepts

![Framework](https://raw.githubusercontent.com/AIMOverse/amico/refs/heads/main/images/framework-v2.png)

#### Framework Layers

- **Interaction Layer**: Manages communication between agents and the environment. This includes:

  - **Sensors**: Acquire the current state of the environment.
  - **Effectors**: Execute actions.
  - The environment can be physical (real-world) or virtual (the Internet, blockchain, etc.).
  - Hardware sensor and effector drivers are implemented in the `amico-firmware` crate.
  - **Future Plans**: Decoupling into:
    - **Environment Layer**: Passively receives/responds to environmental inputs.
    - **Interaction Layer**: Actively handles actions and state changes from users and agents.

- **Agent Layer**: Encapsulates core agent logic, including state management, decision-making, and action execution. Key components:

  - **LLM Providers** and **RAG Systems** implemented as plugins.
  - **Task execution models** (see _Model-Based Agents_ below) implemented in `amico-std`, with plugin support for custom models.

- **Engine Layer**: Handles task scheduling, event generation, and action selection. The default **Action Selector** is in `amico-std`, but custom implementations can be added via plugins.

#### Pluggable Modules

- **LLM Services**: Provides content generation, integrating LLM calls, RAG knowledge base, tool calling, etc.
- **LLM Providers**: API integrations for services like OpenAI, DeepSeek, etc.
- **Effectors**: Execute actions such as hardware control, transactions, content posting, and messaging.
- **Sensors**: Capture environmental data, such as sensor readings and social media content.
- **Hardware Abstraction**: Low-level interface for embedded device interaction.

#### Low-Level Plugins

- **RAG Systems**: Implements retrieval-augmented generation.
- **Task Executors**: Provides task execution workflows (e.g., Model-Based Agents).
- **Action Selectors**: Implements action selection algorithms.
- **Event Generators**: Generates events based on the current environment state.

### Model-Based Agents: Basic Design

![Basic Design](https://raw.githubusercontent.com/AIMOverse/amico/refs/heads/main/images/model_based.png)

- **State Representation**: The agent perceives and represents the current environment state.
- **World Evolution**: Predicts the impact of actions.
- **Condition-Action Rules**: Guides decision-making.

### Task Execution Workflow

![Task Execution Workflow](https://raw.githubusercontent.com/AIMOverse/amico/refs/heads/main/images/task_exec.png)

- **Event-Triggered Tasks**

  - Tasks initiate based on events (e.g., timers, on-chain/off-chain signals, messages from other agents).
  - Each event carries context, providing additional knowledge for decision-making.

- **Knowledge Acquisition**

  - The agent gathers information from its internal knowledge base and real-time data sources.
  - Information is synthesized into a comprehensive report to support decision-making.

- **Decision Making**

  - The agent evaluates potential actions and selects the most informed response.
  - Possible responses include executing a task, responding to a user, or both.
  - In SWARM systems, agents may seek consensus before executing critical actions.

- **Execution of Decision**

  - Actions can range from executing transactions to posting content.
  - If required, the agent communicates with other agents before execution.

- **Agent Response**
  - Provides human-readable feedback after execution.
  - Responses may include tool calls for embedded devices (e.g., triggering a motor).

## License

Amico is released under the [MIT License](https://raw.githubusercontent.com/AIMOverse/amico/main/LICENSE).

## Contributing

Contributions are welcome! Please read our [contributing guidelines](https://raw.githubusercontent.com/AIMOverse/amico/main/CONTRIBUTING.md) before submitting a pull request.
