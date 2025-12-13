# Project Decisions – History-Driven Simulation Game

This document records all architectural and design decisions made for this project.

---

## Core Concept

**Decision**: Build a **simulation-first game** inspired by Dwarf Fortress world generation and "Hunger Games generators".

- A group of people is initialized with parameters (resources, items, traits, relationships).
- Time advances in **cycles** (8 cycles per day).
- Over weeks/months/years, entities act autonomously.
- The main output is **history**: logs, events, timelines, statistics.
- No focus on graphics initially; text/log output is sufficient.

**Philosophy**: The game is about **watching what happens**, not direct control.

---

## Architecture

### ECS / Data-Driven Design

**Decision**: Use an **Entity–Component–System (ECS)**-style architecture.

- **Entity**: just an ID.
- **Component**: pure data (Hunger, Inventory, Location, RotTimer, Health).
- **System**: logic that runs on entities with specific components.

**Rationale**:
- Scales well with many entities
- Avoids complex inheritance trees
- Easy to add/remove behaviors dynamically
- Deterministic and replay-friendly
- Very suitable for long-running simulations

### Backend / Frontend Split

**Decision**: Separate simulation core from UI/display layer.

**Simulation Core (Backend)**:
- Pure logic, no UI
- Owns the world state
- Deterministic (seeded RNG)
- API-like surface: `step(cycles)`, `inject_event(...)`, `get_events()`, `get_snapshot()`
- Can run headless (fast, no rendering)
- Easy to test and replay

**UI / Engine Layer (Frontend)**:
- Displays logs, charts, timelines
- Controls time (pause, step, fast-forward)
- Does **not** decide outcomes
- Can be swapped later (CLI → engine → web UI)

---

## Scale & Performance

### Target Scale

**Decision**: Design for **thousands of entities** from the start.

### Scheduling Model

**Decision**: Use **event-driven scheduling**, not polling.

- Do not update everything every cycle
- Use an **event scheduler** (priority queue or time buckets)
- Schedule future events once (e.g., food rots at cycle 120)
- Each cycle: process only events due now, wake only agents that need decisions

**Performance target**: O(number of agents + number of events), not O(agents × agents)

### Partitioning

**Decision**: Limit entity interactions through **partitioning**.

- Locations / containers (camp, room, district)
- Groups (faction, team, job site)
- Graphs (connected places with travel time)
- Subscriptions / relationships

Entities only consider *relevant* others.

---

## Persistence & Determinism

### Persistence

**Decision**: All state must be **persistent and serializable**.

**Rationale**: Future "rewind" feature must be possible. Design data structures with save/load in mind from the start.

### Determinism

**Decision**: Use **seeded RNG** for full determinism.

- All randomness flows from a single seed
- Same seed + same inputs = identical simulation
- Enables: sharing scenarios, debugging, replay verification

**Implementation**: One RNG instance passed through all systems.

---

## AI Architecture

### Hierarchical AI System

**Decision**: Use a **two-tier AI architecture**.

#### Individual Level: Utility-Based AI

**What**: Every possible action gets a utility score (0-100), agent picks highest.

**Example**:
```
Eat food: 90 points (very hungry)
Find food: 75 points (no food nearby)
Sleep: 20 points (not tired)
→ Chooses "Eat food"
```

**Rationale**:
- Simple to implement and debug
- Emergent behavior from simple rules
- Predictable performance: O(number of possible actions)
- Easy to tune scoring functions
- Handles competing needs naturally
- Scales to thousands of agents

#### Group Level: Goal-Oriented Action Planning (GOAP)

**What**: Groups/factions are GOAP agents that make strategic plans.

**Key insight**: The group itself is the planning entity, not individual leaders.

**Example**:
```
Group Goal: "Establish secure settlement"
Plan:
1. SendScoutingParty(northern_valley)
2. OrganizeResourceGathering(wood)
3. AssignGuardDuty(strongest_member, camp)
4. BuildShelter(northern_valley)
```

**How groups affect individuals**:
1. Group selects suitable individuals based on utility
2. Assigns them a `GroupTask` component
3. Individual utility functions prioritize "complete group task"
4. Person executes task using utility-based decisions for details

**Rationale**:
- Individuals stay reactive (no planning overhead for thousands)
- Groups think strategically (much fewer groups: 10-100)
- Natural narrative structure (group goals create story arcs)
- Emergent conflict (groups with competing plans create drama)
- Realistic scale (matches human organization)

### Narrative-Driven Group Goals

**Decision**: Groups can have story goals that create situations.

Examples:
- `"Avenge fallen leader"` → conflict
- `"Find the lost expedition"` → rescue missions
- `"Escape the valley before winter"` → time pressure
- `"Establish trade with neighbors"` → diplomacy

---

## Technology

### Language

**Decision**: Use **Rust**.

**Rationale**: Performance requirements for thousands of entities.

**Note**: Learning Rust as we build. Start with simple, clear code.

### Development Approach

**Decision**: **No game engine initially**.

- Work directly in VS Code
- CLI / text output only
- Focus on simulation rules, data model, event system
- Create logs that are interesting to read

**Later**: Consider Bevy, Godot, or other frontends. Simulation core remains unchanged.

---

## Implementation Roadmap

1. **Phase 1**: Standalone simulation project
   - Entities, components, systems
   - Cycle-based scheduler
   - Simple utility-based agents with needs and actions
   - Detailed text logs

2. **Phase 2**: Complexity
   - Social needs, relationships
   - Environmental awareness
   - Resource scarcity

3. **Phase 3**: Group-level GOAP
   - Group planning system
   - Group tasks assigned to individuals
   - Narrative goals

4. **Phase 4** (Optional): UI/Engine integration

---

## Design Principles

- Prefer **events over polling**
- Prefer **linear scaling** over quadratic
- Keep simulation logic **engine-agnostic**
- Make the simulation **readable as a story**
- Start simple; **complexity should emerge**, not be hard-coded
