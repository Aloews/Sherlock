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
    /// Regeneration per simulation tick; calendar conversion is a world-level rule.
    pub regeneration_per_tick: u64,
    /// Optional hard ecological stock limit.
    #[serde(default)]
    pub capacity: Option<u64>,
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
    /// Tradable stocks, separate from the subsistence food pool.
    #[serde(default)]
    pub inventory: BTreeMap<ResourceKind, u64>,
    /// Integer minor currency units; floating-point money is forbidden.
    #[serde(default)]
    pub treasury_balance: i64,
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
    TradeExecuted {
        seller_id: EntityId,
        buyer_id: EntityId,
        kind: ResourceKind,
        quantity: u64,
        unit_price: u64,
        total_price: i64,
        seller_balance_after: i64,
        buyer_balance_after: i64,
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
    InvalidResourceDeposit { settlement_id: EntityId, kind: ResourceKind },
    SettlementIdMismatch { map_id: EntityId, settlement_id: EntityId },
    SelfTrade,
    ZeroTradeQuantity,
    ZeroUnitPrice,
    InsufficientInventory { settlement_id: EntityId, kind: ResourceKind, required: u64, available: u64 },
    InsufficientTreasury { settlement_id: EntityId, required: i64, available: i64 },
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
        for (key, deposit) in &settlement.deposits {
            if *key != deposit.kind || deposit.capacity.is_some_and(|cap| deposit.stock > cap) {
                return Err(WorldError::InvalidResourceDeposit {
                    settlement_id: settlement.id,
                    kind: deposit.kind,
                });
            }
        }
        self.settlements.insert(settlement.id, settlement);
        Ok(())
    }

    /// Execute a fully atomic spot trade between two settlements.
    /// Money uses signed integer minor units; both balances and inventories
    /// change together or not at all.
    pub fn execute_trade(
        &mut self,
        seller_id: EntityId,
        buyer_id: EntityId,
        kind: ResourceKind,
        quantity: u64,
        unit_price: u64,
    ) -> Result<(), WorldError> {
        if seller_id == buyer_id {
            return Err(WorldError::SelfTrade);
        }
        if quantity == 0 {
            return Err(WorldError::ZeroTradeQuantity);
        }
        if unit_price == 0 {
            return Err(WorldError::ZeroUnitPrice);
        }
        let total_unsigned = quantity.checked_mul(unit_price)
            .ok_or(WorldError::ArithmeticOverflow)?;
        let total = i64::try_from(total_unsigned)
            .map_err(|_| WorldError::ArithmeticOverflow)?;

        let mut next = self.clone();
        let buyer_balance = next.settlements.get(&buyer_id)
            .ok_or(WorldError::MissingSettlement(buyer_id))?.treasury_balance;
        if buyer_balance < total {
            return Err(WorldError::InsufficientTreasury {
                settlement_id: buyer_id, required: total, available: buyer_balance,
            });
        }
        let seller_stock = next.settlements.get(&seller_id)
            .ok_or(WorldError::MissingSettlement(seller_id))?
            .inventory.get(&kind).copied().unwrap_or(0);
        if seller_stock < quantity {
            return Err(WorldError::InsufficientInventory {
                settlement_id: seller_id, kind, required: quantity, available: seller_stock,
            });
        }
        let seller_balance_after = next.settlements[&seller_id].treasury_balance
            .checked_add(total).ok_or(WorldError::ArithmeticOverflow)?;
        let buyer_balance_after = buyer_balance.checked_sub(total)
            .ok_or(WorldError::ArithmeticOverflow)?;
        let buyer_stock_after = next.settlements[&buyer_id].inventory.get(&kind)
            .copied().unwrap_or(0).checked_add(quantity)
            .ok_or(WorldError::ArithmeticOverflow)?;

        {
            let seller = next.settlements.get_mut(&seller_id).expect("validated seller");
            let stock = seller.inventory.get_mut(&kind).expect("validated seller stock");
            *stock -= quantity;
            seller.treasury_balance = seller_balance_after;
        }
        {
            let buyer = next.settlements.get_mut(&buyer_id).expect("validated buyer");
            buyer.inventory.insert(kind, buyer_stock_after);
            buyer.treasury_balance = buyer_balance_after;
        }
        next.event_log.push(WorldEvent::TradeExecuted {
            seller_id, buyer_id, kind, quantity, unit_price, total_price: total,
            seller_balance_after, buyer_balance_after,
        });
        *self = next;
        Ok(())
    }

    /// Advance one coarse-grained world tick in stable entity/resource order.
    ///
    /// The operation is atomic: if any settlement cannot meet food consumption
    /// or arithmetic overflows, the world is left unchanged.
    pub fn advance_tick(&mut self) -> Result<(), WorldError> {
        let mut next = self.clone();
        let next_tick = next
            .tick
            .checked_add(1)
            .ok_or(WorldError::ArithmeticOverflow)?;
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
                let regenerated_stock = deposit.stock
                    .checked_add(deposit.regeneration_per_tick)
                    .ok_or(WorldError::ArithmeticOverflow)?;
                deposit.stock = deposit.capacity
                    .map_or(regenerated_stock, |capacity| regenerated_stock.min(capacity));
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
        next.event_log
            .push(WorldEvent::WorldAdvanced { tick: next_tick });
        *self = next;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), WorldError> {
        for (map_id, settlement) in &self.settlements {
            if *map_id != settlement.id {
                return Err(WorldError::SettlementIdMismatch {
                    map_id: *map_id,
                    settlement_id: settlement.id,
                });
            }
            if settlement.name.trim().is_empty() {
                return Err(WorldError::EmptySettlementName);
            }
            if settlement.population == 0 {
                return Err(WorldError::ZeroPopulation);
            }
            for (key, deposit) in &settlement.deposits {
                if *key != deposit.kind || deposit.capacity.is_some_and(|cap| deposit.stock > cap) {
                    return Err(WorldError::InvalidResourceDeposit {
                        settlement_id: settlement.id,
                        kind: deposit.kind,
                    });
                }
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
            inventory: BTreeMap::new(),
            treasury_balance: 0,
            deposits: BTreeMap::from([(
                ResourceKind::Timber,
                ResourceDeposit {
                    kind: ResourceKind::Timber,
                    stock: 20,
                    regeneration_per_tick: 2,
                    capacity: Some(100),
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
            WorldEvent::FoodProduced {
                settlement_id: 7,
                amount: 8,
                ..
            }
        )));
        assert!(world.event_log.iter().any(|event| matches!(
            event,
            WorldEvent::FoodConsumed {
                settlement_id: 7,
                amount: 10,
                ..
            }
        )));
    }

    #[test]
    fn failed_tick_is_atomic() {
        let mut world = World::new(1, 42);
        world.add_settlement(settlement(7, 0, 0)).unwrap();
        let before = world.clone();

        assert!(matches!(
            world.advance_tick(),
            Err(WorldError::FoodShortage {
                settlement_id: 7,
                ..
            })
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
    fn trade_transfers_goods_and_balances_atomically() {
        let mut world = World::new(1, 7);
        let mut seller = settlement(1, 10, 5);
        seller.inventory.insert(ResourceKind::Timber, 12);
        let mut buyer = settlement(2, 10, 5);
        buyer.treasury_balance = 500;
        world.add_settlement(seller).unwrap();
        world.add_settlement(buyer).unwrap();

        world.execute_trade(1, 2, ResourceKind::Timber, 4, 25).unwrap();

        assert_eq!(world.settlements[&1].inventory[&ResourceKind::Timber], 8);
        assert_eq!(world.settlements[&2].inventory[&ResourceKind::Timber], 4);
        assert_eq!(world.settlements[&1].treasury_balance, 100);
        assert_eq!(world.settlements[&2].treasury_balance, 400);
        assert!(matches!(world.event_log.last(), Some(WorldEvent::TradeExecuted {
            seller_id: 1, buyer_id: 2, quantity: 4, total_price: 100, ..
        })));
    }

    #[test]
    fn rejected_trade_leaves_world_unchanged() {
        let mut world = World::new(1, 7);
        let mut seller = settlement(1, 10, 5);
        seller.inventory.insert(ResourceKind::Timber, 2);
        let mut buyer = settlement(2, 10, 5);
        buyer.treasury_balance = 10;
        world.add_settlement(seller).unwrap();
        world.add_settlement(buyer).unwrap();
        let before = world.clone();

        assert!(matches!(
            world.execute_trade(1, 2, ResourceKind::Timber, 3, 25),
            Err(WorldError::InsufficientTreasury { settlement_id: 2, .. })
        ));
        assert_eq!(world, before);
    }

    #[test]
    fn trade_rejects_overflow_and_self_trade() {
        let mut world = World::new(1, 7);
        let mut seller = settlement(1, 10, 5);
        seller.inventory.insert(ResourceKind::Iron, u64::MAX);
        let mut buyer = settlement(2, 10, 5);
        buyer.treasury_balance = i64::MAX;
        world.add_settlement(seller).unwrap();
        world.add_settlement(buyer).unwrap();
        let before = world.clone();

        assert_eq!(
            world.execute_trade(1, 2, ResourceKind::Iron, u64::MAX, 2),
            Err(WorldError::ArithmeticOverflow)
        );
        assert_eq!(world, before);
        assert_eq!(world.execute_trade(1, 1, ResourceKind::Iron, 1, 1), Err(WorldError::SelfTrade));
        assert_eq!(world, before);
    }

    #[test]
    fn world_snapshot_round_trip_preserves_event_history() {
        let mut world = World::new(1, 42);
        let mut seller = settlement(1, 10, 5);
        seller.inventory.insert(ResourceKind::Stone, 9);
        let mut buyer = settlement(2, 10, 5);
        buyer.treasury_balance = 100;
        world.add_settlement(seller).unwrap();
        world.add_settlement(buyer).unwrap();
        world.execute_trade(1, 2, ResourceKind::Stone, 2, 10).unwrap();
        world.advance_tick().unwrap();

        let encoded = serde_json::to_vec(&world).unwrap();
        let decoded: World = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded, world);
    }

    #[test]
    fn resource_regeneration_respects_capacity() {
        let mut world = World::new(1, 2);
        let mut town = settlement(7, 10, 5);
        town.deposits.insert(ResourceKind::Iron, ResourceDeposit {
            kind: ResourceKind::Iron,
            stock: 99,
            regeneration_per_tick: 5,
            capacity: Some(100),
        });
        world.add_settlement(town).unwrap();
        world.advance_tick().unwrap();
        assert_eq!(world.settlements[&7].deposits[&ResourceKind::Iron].stock, 100);
    }

    #[test]
    fn invalid_deposit_capacity_is_rejected_before_insertion() {
        let mut world = World::new(1, 2);
        let mut town = settlement(7, 10, 5);
        town.deposits.insert(ResourceKind::Iron, ResourceDeposit {
            kind: ResourceKind::Iron,
            stock: 101,
            regeneration_per_tick: 1,
            capacity: Some(100),
        });
        assert!(matches!(
            world.add_settlement(town),
            Err(WorldError::InvalidResourceDeposit { settlement_id: 7, kind: ResourceKind::Iron })
        ));
        assert!(world.settlements.is_empty());
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
