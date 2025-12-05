DEFINE TABLE racial_trait TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

-- ------------------------------
-- FIELDS
-- ------------------------------ 

DEFINE FIELD description ON racial_trait TYPE string PERMISSIONS FULL;
DEFINE FIELD trait_name ON racial_trait TYPE string PERMISSIONS FULL;



CREATE racial_trait SET
    trait_name = "Darkvision",
    description = "Can see in darkness up to 60 feet";

CREATE racial_trait SET
    trait_name = "Keen Senses",
    description = "Proficiency in Perception";

CREATE racial_trait SET
    trait_name = "Fey Ancestry",
    description = "Advantage on charm saves";

CREATE racial_trait SET
    trait_name = "Dwarven Resilience",
    description = "Advantage vs poison and resistance";
