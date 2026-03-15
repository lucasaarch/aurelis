# Character Status Architecture

## Scope
This document defines the official architecture and calculation rules for character status assembly before combat and dungeon runtime.

It covers:
- character base stats
- class/path bonuses
- equipped item calculation
- passive and temporary modifiers
- reward stats separation

It does not cover:
- active damage execution
- targeting/combat resolution
- dungeon systems
- quest reward flow

## Responsibility Split

### `api-server`
The `api-server` persists player state and returns raw snapshot data.

It is responsible for:
- character identity and progression
- inventories
- equipped item instances
- item instance state (`attributes_json`, refinement, gem slots)
- learned character-tier unlocks

It is not responsible for:
- calculating final combat stats
- interpreting gameplay formulas
- applying runtime modifiers

### `game-server`
The `game-server` is authoritative for character runtime assembly and stat calculation.

It is responsible for:
- reconstructing `RuntimeCharacter`
- calculating final character stats
- applying passive and temporary modifiers
- tracking current HP/MP during the session

## Character Stat Layers
Final character status is assembled from these layers:

1. `base`
2. `class`
3. `equipment`
4. `persistent modifiers`
5. `timed modifiers`

The final result is:

`final = base + class + equipment + persistent + timed`

## Base and Class Stats

### Base
Base stats come from `CharacterData.base_stats`.

### Class
Class bonuses come from the current evolution line/class.

The base class has:
- neutral affinity

Evolution lines define:
- `Physical`
- `Magical`

Affinity influences skill design and progression logic, but does not automatically zero out unrelated stats.

## Equipment Architecture
Equipment is split into:

1. `ItemData`
2. `ItemInstance`
3. equipped runtime calculation

### `ItemData`
Defines static catalog behavior:
- `fixed_stats`
- `fixed_special_effects`
- `gem_slots`
- identification rules
- slot/type metadata

### `ItemInstance`
Defines runtime-persistent state of a specific item:
- `refinement`
- `bonus_gem_slots`
- `attributes_json`
- socketed gem instances
- trade/shared-storage flags

### Equipped Item Calculation
Each equipped item is calculated independently before being added to the character.

Current equipped item pipeline:

1. item `fixed_stats`
2. instance `additional_effects` from `attributes_json`
3. socketed gem effects
4. refinement bonus
5. final contribution of the equipped item

The character never calculates directly from raw item definitions alone. It consumes the resolved result of each equipped item instance.

## Fixed Stats vs Fixed Effects vs Additional Effects

### `fixed_stats`
These define the structural identity of the item.

Examples:
- weapon attack
- chest HP/defense
- gloves attack speed
- shoes movement speed

### `fixed_special_effects`
These are immutable special effects that always exist on every instance of the catalog item.

Examples:
- fixed crit chance
- fixed crit damage

### `additional_effects`
These are instance-specific rolled or modified effects stored in `attributes_json`.

Examples:
- `physical_attack_level +375`
- `crit_damage +5`
- `physical_atk +10%`

These are mutable at the instance level and belong to the live item, not to the catalog.

## Gems

Gems support two layers:

1. fixed gem modifiers
2. rolled gem instance modifiers

### Gem Catalog
Gem catalog data may define:
- fixed effects
- a weighted effect pool
- min/max roll ranges
- roll count

### Gem Instance
Rolled gem results are persisted in the gem instance `attributes_json`.

### Gem Restrictions
Gems must not grant:
- `physical_atk`
- `magical_atk`
- `physical_def`
- `magical_def`

Those stats remain exclusive to equipment itself.

## Refinement

### General Rule
Refinement applies only to equipped items.

Current cap:
- `+7`

### Official Constraint
Refinement scales only the item's `fixed_stats`.

Refinement does **not** scale:
- gem effects
- instance random effects from `attributes_json`
- fixed special effects

### Scaling Authority
Refinement is calculated per equipment type through `EquipmentSlot`.

Each slot defines its own refinement profile.

This avoids a global refinement formula that would over-scale sensitive stats like:
- movement speed
- attack speed
- critical stats

## Modifier Semantics

### Modifier Families
There are two runtime modifier families:
- `persistent modifiers`
- `timed modifiers`

Examples:
- passive skill = persistent
- temporary buff = timed

### Percent Modifiers
Percent modifiers are expressed in basis points.

Examples:
- `1000 = 10%`
- `1500 = 15%`

### Official Percentage Rule
Percent modifiers always apply over:

`base + class + equipment`

They do **not** apply over previously applied modifiers.

That means:
- multiple percent modifiers use the same base
- percent modifiers do not stack multiplicatively
- percent modifiers do not cascade into each other

Example:
- physical attack base = `1000`
- modifier A = `+15%`
- modifier B = `+30%`

Result:
- `1000 + 150 + 300 = 1450`

Not:
- `1000 -> 1150 -> 1495`

This rule is intentional and prevents abusive runtime scaling.

## Critical Damage Semantics
`crit_damage` is represented as a total percentage multiplier value.

Example:
- `crit_damage = 150`

This means:
- critical hits deal `1.5x` normal damage

## Reward Stats
Reward-related bonuses are separated from combat stats.

Current reward stat family:
- `experience_gain`
- `drop_rate`
- `credit_gain`

These use the same modifier infrastructure but are not part of combat stat assembly.

Reward stats are not part of passive skill trees in the current design.

## Runtime Resources
Current HP and MP are session-only runtime data.

They live only in the `game-server` session/runtime and are not persisted as authoritative long-term character state.

This is intentional because runtime maxima may change dynamically due to:
- equipment changes
- buffs
- passives

### Important Distinction
- `final_stats.core.hp/mp` = current maximums
- `resources.current_hp/current_mp` = current volatile values inside the session

Increasing maximum MP or HP does not automatically refill the current resource unless explicitly designed to do so.

## Skill Interaction with Status

### Passive Skills
Passive skills are converted into persistent modifiers automatically when available to the character.

### Advantage Skills
Advantage skills apply timed modifiers, consume MP, and enter cooldown.

### Active and Special Active Skills
Their damage/combat execution is outside the scope of this status architecture.

## Current Architectural Decisions
These are considered official for the current phase:

- `CombatStats` and `RewardStats` are separate
- final character stats are calculated only in the `game-server`
- `ItemData` and `ItemInstance` are separate responsibilities
- `fixed_stats` are separate from `fixed_special_effects`
- random/instance effects live in `attributes_json`
- gem rolls live in gem instance `attributes_json`
- refinement scales only `fixed_stats`
- percent modifiers use `base + class + equipment` as their base
- percent modifiers never cascade on top of other modifiers
- current HP/MP are runtime-only session values

## Status Phase Completion
For the current project phase, the character status architecture is considered complete enough to support:
- character assembly before dungeon entry
- equipment-based progression
- passive skill influence
- temporary stat buffs
- future catalog expansion without architectural rework
