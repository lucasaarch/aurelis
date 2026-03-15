# Character Loading Plan

## Objetivo atual

Fechar o carregamento do personagem no `game-server` com status funcionais e persistência coerente, mantendo o `api-server` como fonte de estado persistido e o `game-server` como fonte de cálculo/runtime.

## O que já está pronto

- Snapshot interno do personagem entre `api-server` e `game-server`
- Afinidade de combate por linha evolutiva
- Modelagem de skill com:
  - `Active`
  - `Advantage`
  - `SpecialActive`
  - `Passive`
- Unlock de skill por:
  - nível
  - tier de livro do personagem
- Carregamento de:
  - inventários
  - equipamentos
  - item instances
  - gems socketadas
- Builder de runtime no `game-server`
- Cálculo atual de:
  - base do personagem
  - bônus da classe
  - `RewardStats` para bônus de recompensa/progressão
  - afinidade atual da linha evolutiva
  - skills disponíveis derivadas a partir de nível + linha atual + unlocks persistidos
  - stats fixos do equipamento
  - efeitos fixos imutáveis do item
  - efeitos persistentes da instância (`attributes_json`)
  - modificadores temporários
- Testes unitários do runtime cobrindo base, classe, equipamento, modifiers e item identificado

## Decisões de modelagem

- `ItemData` / catálogo:
  - stats fixos do item
  - efeitos fixos imutáveis do item
  - regras de identificação
  - sockets base e limite de sockets extras

- `ItemInstance` / instância viva:
  - refinamento
  - `bonus_gem_slots`
  - `attributes_json`
  - gems encaixadas
  - ownership/trade/storage

- `attributes_json` guarda apenas estado variável da instância:
  - `identified`
  - `roll_bias`
  - `reroll_count`
  - `additional_effects`

- `CombatStats` e `RewardStats` são separados:
  - `CombatStats` cobre combate/runtime direto
  - `RewardStats` cobre bônus como XP, drop e créditos

- Unlock de skill trancada é persistido no personagem, não por `skill_slug`:
  - `beginner_skill_unlocked`
  - `intermediate_skill_unlocked`

- Skills normais não são persistidas como "aprendidas":
  - o runtime deriva automaticamente a disponibilidade por nível e linha atual

## Etapa atual

Consolidar persistência e runtime para:
- identificação de item
- afinidade/skills derivadas
- unlock por livro em nível de personagem

Mudança estrutural aplicada:
- `item_instances.gem_slots` foi renomeado conceitualmente para `bonus_gem_slots`

Motivo:
- sockets base pertencem ao catálogo do item
- sockets extras pertencem à instância viva

Infra já pronta nesta etapa:
- catálogo com `fixed_special_effects` e `identification`
- runtime com geração de `additional_effects`
- runtime com camadas separadas para `CombatStats` e `RewardStats`
- runtime com lista de skills disponíveis derivada do catálogo
- RPC interno para persistir estado da instância do item

## Próximo passo imediato

Próximo passo imediato:

1. Ler `EquipmentIdentificationRules` do catálogo
2. Gerar `additional_effects` a partir do pool do item
3. Persistir o resultado no `api-server`
4. Plugar isso em um fluxo real de ação/comando
5. Implementar consumo do livro para marcar:
   - `beginner_skill_unlocked`
   - `intermediate_skill_unlocked`
6. Recarregar/refletir isso no runtime do personagem

## Passos seguintes

1. Aplicar gems como `RuntimeModifier`
2. Validar `base_slots + bonus_gem_slots`
3. Implementar passivas reais de skill usando a mesma infraestrutura de modifiers
4. Começar a alimentar `RewardStats` a partir de catálogo/passivas/equipamentos
5. Integrar persistência de identificação/reroll/socket no `api-server`

## Regra de ouro

O `api-server` persiste.

O `game-server` interpreta, valida e calcula.
