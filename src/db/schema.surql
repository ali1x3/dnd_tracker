-- Main table
DEFINE TABLE race TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

-- Basic fields
DEFINE FIELD race_name ON TABLE race TYPE string PERMISSIONS FULL;
DEFINE FIELD move_speed ON TABLE race TYPE int PERMISSIONS FULL;

-- Nested object for ability_score_increase
DEFINE FIELD ability_score_increase ON TABLE race TYPE object PERMISSIONS FULL;
DEFINE FIELD ability_score_increase.strength     ON TABLE race TYPE int PERMISSIONS FULL;
DEFINE FIELD ability_score_increase.dexterity    ON TABLE race TYPE int PERMISSIONS FULL;
DEFINE FIELD ability_score_increase.constitution ON TABLE race TYPE int PERMISSIONS FULL;
DEFINE FIELD ability_score_increase.intelligence ON TABLE race TYPE int PERMISSIONS FULL;
DEFINE FIELD ability_score_increase.wisdom       ON TABLE race TYPE int PERMISSIONS FULL;
DEFINE FIELD ability_score_increase.charisma     ON TABLE race TYPE int PERMISSIONS FULL;

-- Racial traits as array of objects (with level)
DEFINE FIELD racial_traits ON TABLE race TYPE array<object> PERMISSIONS FULL;
DEFINE FIELD racial_traits[*].level       ON TABLE race TYPE int PERMISSIONS FULL;
DEFINE FIELD racial_traits[*].trait_name ON TABLE race TYPE string PERMISSIONS FULL;
DEFINE FIELD racial_traits[*].description ON TABLE race TYPE string PERMISSIONS FULL;

-- Skill proficiency bonus as array of strings (enum serialized as string)
DEFINE FIELD skill_proficiency_bonus ON TABLE race TYPE array<string> PERMISSIONS FULL;

-- Optional: languages
DEFINE FIELD languages ON TABLE race TYPE array<string> PERMISSIONS FULL;
