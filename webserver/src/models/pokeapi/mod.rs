use galvyn::rorm::Model;
use galvyn::rorm::prelude::ForeignModel;

#[derive(Model)]
#[rorm(rename = "pokemon_v2_ability")]
pub struct Ability {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 200)]
    pub name: String,

    pub is_main_series: bool,

    pub generation_id: Option<i32>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_abilitychange")]
pub struct AbilityChange {
    #[rorm(id)]
    pub id: i32,

    pub ability_id: Option<ForeignModel<Ability>>,

    pub version_group_id: Option<ForeignModel<versiongroup>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_abilitychangeeffecttext")]
pub struct AbilityChangeEffectText {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 6000)]
    pub effect: String,

    pub ability_change_id: Option<ForeignModel<AbilityChange>>,

    pub language_id: Option<ForeignModel<language>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_abilityeffecttext")]
pub struct AbilityEffectText {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 6000)]
    pub effect: String,

    #[rorm(max_length = 300)]
    pub short_effect: String,

    pub ability_id: Option<ForeignModel<Ability>>,

    pub language_id: Option<ForeignModel<language>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_abilityflavortext")]
pub struct AbilityFlavorText {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 500)]
    pub flavor_text: String,

    pub ability_id: Option<ForeignModel<Ability>>,

    pub language_id: Option<ForeignModel<language>>,

    pub version_group_id: Option<ForeignModel<versiongroup>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_abilityname")]
pub struct AbilityName {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 200)]
    pub name: String,

    pub ability_id: Option<ForeignModel<Ability>>,

    pub language_id: Option<ForeignModel<language>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_berry")]
pub struct Berry {
    #[rorm(id)]
    pub id: i32,

    pub natural_gift_power: i32,

    pub size: i32,

    pub max_harvest: i32,

    pub growth_time: i32,

    pub soil_dryness: i32,

    pub smoothness: i32,

    pub berry_firmness_id: Option<ForeignModel<BerryFirmness>>,

    pub item_id: Option<ForeignModel<item>>,

    #[rorm(max_length = 200)]
    pub name: String,

    pub natural_gift_type_id: Option<ForeignModel<Type>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_berryfirmness")]
pub struct BerryFirmness {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 200)]
    pub name: String,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_berryfirmnessname")]
pub struct BerryFirmnessName {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 200)]
    pub name: String,

    pub berry_firmness_id: Option<ForeignModel<BerryFirmness>>,

    pub language_id: Option<ForeignModel<language>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_berryflavor")]
pub struct BerryFlavor {
    #[rorm(id)]
    pub id: i32,

    pub context_type_id: Option<ForeignModel<ContestType>>,

    #[rorm(max_length = 200)]
    pub name: String,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_berryflavormap")]
pub struct BerryFlavorMap {
    #[rorm(id)]
    pub id: i32,

    pub potency: i32,

    pub berry_id: Option<ForeignModel<Berry>>,

    pub berry_flavor_id: Option<ForeignModel<BerryFlavor>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_berryflavorname")]
pub struct BerryFlavorName {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 200)]
    pub name: String,

    pub berry_flavor_id: Option<ForeignModel<BerryFlavor>>,

    pub language_id: Option<ForeignModel<language>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_characteristic")]
pub struct Characteristic {
    #[rorm(id)]
    pub id: i32,

    pub gene_mod_5: i32,

    pub stat_id: Option<ForeignModel<stat>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_characteristicdescription")]
pub struct CharacteristicDescription {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 1000)]
    pub description: String,

    pub characteristic_id: Option<ForeignModel<Characteristic>>,

    pub language_id: Option<ForeignModel<language>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_contestcombo")]
pub struct ContestCombo {
    #[rorm(id)]
    pub id: i32,

    pub first_move_id: Option<ForeignModel<Move>>,

    pub second_move_id: Option<ForeignModel<Move>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_contesteffect")]
pub struct ContestEffect {
    #[rorm(id)]
    pub id: i32,

    pub appeal: i32,

    pub jam: i32,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_contesteffecteffecttext")]
pub struct ContestEffectEffectText {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 6000)]
    pub effect: String,

