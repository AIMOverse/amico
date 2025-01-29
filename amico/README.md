# Amico AI Agent Framework

![Rust](

Amico is an AI Agent Framework designed for DePin devices.

## Core Focus

- **Configurable**
- **Extensible**
- **Modular**
- **Scalable**

## Architecture V1

### Type: Model-Based Agent

#### Design

1. **Basic Design**
   ![Basic Design](pictures/amico_basic_design.png)
2. **State Representation**: The state agent acquires the current state of the environment through sensors and represents it. This state describes the specific situation of the current world, such as the attributes of location, resources, or objects.
3. **World Evolution**: Predicts the impact of actions.
4. **Condition-Action Rules**: Module for decision-making.
   ![Condition-Action Rules](path_to_image)

### Modules

1. **amico-core**: Defines the underlying workflow (Event -> Select Action -> Execute Action) and Model (Perceive Environment and Sense Environment).
2. **amico-firmware**: Hardware control.
3. **amico-macros**: Procedural macros.
4. **amico-plugins**: Plugins.

## Quick Start

### Install & Run Amico Immediately

```bash
cargo install amico

# This will run Amico using the default config.
amico