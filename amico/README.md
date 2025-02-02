# AMICO - AI Agent Framework for DePin Devices

## Overview

AMICO is an AI Agent Framework designed for DePin devices. It provides a modular, extensible architecture that enables
intelligent agents to perceive their environment, process events, and execute actions. The framework is highly
customizable, allowing users to configure input sources, event generation, action selection, and execution logic to fit
specific use cases.

## Modules

AMICO consists of four primary modules:

1. **amico-core**: Defines the underlying workflow, handling events, selecting actions, and executing them.
2. **amico-firmware**: Provides hardware control capabilities.
3. **amico-macros**: Implements procedural macros for enhancing development efficiency.
4. **amico-plugins**: Supports plugin-based extensions to enhance functionality.

## Architecture

The core architecture of AMICO follows an event-driven workflow:

1. **Inputs**: Input sources such as cameras, sensors, and microphones provide raw data.
2. **Event Generation**: The `EventGenerator` processes input data to create relevant events, which may have expiration
   times.
3. **Event Pool**: Stores generated events until they expire or are processed.
4. **Action Selection**: The `ActionSelector` reads unexpired events from the event pool and determines the most
   suitable action.
5. **Execution**: The selected action is executed by the agent.

### Key Features

- High flexibility: Users can define and modify `Inputs`, `EventGenerator`, `Action`, and `ActionSelector` to fit custom
  requirements.
- Adaptive decision-making: The framework supports a condition-action rules approach and can be extended with
  reinforcement learning techniques (e.g., Q-learning, DQN).
- Modular design: The plugin system allows users to extend functionalities seamlessly.

## Quick Start

### Install & Run AMICO Immediately

```bash
cargo install amico

# This will run AMICO using the default config.
amico
```

## Future Improvements

- **Event expiration mechanism**: Implement adaptive expiration strategies (e.g., priority queues, sliding window
  expiration) to optimize performance.
- **Enhanced decision logic**: Investigate support for reinforcement learning-based decision-making within
  `ActionSelector`.
- **Plugin security**: Strengthen security for dynamically loaded plugins using WebAssembly (WASM) or sandboxing
  techniques.

## Repository Links

- [amico-core](https://github.com/AIMOverse/amico/tree/main/amico-core)
- [amico-firmware](https://github.com/AIMOverse/amico/tree/main/amico-firmware)
- [amico-macros](https://github.com/AIMOverse/amico/tree/main/amico-macros)
- [amico-plugins](https://github.com/AIMOverse/amico/tree/main/amico-plugins)

## License

AMICO is released under the [MIT License](https://raw.githubusercontent.com/AIMOverse/amico/main/LICENSE).

## Contributing

Contributions are welcome! Please read
our [contributing guidelines](https://raw.githubusercontent.com/AIMOverse/amico/main/CONTRIBUTING.md) before submitting
a pull request.

