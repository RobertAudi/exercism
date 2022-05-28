// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        (self.health <= 0).then(|| Self {
            health: 100,
            level: self.level,
            mana: (self.level >= 10).then(|| 100),
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.level < 10 {
            self.health = if self.health >= mana_cost {
                self.health - mana_cost
            } else {
                0
            };

            0
        } else {
            match self.mana {
                Some(x) => {
                    if x >= mana_cost {
                        self.mana = Some(x - mana_cost);
                        mana_cost * 2
                    } else {
                        0
                    }
                }
                None => 0,
            }
        }
    }
}
