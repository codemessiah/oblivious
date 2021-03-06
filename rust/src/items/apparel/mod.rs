use super::*;
use super::super::definitions::*;

use std::fmt;

pub trait Apparel: Item {
    fn position(&self) -> ApparelPos;
}

impl fmt::Debug for Apparel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ApparelPos {
    Head,
    Torso,
    Hands,
    Feet,
}

#[derive(Clone)]
pub struct ApparelPlacement {
    pub head: Option<&'static Apparel>,
    pub torso: Option<&'static Apparel>,
    pub hands: Option<&'static Apparel>,
    pub feet: Option<&'static Apparel>,
}

impl ApparelPlacement {
    pub fn new() -> Self {
        Self { head: None, torso: None, hands: None, feet: None }
    }

    fn dequip_garment(&mut self, garment: &'static Apparel) -> &'static Apparel {
        use self::ApparelPos::*;
        match garment.position() {
            Head => self.head = None,
            Torso => self.torso = None,
            Hands => self.hands = None,
            Feet => self.feet = None,
        }
        garment
    }

    pub fn dequip(&mut self, position: ApparelPos) -> Option<Vec<&'static Apparel>> {
        use self::ApparelPos::*;
        match position {
            Head => match self.head {
                Some(garment) => Some(vec!(self.dequip_garment(garment))),
                _ => None
            }
            Torso => match self.torso {
                Some(garment) => Some(vec!(self.dequip_garment(garment))),
                _ => None
            }
            Hands => match self.hands {
                Some(garment) => Some(vec!(self.dequip_garment(garment))),
                _ => None
            }
            Feet => match self.feet {
                Some(garment) => Some(vec!(self.dequip_garment(garment))),
                _ => None
            }
        }
    }

    pub fn equip(&mut self, garment: &'static Apparel) -> Option<Vec<&'static Apparel>> {
        use self::ApparelPos::*;
        match garment.position() {
            Head => match self.head {
                Some(other) => {
                    let ret = self.dequip_garment(other);
                    self.head = Some(garment);
                    Some(vec!(ret))
                }
                None => {
                    self.head = Some(garment);
                    None
                }
            }
            Torso => match self.torso {
                Some(other) => {
                    let ret = self.dequip_garment(other);
                    self.torso = Some(garment);
                    Some(vec!(ret))
                }
                None => {
                    self.torso = Some(garment);
                    None
                }
            }
            Hands => match self.hands {
                Some(other) => {
                    let ret = self.dequip_garment(other);
                    self.hands = Some(garment);
                    Some(vec!(ret))
                }
                None => {
                    self.hands = Some(garment);
                    None
                }
            }
            Feet => match self.feet {
                Some(other) => {
                    let ret = self.dequip_garment(other);
                    self.feet = Some(garment);
                    Some(vec!(ret))
                }
                None => {
                    self.feet = Some(garment);
                    None
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Clothing {
    name: &'static str,
    position: ApparelPos,
    weight: u16,
    value: u16,
}

impl Item for Clothing {
    fn name(&self) -> &str {
        &self.name
    }

    fn weight(&self) -> u16 {
        self.weight
    }

    fn value(&self) -> u16 {
        self.value
    }

    fn intrinsic(&self) -> ItemType {
        ItemType::Clothing(&self)
    }
}

impl Apparel for Clothing {
    fn position(&self) -> ApparelPos {
        self.position
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Armor {
    name: &'static str,
    position: ApparelPos,
    base_armor: u16,
    weight: u16,
    value: u16,
}

impl Armor {
    pub fn armor(&self) -> u16 {
        self.base_armor
    }
}

impl Item for Armor {
    fn name(&self) -> &str {
        &self.name
    }

    fn weight(&self) -> u16 {
        self.weight
    }

    fn value(&self) -> u16 {
        self.value
    }

    fn intrinsic(&self) -> ItemType {
        ItemType::Armor(&self)
    }
}

impl Apparel for Armor {
    fn position(&self) -> ApparelPos {
        self.position
    }
}

// GENERIC

pub static FOOTWRAPS: Clothing = Clothing {
    name: "Footwraps",
    position: ApparelPos::Feet,
    weight: 1,
    value: 1,
};

pub static ROUGHSPUN_TUNIC: Clothing = Clothing {
    name: "Roughspun Tunic",
    position: ApparelPos::Torso,
    weight: 1,
    value: 1,
};

// STORMCLOAK

pub static STORMCLOAK_CUIRASS: Armor = Armor {
    name: "Stormcloak Cuirass",
    position: ApparelPos::Torso,
    base_armor: 23,
    weight: 8,
    value: 25,
};

// IMPERIAL

pub static IMPERIAL_LIGHT_ARMOR: Armor = Armor {
    name: "Imperial Light Armor",
    position: ApparelPos::Torso,
    base_armor: 23,
    weight: 6,
    value: 75,
};

pub static IMPERIAL_LIGHT_BOOTS: Armor = Armor {
    name: "Imperial Light Boots",
    position: ApparelPos::Feet,
    base_armor: 7,
    weight: 2,
    value: 15,
};

pub static IMPERIAL_LIGHT_BRACERS: Armor = Armor {
    name: "Imperial Light Bracers",
    position: ApparelPos::Hands,
    base_armor: 7,
    weight: 1,
    value: 15,
};

pub static IMPERIAL_LIGHT_HELMET: Armor = Armor {
    name: "Imperial Light Helmet",
    position: ApparelPos::Head,
    base_armor: 12,
    weight: 2,
    value: 35,
};