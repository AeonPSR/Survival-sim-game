# Procedural Triage Scenario Generator - Project Summary

## Project Overview
The goal is to build a **Dwarf Fortress-style procedural generation system** for apocalyptic and survival scenarios. 

The player selects a small number of high-level parameters. The system then generates a specific scenario (including a pool of candidates, constraints, and events). The core gameplay loop is the player performing **triage** — deciding who gets the limited spots in the survivor group (bunker, train, generator-powered settlement, etc.). The system then resolves the consequences of those choices.

The system is designed to produce meaningfully different experiences, stories, and tactical/moral choices based on the combination of parameters. It started as a potential tech demo but is intended to become a real game with replayability, player agency, and clear feedback loops.

## Current Core Building Blocks (High-Level & Generic)

These are kept deliberately abstract so the generation logic stays simple and reusable.

- **The Setting**  
  Broad world/genre type (examples: Post-apocalyptic, Sci-fi, Dark Fantasy, Dystopian future, Fantasy, Grimdark, etc.)

- **The Menace**  
  The main threat or pressure (examples: Extreme cold, Resource scarcity, Zombies/infection, Tyranny/oppression, Toxic environment, Political control, etc.)

- **The Group**  
  Scale and nature of the population (examples: Single person, Small group, Society / large population, Rotating groups, etc.)

- **The Mac Guffin**  
  The central element or constraint around which the scenario revolves. Designed to be flexible and adapt based on other parameters (examples: Central infrastructure, Moving/confined transport system, Important artifact, Key individual, Mobile fortress, Symbol of resistance, etc.)

- **The Objective**  
  The high-level goal of the scenario. Current approved set (very generic):
  - **Survive** (reserved only for cases where pure survival is genuinely the main point, e.g. Frostpunk-style or Alien)
  - **Adapt** (adjusting to a new normal)
  - **Beat the menace**
  - **Find a location**
  - **Understanding something**

Additional Objective directions explored (still being evaluated for distinctness and usefulness):
- Rebuild society
- Protect key person / thing
- Deliver important item
- Change the system / Revolution
- Achieve domination
- Preserve culture

## Key Design Principles Established During Development

- **Maximal generality**: Every block and every value must stay at a very high level of abstraction. Specific story details, character names, or overly narrative phrasing are avoided.
- **"Survive" is special**: It should only be used when surviving is literally the central and primary goal. Many scenarios that look like survival actually have other main Objectives.
- **Mac Guffin is flexible**: It can adapt (e.g. "central infrastructure" becomes a generator in extreme cold scenarios or a master factory in virus scenarios).
- Many apparent "new" Objectives collapse into existing ones (e.g. Revolution often reduces to Beat the menace; Protect the future often reduces to Survive).
- The system should support clear cause-and-effect: different parameter combinations should produce different candidate pools, different constraints, different events, and different triage pressures.

## What We Did in This Conversation

1. **Initial Framework**  
   Started with the classic 4 blocks (Menace, Group, Mac Guffin, Objective) and quickly added **Setting** as a necessary fifth dimension.

2. **Refinement for Generality**  
   Multiple iterations to strip away specificity. Corrected overly narrative or game-specific phrasing in Objectives and other blocks. Standardized on very broad terms.

3. **Media Examples & Validation**  
   Created multiple tables mapping real films, TV shows, books, and games to the blocks. This helped test whether the categories were useful and distinct. Examples included Frostpunk, Snowpiercer, Mad Max: Fury Road, Dune, Children of Men, The Postman, Station Eleven, Alien, Subnautica, Warhammer 40k, and many others.

4. **Objective Discussions**  
   Extensive back-and-forth on what counts as a truly distinct Objective. Refined the approved list and explored (then often discarded) candidates like Revolution, Protect the future, Preserve culture, Achieve domination, etc., because many reduced to existing categories.

5. **Mac Guffin Flexibility**  
   Agreed that the Mac Guffin should be adaptable rather than rigidly defined, so the same parameter can manifest differently depending on the Menace and Setting.

6. **Game Design Thinking**  
   Discussed how to turn this from a pure generator into an actual game:
   - Parameter selection phase
   - Scenario + candidate generation
   - Player triage/selection phase (the main gameplay)
   - Consequence resolution phase
   - Replayability through different parameter combinations

7. **Ongoing Refinement**  
   Continuous feedback on keeping language and concepts at the right level of globality/abstraction for easy implementation in code.

## Current Status (as of this summary)

- The high-level framework is solid and consistent.
- The main remaining work is finalizing a small, truly distinct set of Objective categories and defining the actual generation rules (how each combination of parameters affects events, candidate traits, constraints, and post-triage outcomes).
- The system is ready to move from discussion into prototyping (e.g. a simple text-based or Python version that takes parameters and outputs a scenario + candidate list).

## Next Steps Ideas

- Finalize the Objective list and decide which additional ones (if any) are worth keeping as truly separate.
- Define simple generation rules for each block (tables, weights, or small simulation steps).
- Build a minimal prototype (parameter input → generated scenario + candidates).
- Test with real combinations to see if different parameters actually produce different triage experiences.

---

**File created**: `project_summary.md`  
This document can be updated as the project evolves.