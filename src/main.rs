use person::{croak_vote, Person};
use mafia_world::{print_game_summary, CroakVoteResult, MafiaWorldTime};
use bevy_ecs::prelude::*;

pub mod person;
pub mod mafia_world;

// target role list:
// town:
/**
 * target role list:
 * town:
 *  lookout, doctor, detective
 * mafia:
 *  aronist, godfather
 * Neutral:
 *  Werewolf
 */
// 

fn main() {
    let mut world = World::new();

    world.spawn(Person::new("George".into()));
    world.spawn(Person::new("Lisa".into()));
    world.spawn(Person::new("Jabeson".into()));

    world.insert_resource(MafiaWorldTime::Day);
    world.insert_resource(CroakVoteResult::default());

    let mut schedule = Schedule::default();

    let mut query = world.query::<&mut Person>();
    for mut person in query.iter_mut(&mut world) {
        match person.name.as_str() {
            "George" => person.croak_vote_target = Some("Lisa".into()),
            "Lisa" => person.croak_vote_target = Some("Jabeson".into()),
            "Jabeson" => person.croak_vote_target = Some("Lisa".into()),
            _ => ()
        }
    }

    schedule.add_systems((croak_vote, print_game_summary).chain());

    schedule.run(&mut world);
    //so how can I set

    //now I add systems and run them on the schedule
}
