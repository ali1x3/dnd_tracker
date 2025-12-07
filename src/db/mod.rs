use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::Error;
use surrealdb::Surreal;
use surrealdb::engine::local::RocksDb;

pub mod models;

use crate::db::models::{Die, Race, Skill, Weapon};
use crate::pages::weapons;
use crate::DbResponse;

use super::consts;
use super::Command;
use models::RacialTrait;
use surrealdb::sql::Id;
use surrealdb::sql::Thing;

struct Database {
    client: Surreal<surrealdb::engine::local::Db>,
    command_receiver: Receiver<Command>,
    response_sender: Sender<DbResponse>,
}

pub async fn init_db(
    command_receiver: Receiver<Command>,
    response_sender: Sender<DbResponse>,
) -> Result<(), Error> {
    let client = Surreal::new::<RocksDb>("db_data").await?;

    let db = Database {
        client,
        command_receiver,
        response_sender,
    };

    db.client.use_ns("app").use_db("main").await?;

    //let response: Option<RacialTrait> = db
    //    .client
    //    .create(consts::RACIAL_TRAITS_TABLE)
    //    .content(RacialTrait {
    //        id: Thing::from((consts::RACIAL_TRAITS_TABLE, Id::rand())),
    //        trait_name: "Test".into(),
    //        description: "Testing".into(),
    //    })
    //    .await?;
    //let mut response = db.client.select("SELECT * FROM racial_trait").await?;
    //let traits: Vec<RacialTraits> = response.take(0)?;

    
    // let response: Option<Race> = db
    //     .client
    //     .create("race")
    //     .content(Race {
    //         race_name: "elf".to_string(),
    //         move_speed: 30,
    //         ability_score_increase: models::AbilityScoreBonus { strength: 0, dexterity: 2, constitution: 0, intelligence: 0, wisdom: 0, charisma: 1 },
    //         racial_traits: vec![
    //             RacialTrait { level: 1, trait_name: "dark vision".into(), description: "see in dim light 60ft".into() },
    //             RacialTrait { level: 2, trait_name: "Keen Senses".into(), description: "Proficiency in Perception".into() }
    //         ],
    //         skill_proficiency_bonus: vec![
    //             Skill::Perception,
    //             Skill::Stealth,
    //         ],
    //         id: Thing::from(("race", Id::rand())),
    //         languages: vec![
    //             "Elvish".into(),
    //             "Common".into(),
    //             "Choose 1".into(),
    //         ],
    //     })
    //     .await?;
    let races: Vec<Race> = db.client.select(consts::RACE_TABLE).await?;
    println!("{:#?}", races);

    loop {
        if let Ok(command) = db.command_receiver.try_recv() {
            db.handle_command(command).await?
        }
    }
}

impl Database {
    async fn handle_command(&self, command: Command) -> Result<(), Error> {
        match command {
            Command::LoadRaces => {
                let races: Vec<Race> = self.client.select(consts::RACE_TABLE).await?;
                let _ = self.response_sender.send(DbResponse::Races(Some(races)));
                Ok(())
            },
            Command::CreateRace(race)=> {
                let response: Option<Race> = self
                    .client
                    .create(consts::RACE_TABLE)
                    .content(race)
                .await?;
                Ok(())
            },
            Command::DeleteRace(thing) => {
                let race: Option<Race> = self.client.delete((thing.tb, thing.id.to_string())).await?;
                Ok(())
            },
            Command::LoadWeapons => {
                let weapons: Vec<Weapon> = self.client.select(consts::WEAPON_TABLE).await?;
                let _ = self.response_sender.send(DbResponse::Weapons(Some(weapons)));
                Ok(())
            },
            Command::CreateWeapon(weapon) => {
                let response: Option<Weapon> = self
                    .client
                    .create(consts::WEAPON_TABLE)
                    .content(weapon)
                .await?;
                println!("{:#?}", response);
                Ok(())
            },
            Command::DeleteWeapon(thing) => {
                let weapon: Option<Weapon> = self.client.delete((thing.tb, thing.id.to_string())).await?;
                Ok(())
            },
        }
    }
}
