mod engine;
mod food;

#[derive(Clone, Debug, PartialEq)]
enum Event {
    Dummy,   // placeholder until food.rs exists
}

fn main() {
    let mut the_world = engine::Scheduler::new();

    the_world.schedule_at(30, Event::Dummy);
    the_world.schedule_at(10, Event::Dummy);
    the_world.schedule_at(20, Event::Dummy);

    while let Some((time, event)) = the_world.pop_next() {
        println!("{} -> {:?}", time, event);
    }
}