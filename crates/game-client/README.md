# Game Client

The `game-client` is currently a developer viewer for the assembled character state coming from the `game-server`.

It does three things:

1. connects to the UDP game server
2. authenticates, lists characters, and selects a character
3. renders the resolved snapshot in a Bevy window

The client does not rebuild character math locally. It only renders the snapshot and deltas sent by the server.

## Environment

Supported environment variables:

- `GAME_SERVER_ADDR`
  - default: `127.0.0.1:5000`
- `GAME_CLIENT_EMAIL`
  - optional debug shortcut that pre-fills the login form
- `GAME_CLIENT_PASSWORD`
  - optional debug shortcut that pre-fills the login form
- `GAME_CLIENT_TOKEN`
  - optional legacy fallback for authentication
- `GAME_CLIENT_CHARACTER_ID`
  - optional debug shortcut to auto-select a character after login
- `GAME_CLIENT_AUTO_ACTION`
  - optional action automatically sent after the full character snapshot loads
- `GAME_CLIENT_ENABLE_AUDIO`
  - optional
  - defaults to `false` under WSL
  - defaults to `true` elsewhere
- `WGPU_BACKEND`
  - optional
  - when not set, the client prefers `GL` under WSL to avoid unstable Vulkan `dzn` behavior
  - examples:
    - `WGPU_BACKEND=gl`
    - `WGPU_BACKEND=vulkan`

## Running

UI-first flow:

```sh
cargo run -p game-client
```

Use the window to:

1. type email and password
2. login
3. refresh or create characters
4. select a character

Debug shortcut example:

```sh
GAME_CLIENT_TOKEN="<jwt>" \
GAME_CLIENT_CHARACTER_ID="<character-uuid>" \
cargo run -p game-client
```

Preferred login flow:

```sh
GAME_CLIENT_EMAIL="<email>" \
GAME_CLIENT_PASSWORD="<password>" \
cargo run -p game-client
```

WSL-safe example:

```sh
WGPU_BACKEND=gl \
GAME_CLIENT_ENABLE_AUDIO=0 \
GAME_CLIENT_TOKEN="<jwt>" \
GAME_CLIENT_CHARACTER_ID="<character-uuid>" \
cargo run -p game-client
```

## Auto actions

Supported `GAME_CLIENT_AUTO_ACTION` formats:

- `use_item:<inventory_type>:<slot>`
- `use_item_on_equipment:<inventory_type>:<slot>:<equipment_slot>`
- `equip_item:<inventory_type>:<slot>`
- `unequip_item:<equipment_slot>`
- `refine_equipment:<equipment_slot>`
- `socket_gem:<equipment_slot>:<inventory_type>:<slot>:<socket_index>`

Examples:

```sh
GAME_CLIENT_AUTO_ACTION=use_item:special:0
GAME_CLIENT_AUTO_ACTION=use_item_on_equipment:special:0:weapon
GAME_CLIENT_AUTO_ACTION=equip_item:equipment:3
GAME_CLIENT_AUTO_ACTION=refine_equipment:weapon
GAME_CLIENT_AUTO_ACTION=socket_gem:weapon:material:0:0
```

## Character viewer

The window shows:

- character identity and progression
- final combat stats
- equipped items with rolled effects and gems
- inventory contents
- runtime state such as current HP/MP, buffs, and cooldowns

## Current item-modification rules

Current gameplay rules already enforced server-side:

- equipment reroll consumes one `equipment_reroll_scroll`
- reroll changes only `additional_effects`
- reroll does not touch `fixed_stats`
- reroll does not touch `fixed_special_effects`
- target equipment must already be identified
- gem socketing consumes the gem immediately
- socketing another gem in the same slot overwrites the previous gem and destroys it
- refinement currently has chance-based outcomes and a cap of `+7`
