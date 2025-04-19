# JamesScape

A RuneScape-style MMORPG built with Rust and Bevy.

## Project Overview

JamesScape is a RuneScape-inspired MMORPG with a focus on skill-based progression, open-world exploration, player economy, and quest-driven gameplay. The game maintains the core appeal of RuneScape while implementing modern game development practices.

## Technology Stack

- **Primary language**: Rust
- **Game engine**: Bevy
- **Networking**: Renet
- **UI**: Bevy EGUI
- **Database**: (To be implemented)

## Core Systems

1. **Player & Character Systems**
   - Character creation with customization options
   - Skill-based progression system (20+ skills similar to RuneScape)
   - Experience and leveling mechanics
   - Inventory system with equipment slots and bank storage

2. **World & Environment**
   - World generation with distinct regions
   - Resource nodes (mining spots, fishing areas, etc.)
   - Day/night cycle and weather effects
   - Instanced dungeons and special areas

3. **Combat System**
   - Three combat styles: melee, ranged, and magic
   - Combat level calculation based on skill levels
   - PvE and controlled PvP zones
   - Monster AI with varying difficulty levels

4. **Quest System**
   - Multi-stage quests with branching dialogues
   - Quest rewards affecting world state
   - Quest log and tracking
   - Achievement system

5. **Economy System**
   - Resource gathering mechanics
   - Crafting system with recipes and skill requirements
   - Player-to-player trading
   - Grand Exchange/auction house functionality

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/jamesscape.git
   cd jamesscape
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the game:
   ```
   cargo run
   ```

## Development Roadmap

- **Phase 1**: Foundation & Core Mechanics
- **Phase 2**: World Building & Content
- **Phase 3**: Systems Integration & Economy
- **Phase 4**: Social Features & Polishing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by RuneScape
- Built with Bevy Engine