    pub effect_id: Option<ForeignModel<ContestEffect>>,

    pub language_id: Option<ForeignModel<language>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_contesteffectflavortext")]
pub struct ContestEffectFlavorText {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 500)]
    pub flavor_text: String,

    pub contest_effect_id: Option<ForeignModel<ContestEffect>>,

    pub language_id: Option<ForeignModel<language>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_contesttype")]
pub struct ContestType {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 200)]
    pub name: String,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_contesttypename")]
pub struct ContestTypeName {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 200)]
    pub name: String,

    #[rorm(max_length = 10)]
    pub flavor: String,

    #[rorm(max_length = 10)]
    pub color: String,

    pub contest_type_id: Option<ForeignModel<ContestType>>,

    pub language_id: Option<ForeignModel<language>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_egggroup")]
pub struct EggGroup {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 200)]
    pub name: String,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_egggroupname")]
pub struct EggGroupName {
    #[rorm(id)]
    pub id: i32,

    #[rorm(max_length = 200)]
    pub name: String,

    pub egg_group_id: Option<ForeignModel<EggGroup>>,

    pub language_id: Option<ForeignModel<language>>,
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_encounter")]
pub struct encounter {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_encountercondition")]
pub struct encountercondition {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_encounterconditionname")]
pub struct encounterconditionname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_encounterconditionvalue")]
pub struct encounterconditionvalue {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_encounterconditionvaluemap")]
pub struct encounterconditionvaluemap {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_encounterconditionvaluename")]
pub struct encounterconditionvaluename {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_encountermethod")]
pub struct encountermethod {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_encountermethodname")]
pub struct encountermethodname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_encounterslot")]
pub struct encounterslot {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_evolutionchain")]
pub struct evolutionchain {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_evolutiontrigger")]
pub struct evolutiontrigger {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_evolutiontriggername")]
pub struct evolutiontriggername {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_experience")]
pub struct experience {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_gender")]
pub struct gender {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_generation")]
pub struct generation {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_generationname")]
pub struct generationname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_growthrate")]
pub struct growthrate {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_growthratedescription")]
pub struct growthratedescription {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_item")]
pub struct item {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemattribute")]
pub struct itemattribute {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemattributedescription")]
pub struct itemattributedescription {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemattributemap")]
pub struct itemattributemap {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemattributename")]
pub struct itemattributename {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemcategory")]
pub struct itemcategory {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemcategoryname")]
pub struct itemcategoryname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemeffecttext")]
pub struct itemeffecttext {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemflavortext")]
pub struct itemflavortext {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemflingeffect")]
pub struct itemflingeffect {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemflingeffecteffecttext")]
pub struct itemflingeffecteffecttext {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemgameindex")]
pub struct itemgameindex {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemname")]
pub struct itemname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itempocket")]
pub struct itempocket {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itempocketname")]
pub struct itempocketname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_itemsprites")]
pub struct itemsprites {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_language")]
pub struct language {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_languagename")]
pub struct languagename {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_location")]
pub struct location {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_locationarea")]
pub struct locationarea {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_locationareaencounterrate")]
pub struct locationareaencounterrate {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_locationareaname")]
pub struct locationareaname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_locationgameindex")]
pub struct locationgameindex {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_locationname")]
pub struct locationname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_machine")]
pub struct machine {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_move")]
pub struct Move {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_moveattribute")]
pub struct moveattribute {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_moveattributedescription")]
pub struct moveattributedescription {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_moveattributemap")]
pub struct moveattributemap {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_moveattributename")]
pub struct moveattributename {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movebattlestyle")]
pub struct movebattlestyle {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movebattlestylename")]
pub struct movebattlestylename {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movechange")]
pub struct movechange {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movedamageclass")]
pub struct movedamageclass {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movedamageclassdescription")]
pub struct movedamageclassdescription {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movedamageclassname")]
pub struct movedamageclassname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_moveeffect")]
pub struct moveeffect {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_moveeffectchange")]
pub struct moveeffectchange {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_moveeffectchangeeffecttext")]
pub struct moveeffectchangeeffecttext {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_moveeffecteffecttext")]
pub struct moveeffecteffecttext {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_moveflavortext")]
pub struct moveflavortext {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movelearnmethod")]
pub struct movelearnmethod {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movelearnmethoddescription")]
pub struct movelearnmethoddescription {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movelearnmethodname")]
pub struct movelearnmethodname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movemeta")]
pub struct movemeta {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movemetaailment")]
pub struct movemetaailment {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movemetaailmentname")]
pub struct movemetaailmentname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movemetacategory")]
pub struct movemetacategory {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movemetacategorydescription")]
pub struct movemetacategorydescription {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movemetastatchange")]
pub struct movemetastatchange {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movename")]
pub struct movename {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movetarget")]
pub struct movetarget {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movetargetdescription")]
pub struct movetargetdescription {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_movetargetname")]
pub struct movetargetname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_nature")]
pub struct nature {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_naturebattlestylepreference")]
pub struct naturebattlestylepreference {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_naturename")]
pub struct naturename {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_naturepokeathlonstat")]
pub struct naturepokeathlonstat {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_palpark")]
pub struct palpark {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_palparkarea")]
pub struct palparkarea {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_palparkareaname")]
pub struct palparkareaname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokeathlonstat")]
pub struct pokeathlonstat {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokeathlonstatname")]
pub struct pokeathlonstatname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokedex")]
pub struct pokedex {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokedexdescription")]
pub struct pokedexdescription {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokedexname")]
pub struct pokedexname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokedexversiongroup")]
pub struct pokedexversiongroup {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemon")]
pub struct pokemon {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonability")]
pub struct pokemonability {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonabilitypast")]
pub struct pokemonabilitypast {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemoncolor")]
pub struct pokemoncolor {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemoncolorname")]
pub struct pokemoncolorname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemoncries")]
pub struct pokemoncries {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemondexnumber")]
pub struct pokemondexnumber {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonegggroup")]
pub struct pokemonegggroup {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonevolution")]
pub struct pokemonevolution {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonform")]
pub struct pokemonform {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonformgeneration")]
pub struct pokemonformgeneration {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonformname")]
pub struct pokemonformname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonformsprites")]
pub struct pokemonformsprites {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonformtype")]
pub struct pokemonformtype {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemongameindex")]
pub struct pokemongameindex {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonhabitat")]
pub struct pokemonhabitat {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonhabitatname")]
pub struct pokemonhabitatname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonitem")]
pub struct pokemonitem {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonmove")]
pub struct pokemonmove {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonshape")]
pub struct pokemonshape {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonshapename")]
pub struct pokemonshapename {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonspecies")]
pub struct pokemonspecies {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonspeciesdescription")]
pub struct pokemonspeciesdescription {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonspeciesflavortext")]
pub struct pokemonspeciesflavortext {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonspeciesname")]
pub struct pokemonspeciesname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonsprites")]
pub struct pokemonsprites {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemonstat")]
pub struct pokemonstat {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemontype")]
pub struct pokemontype {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_pokemontypepast")]
pub struct pokemontypepast {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_region")]
pub struct region {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_regionname")]
pub struct regionname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_stat")]
pub struct stat {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_statname")]
pub struct statname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_supercontestcombo")]
pub struct supercontestcombo {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_supercontesteffect")]
pub struct supercontesteffect {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_supercontesteffectflavortext")]
pub struct supercontesteffectflavortext {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_type")]
pub struct Type {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_typeefficacy")]
pub struct typeefficacy {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_typeefficacypast")]
pub struct typeefficacypast {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_typegameindex")]
pub struct typegameindex {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_typename")]
pub struct typename {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_typesprites")]
pub struct typesprites {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_version")]
pub struct version {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_versiongroup")]
pub struct versiongroup {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_versiongroupmovelearnmethod")]
pub struct versiongroupmovelearnmethod {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_versiongroupregion")]
pub struct versiongroupregion {
    #[rorm(id)]
    pub id: i32,
    // TODO
}

#[derive(Model)]
#[rorm(rename = "pokemon_v2_versionname")]
pub struct versionname {
    #[rorm(id)]
    pub id: i32,
    // TODO
}
