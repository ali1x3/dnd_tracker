use anyhow::Error;
use serde::{Deserialize, Serialize};
use surrealdb::sql;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Race {
    pub id: sql::Thing,
    pub race_name: String,
    pub move_speed: i16,
    pub ability_score_increase: AbilityScoreBonus,
    pub racial_traits: Vec<RacialTrait>,
    pub skill_proficiency_bonus: Vec<Skill>,
    pub languages: Vec<String>,
}

#[derive(Default, Clone)]
pub struct RaceBuilder {
    id: Option<sql::Thing>,
    race_name: Option<String>,
    move_speed: Option<i16>,
    ability_score_increase: Option<AbilityScoreBonus>,
    racial_traits: Vec<RacialTrait>,
    skill_proficiency_bonus: Vec<Skill>,
    languages: Vec<String>,
}

#[derive(Debug)]
pub enum BuildError {
    MissingField(&'static str),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::MissingField(field) => write!(f, "Missing required field: {}", field),
        }
    }
}

impl RaceBuilder {
    /// Starts a new builder instance.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&mut self, table: &str) -> &mut Self {
        self.id = Some(sql::Thing::from((table, sql::Id::rand())));
        self
    }

    pub fn race_name(&mut self, name: String) -> &mut Self {
        self.race_name = Some(name);
        self
    }

    pub fn move_speed(&mut self, speed: i16) -> &mut  Self {
        self.move_speed = Some(speed);
        self
    }

    pub fn ability_score_increase(&mut self, bonus: AbilityScoreBonus) -> &mut Self {
        self.ability_score_increase = Some(bonus);
        self
    }

    pub fn add_racial_trait(&mut self, trait_item: RacialTrait) -> &mut Self {
        self.racial_traits.push(trait_item);
        self
    }

    pub fn add_skill_proficiency(&mut self, skill: Skill) -> &mut Self {
        self.skill_proficiency_bonus.push(skill);
        self
    }

    pub fn add_language(&mut self, language: String) -> &mut Self {
        self.languages.push(language);
        self
    }

    pub fn build(mut self) -> Result<Race, BuildError> {
        if let Some(name) = &self.race_name {
            if name.is_empty() {
                self.race_name = None;
            }
        }

        let id = self.id.ok_or(BuildError::MissingField("id"))?;
        let race_name = self.race_name.ok_or(BuildError::MissingField("race_name"))?;
        let move_speed = self.move_speed.ok_or(BuildError::MissingField("move_speed"))?;
        let ability_score_increase = self.ability_score_increase.ok_or(BuildError::MissingField("ability_score_increase"))?;

        let racial_traits = self.racial_traits;
        let skill_proficiency_bonus = self.skill_proficiency_bonus;
        let languages = self.languages;

        Ok(Race {
            id,
            race_name,
            move_speed,
            ability_score_increase,
            racial_traits,
            skill_proficiency_bonus,
            languages,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RacialTrait {
    pub level: i8,
    pub trait_name: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbilityScoreBonus {
    pub strength: i8,
    pub dexterity: i8,
    pub constitution: i8,
    pub intelligence: i8,
    pub wisdom: i8,
    pub charisma: i8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl PartialEq for Skill{
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Skill::Acrobatics       => write!(f, "Acrobatics"),
            Skill::AnimalHandling   => write!(f, "Animal Handling"),
            Skill::Arcana           => write!(f, "Arcana"),
            Skill::Athletics        => write!(f, "Athletics"),
            Skill::Deception        => write!(f, "Deception"),
            Skill::History          => write!(f, "History"),
            Skill::Insight          => write!(f, "Insight"),
            Skill::Intimidation     => write!(f, "Intimidation"),
            Skill::Investigation     => write!(f, "Investigation"), // also spelling?
            Skill::Medicine         => write!(f, "Medicine"),
            Skill::Nature           => write!(f, "Nature"),
            Skill::Perception       => write!(f, "Perception"),
            Skill::Performance      => write!(f, "Performance"),
            Skill::Persuasion       => write!(f, "Persuasion"),
            Skill::Religion         => write!(f, "Religion"),
            Skill::SleightOfHand    => write!(f, "Sleight of Hand"),
            Skill::Stealth          => write!(f, "Stealth"),
            Skill::Survival         => write!(f, "Survival"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}
