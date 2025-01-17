use std::collections::HashMap;

use bevy_ecs::prelude::*;

use crate::person::Person;

#[derive(Resource, Default)]
pub enum MafiaWorldTime{
    #[default]
    Day,
    Night
}

#[derive(Resource, Default)]
pub struct CroakVoteResult{
    pub result: HashMap<String, Vec<String>>
}


pub fn print_game_summary(mut query : Query<&mut Person>) {
    for person in &mut query {
        println!("{:?}", person )
    }
}