mod types;
mod components;
mod scheduler;

use std::collections::HashMap;
use types::EntityId;
use components::Hunger;
use scheduler::{Scheduler, EventType};

fn main() {
    let mut hungers: HashMap<EntityId, Hunger> = HashMap::new();
    let mut scheduler = Scheduler::new();
    
    // Create entities with hunger
    hungers.insert(0, Hunger { current: 100.0, decay_rate: 5.0 });
    hungers.insert(1, Hunger { current: 80.0, decay_rate: 3.0 });
    
    // Schedule initial "hunger critical" events
    // Entity 0: 100 → 5 = 95 points to drop, at 5/cycle = 19 cycles
    scheduler.schedule(19, EventType::HungerCritical { entity: 0 });
    // Entity 1: 80 → 5 = 75 points to drop, at 3/cycle = 25 cycles
    scheduler.schedule(25, EventType::HungerCritical { entity: 1 });
    
    // Simulation loop
    let max_cycles = 30;
    
    for current_cycle in 0..max_cycles {
        // Process all events due this cycle
        while let Some(event) = scheduler.pop_due(current_cycle) {
            match event.event_type {
                EventType::HungerCritical { entity } => {
                    println!("Cycle {}: Entity {} is critically hungry!", current_cycle, entity);
                    // Could schedule disease rolls here
                    scheduler.schedule(current_cycle + 1, EventType::DiseaseRoll { entity });
                }
                EventType::DiseaseRoll { entity } => {
                    println!("Cycle {}: Entity {} disease check (would roll dice)", current_cycle, entity);
                    // Reschedule if still critical (for now, just run 3 times)
                    if current_cycle < 28 {
                        scheduler.schedule(current_cycle + 1, EventType::DiseaseRoll { entity });
                    }
                }
            }
        }
    }
    
    println!("Simulation complete!");
}