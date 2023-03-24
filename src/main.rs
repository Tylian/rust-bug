use bevy_ecs::prelude::*;

#[derive(Component)]
struct Words<'a>(&'a str);

fn main() {
    let schedule = Schedule::new();
    schedule.add_system(print_system);
}

fn print_system(query: Query<&Words>) {
    for words in &query {
        println!("{}", words);
    }
}