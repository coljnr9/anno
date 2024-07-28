use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DndApiReply {
    pub index: String,
    pub name: String,
    pub desc: Vec<String>,
    #[serde(rename = "higher_level")]
    pub higher_level: Vec<String>,
    pub range: String,
    pub components: Vec<String>,
    pub material: String,
    pub ritual: bool,
    pub duration: String,
    pub concentration: bool,
    #[serde(rename = "casting_time")]
    pub casting_time: String,
    pub level: i64,
    #[serde(rename = "attack_type")]
    pub attack_type: String,
    pub damage: Damage,
    pub school: School,
    pub classes: Vec<Class>,
    pub subclasses: Vec<Subclass>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    #[serde(rename = "damage_type")]
    pub damage_type: DamageType,
    #[serde(rename = "damage_at_slot_level")]
    pub damage_at_slot_level: DamageAtSlotLevel,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageType {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageAtSlotLevel {
    #[serde(rename = "2")]
    pub n2: String,
    #[serde(rename = "3")]
    pub n3: String,
    #[serde(rename = "4")]
    pub n4: String,
    #[serde(rename = "5")]
    pub n5: String,
    #[serde(rename = "6")]
    pub n6: String,
    #[serde(rename = "7")]
    pub n7: String,
    #[serde(rename = "8")]
    pub n8: String,
    #[serde(rename = "9")]
    pub n9: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct School {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subclass {
    pub index: String,
    pub name: String,
    pub url: String,
}
