use std::collections::HashMap;

use bevy_ecs::prelude::*;

use crate::{mafia_world::CroakVoteResult, person};

#[derive(Component, Default, Debug)]
pub struct Person {
    pub name: String,
    pub resources: Vec<MafiaResources>,
    pub statuses: Vec<Statuses>,
    pub state: State,
    pub croak_vote_target: Option<String>
}


impl Person {
    pub fn new(name: String) -> Person {
        return Person{ name, ..Default::default()}
    }
}

#[derive(Debug)]
pub enum MafiaResources {
    Coin(i32),
    GiftCards(i32)
}

#[derive(Debug)]
pub enum Statuses {
    Doused
}

#[derive(Default, Debug)]
pub enum State {
    #[default]
    Alive,
    Dead
}

pub fn croak_vote(
    mut query : Query<&mut Person>,
    mut croak_vote_result : ResMut<CroakVoteResult>
) 
{

    let mut map = HashMap::<String, Vec<String>>::new();
    for person in &mut query {
        if person.croak_vote_target.is_some() {
            let target = person.croak_vote_target.clone().expect("");
            let accusers = match map.contains_key(&target) {
                true => map.get_mut(&target).expect(""),
                false => {
                    map.insert(target.clone(), Vec::new());
                    map.get_mut(&target).expect("")
                },
            };
            accusers.push(person.name.clone());
        }
    }
    println!("{:?}", &map);
    let to_be_croaked = map.clone().into_iter().map(|item| (item.0, item.1.len())).max().expect("its there man").0;
    let mut person = query.iter_mut().find(|person| person.name == to_be_croaked).expect("it exists");
    croak_vote_result.result = map;
    person.state = State::Dead;
}