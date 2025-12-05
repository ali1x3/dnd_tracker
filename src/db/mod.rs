use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::Error;
use surrealdb::Surreal;
use surrealdb::engine::local::RocksDb;

pub mod models;

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
    let traits: Vec<RacialTrait> = db.client.select(consts::RACIAL_TRAITS_TABLE).await?;
    println!("{:#?}", traits);

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

                let traits: Vec<RacialTrait> = self.client.select(consts::RACIAL_TRAITS_TABLE).await?;
                let _ = self.response_sender.send(DbResponse::Races(Some(traits)));
                Ok(())
            },
        }
    }
}
