import std/[httpclient, json, strformat, times]
var client = newHttpClient()

let data = client.getContent("http://ddragon.leagueoflegends.com/cdn/12.6.1/data/en_US/champion.json").parseJson
let version = data["version"]
let champions = data["data"]

var buffer: string
buffer.add(fmt"""
// AUTOGENERATED BY /scripts/champions.nim
// VERSION: {version}
// ON: {now()}

""")

buffer.add("""
pub struct ChampionStats {
    pub hp: f32,
    pub hp_per_level: f32,
    pub mp: f32,
    pub mp_per_level: f32,
    pub move_speed: f32,
    pub armor: f32,
    pub armor_per_level: f32,
    pub spell_block: f32,
    pub spell_block_per_level: f32,
    pub attack_range: f32,
    pub hp_regen: f32,
    pub hp_regen_per_level: f32,
    pub mp_regen: f32,
    pub mp_regen_per_level: f32,
    pub crit: f32,
    pub crit_per_level: f32,
    pub attack_damage: f32,
    pub attack_damage_per_level: f32,
    pub attack_speed: f32,
    pub attack_speed_per_level: f32,
}

impl ChampionStats {
    pub fn new(
        hp: f32,
        hp_per_level: f32,
        mp: f32,
        mp_per_level: f32,
        move_speed: f32,
        armor: f32,
        armor_per_level: f32,
        spell_block: f32,
        spell_block_per_level: f32,
        attack_range: f32,
        hp_regen: f32,
        hp_regen_per_level: f32,
        mp_regen: f32,
        mp_regen_per_level: f32,
        crit: f32,
        crit_per_level: f32,
        attack_damage: f32,
        attack_damage_per_level: f32,
        attack_speed: f32,
        attack_speed_per_level: f32,
    ) -> Self {
        Self {
            hp,
            hp_per_level,
            mp,
            mp_per_level,
            move_speed,
            armor,
            armor_per_level,
            spell_block,
            spell_block_per_level,
            attack_range,
            hp_regen,
            hp_regen_per_level,
            mp_regen,
            mp_regen_per_level,
            crit,
            crit_per_level,
            attack_damage,
            attack_damage_per_level,
            attack_speed,
            attack_speed_per_level,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(i32)]
pub enum Champion {
""")
for champion in champions.keys():
  let id = champions[champion]["id"].getStr
  let key = champions[champion]["key"].getStr

  buffer.add(fmt("    {id} = {key},\n"))

buffer.add("""
    }

impl Champion {    
""")

buffer.add("""
    pub fn as_str(&self) -> &str {
        match self {        
""")

for champion in champions.keys():
  let id = champions[champion]["id"].getStr
  let name = champions[champion]["name"].getStr
  buffer.add(fmt("            Champion::{id} => \"{name}\",\n"))

buffer.add("""
        }
    }
""")

buffer.add("""
    pub fn info(&self) -> ChampionStats {
        match self {        
""")

for champion in champions.keys():
  let id = champions[champion]["id"].getStr
  let info = champions[champion]["stats"]
  let hp = info["hp"].getFloat
  let hpPerLevel = info["hpperlevel"].getFloat
  let mp = info["mp"].getFloat
  let mpPerLevel = info["mpperlevel"].getFloat
  let moveSpeed = info["movespeed"].getFloat
  let armor = info["armor"].getFloat
  let armorPerLevel = info["armorperlevel"].getFloat
  let spellBlock = info["spellblock"].getFloat
  let spellBlockPerLevel = info["spellblockperlevel"].getFloat
  let attackRange = info["attackrange"].getFloat
  let hpRegen = info["hpregen"].getFloat
  let hpRegenPerLevel = info["hpregenperlevel"].getFloat
  let mpRegen = info["mpregen"].getFloat
  let mpRegenPerLevel = info["mpregenperlevel"].getFloat
  let crit = info["crit"].getFloat
  let critPerLevel = info["critperlevel"].getFloat
  let attackDamage = info["attackdamage"].getFloat
  let attackDamagePerLevel = info["attackdamageperlevel"].getFloat
  let attackSpeed = info["attackspeed"].getFloat
  let attackSpeedPerLevel = info["attackspeedperlevel"].getFloat

  buffer.add(fmt"""
              Champion::{id} => ChampionStats {{
                  hp: {hp},
                  hp_per_level: {hpPerLevel},
                  mp: {mp},
                  mp_per_level: {mpPerLevel},
                  move_speed: {moveSpeed},
                  armor: {armor},
                  armor_per_level: {armorPerLevel},
                  spell_block: {spellBlock},
                  spell_block_per_level: {spellBlockPerLevel},
                  attack_range: {attackRange},
                  hp_regen: {hpRegen},
                  hp_regen_per_level: {hpRegenPerLevel},
                  mp_regen: {mpRegen},
                  mp_regen_per_level: {mpRegenPerLevel},
                  crit: {crit},
                  crit_per_level: {critPerLevel},
                  attack_damage: {attackDamage},
                  attack_damage_per_level: {attackDamagePerLevel},
                  attack_speed: {attackSpeed},
                  attack_speed_per_level: {attackSpeedPerLevel},
              }},
  """)

buffer.add("""
        }
    }
""")

buffer.add("""
}     
""")

writeFile("../crates/kassatypes/src/champions.rs", buffer)