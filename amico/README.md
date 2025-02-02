# Amico AI Agent Framework

Amico is the next generation Autonomous AI Agent Framework tailored for embedded AI devices and multi-agent systems.

## Architecture Overview

### Model-Based Agents

1. **Basic Design**
   ![Basic Design](https://github.com/AIMOverse/amico/blob/main/images/amico_basic_design.png)
2. **State Representation**: The state agent acquires the current state of the environment through sensors and represents it. This state describes the specific situation of the current world, such as the attributes of location, resources, or objects.
3. **World Evolution**: Predicts the impact of actions.
4. **Condition-Action Rules**: Module for decision-making.

### Modules

1. **amico-core**: Defines the underlying workflow (Event -> Select Action -> Execute Action) and Model (Perceive Environment and Sense Environment).
2. **amico-plugins**: Community plugins.
3. **amico-std**: Built-in standard plugins.
4. **amico-firmware**: Hardware control.
5. **amico-macros**: Procedural macros.

## Quick Start

### Install & Run Amico Immediately

```bash
cargo install amico

# This will run Amico using the default config.
amico
```

### Configuration
