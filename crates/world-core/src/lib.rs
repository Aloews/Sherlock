//! Deterministic, inspectable foundations for the historical world simulation.
//!
//! This crate intentionally starts with a narrow material-world vertical slice.
//! It does not yet model finance, politics, dynasties, culture or football.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type EntityId = u64;
pub type Tick = u64;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    Food,
    Timber,
    Stone,
    Iron,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ResourceDeposit {
    pub kind: ResourceKind,
    pub stock: u64,
    pub annual_regeneration: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Settlement {
    pub id: EntityId,
    pub name: String,
    pub population: u64,
    pub food_stock: u64,
    pub food_production_per_tick: u64,
    pub food_consumption_per_person: u64,
    pub deposits: BTreeMap<ResourceKind, ResourceDeposit>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct World {
    pub id: EntityId,
    pub seed: u64,
    pub tick: Tick,
    pub settlements: BTreeMap<EntityId, Settlement>,
    pub event_log: Vec<WorldEvent>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorldEvent {
    WorldAdvanced {
        tick: Tick,
    },
    FoodProduced {
        settlement_id: EntityId,
        amount: u64,
        resulting_stock: u64,
    },
    FoodConsumed {
        settlement_id: EntityId,
        amount: u64,
        resulting_stock: u64,
    },
    PopulationChanged {
        settlement_id: EntityId,
        previous: u64,
        current: u64,
    },
    ResourceRegenerated {
        settlement_id: EntityId,
        kind: ResourceKind,
        amount: u64,
        resulting_stock: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorldError {
    EmptyWorldName,
    EmptySettlementName,
    ZeroPopulation,
    DuplicateSettlementId(EntityId),
    MissingSettlement(EntityId),
    FoodShortage {
        settlement_id: EntityId,
        required: u64,
        available: u64,
    },
    ArithmeticOverflow,
}

impl World {
    pub fn new(id: EntityId, seed: u64) -> Self {
        Self {
            id,
            seed,
            tick: 0,
            settlements: BTreeMap::new(),
            event_log: Vec::new(),
        }
    }

    pub fn add_settlement(&mut self, settlement: Settlement) -> Result<(), WorldError> {
        if settlement.name.trim().is_empty() {
            return Err(WorldError::EmptySettlementName);
        }
        if settlement.population == 0 {
            return Err(WorldError::ZeroPopulation);
        }
        if self.settlements.contains_key(&settlement.id) {
            return Err(WorldError::DuplicateSettlementId(settlement.id));
        }
        self.settlements.insert(settlement.id, settlement);
        Ok(())
    }

    /// Advance one coarse-grained world tick in stable entity/resource order.
    ///
    /// The operation is atomic: if any settlement cannot meet food consumption
    /// or arithmetic overflows, the world is left unchanged.
    pub fn advance_tick(&mut self) -> Result<(), WorldError> {
        let mut next = self.clone();
        let next_tick = next.tick.checked_add(1).ok_or(WorldError::ArithmeticOverflow)?;
        let settlement_ids: Vec<EntityId> = next.settlements.keys().copied().collect();

        for settlement_id in settlement_ids {
            let settlement = next
                .settlements
                .get_mut(&settlement_id)
                .ok_or(WorldError::MissingSettlement(settlement_id))?;

            let produced = settlement.food_production_per_tick;
            settlement.food_stock = settlement
                .food_stock
                .checked_add(produced)
                .ok_or(WorldError::ArithmeticOverflow)?;
            if produced > 0 {
                next.event_log.push(WorldEvent::FoodProduced {
                    settlement_id,
                    amount: produced,
                    resulting_stock: settlement.food_stock,
                });
            }

            let required = settlement
                .population
                .checked_mul(settlement.food_consumption_per_person)
                .ok_or(WorldError::ArithmeticOverflow)?;
            if settlement.food_stock < required {
                return Err(WorldError::FoodShortage {
                    settlement_id,
                    required,
                    available: settlement.food_stock,
                });
            }
            settlement.food_stock -= required;
            if required > 0 {
                next.event_log.push(WorldEvent::FoodConsumed {
                    settlement_id,
                    amount: required,
                    resulting_stock: settlement.food_stock,
                });
            }

            for deposit in settlement.deposits.values_mut() {
                let before = deposit.stock;
                deposit.stock = deposit
                    .stock
                    .checked_add(deposit.annual_regeneration)
                    .ok_or(WorldError::ArithmeticOverflow)?;
                let regenerated = deposit.stock - before;
                if regenerated > 0 {
                    next.event_log.push(WorldEvent::ResourceRegenerated {
                        settlement_id,
                        kind: deposit.kind,
                        amount: regenerated,
                        resulting_stock: deposit.stock,
                    });
                }
            }
        }

        next.tick = next_tick;
        next.event_log.push(WorldEvent::WorldAdvanced { tick: next_tick });
        *self = next;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), WorldError> {
        for settlement in self.settlements.values() {
            if settlement.name.trim().is_empty() {
                return Err(WorldError::EmptySettlementName);
            }
            if settlement.population == 0 {
                return Err(WorldError::ZeroPopulation);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settlement(id: EntityId, food_stock: u64, production: u64) -> Settlement {
        Settlement {
            id,
            name: format!("Settlement {id}"),
            population: 10,
            food_stock,
            food_production_per_tick: production,
            food_consumption_per_person: 1,
            deposits: BTreeMap::from([(
                ResourceKind::Timber,
                ResourceDeposit {
                    kind: ResourceKind::Timber,
                    stock: 20,
                    annual_regeneration: 2,
                },
            )]),
        }
    }

    #[test]
    fn advances_material_world_and_records_causes() {
        let mut world = World::new(1, 42);
        world.add_settlement(settlement(7, 5, 8)).unwrap();

        world.advance_tick().unwrap();

        let town = &world.settlements[&7];
        assert_eq!(world.tick, 1);
        assert_eq!(town.food_stock, 3);
        assert_eq!(town.deposits[&ResourceKind::Timber].stock, 22);
        assert!(world.event_log.iter().any(|event| matches!(
            event,
            WorldEvent::FoodProduced { settlement_id: 7, amount: 8, .. }
        )));
        assert!(world.event_log.iter().any(|event| matches!(
            event,
            WorldEvent::FoodConsumed { settlement_id: 7, amount: 10, .. }
        )));
    }

    #[test]
    fn failed_tick_is_atomic() {
        let mut world = World::new(1, 42);
        world.add_settlement(settlement(7, 0, 0)).unwrap();
        let before = world.clone();

        assert!(matches!(
            world.advance_tick(),
            Err(WorldError::FoodShortage { settlement_id: 7, .. })
        ));
        assert_eq!(world, before);
    }

    #[test]
    fn same_seed_and_state_produce_same_tick_result() {
        let mut left = World::new(1, 99);
        let mut right = World::new(1, 99);
        left.add_settlement(settlement(2, 10, 5)).unwrap();
        right.add_settlement(settlement(2, 10, 5)).unwrap();

        left.advance_tick().unwrap();
        right.advance_tick().unwrap();

        assert_eq!(left, right);
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let mut world = World::new(1, 1);
        world.add_settlement(settlement(3, 10, 5)).unwrap();
        assert_eq!(
            world.add_settlement(settlement(3, 10, 5)),
            Err(WorldError::DuplicateSettlementId(3))
        );
    }
}
