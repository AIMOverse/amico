# Amico AI Agent Framework

Amico is the next generation Autonomous AI Agent Framework tailored for embedded AI devices and multi-agent systems.

## Getting Started

If you are running the `amico` executable directly, refer to [the Runtime Documentation](https://github.com/AIMOverse/amico/blob/main/amico/README.md)

### Creating Your Own Agent

First, create a new rust project:

```bash
cargo new my_agent --bin
```

Then, add `amico` to your project:

```bash
cargo add amico
```

## Architecture Overview

### Model-Based Agents

![Basic Design](https://github.com/AIMOverse/amico/blob/main/images/model_based.png)

1. **State Representation**: The state agent acquires the current state of the environment through sensors and represents it. This state describes the specific situation of the current world, such as the attributes of location, resources, or objects.
2. **World Evolution**: Predicts the impact of actions.
3. **Condition-Action Rules**: Module for decision-making.

### Event-Triggered Workflows

![Workflow](https://github.com/AIMOverse/amico/blob/main/images/task_exec.png)

- Tasks are triggered by various "events", such as timers, major on-chain or off-chain events, or signals from other agents.
- Each event carries context, the information of the event in natural language, which is then used as an additional knowledge source when the agent gathers information.

### Modules

1. **amico-core**: Defines the underlying workflow (Event -> Select Action -> Execute Action) and Model (Perceive Environment and Sense Environment).
2. **amico-plugins**: Community plugins.
3. **amico-std**: Built-in standard plugins.
4. **amico-firmware**: Hardware control.
5. **amico-macros**: Procedural macros.

## Future Improvements

- **Event expiration mechanism**: Implement adaptive expiration strategies (e.g., priority queues, sliding window
  expiration) to optimize performance.
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
