#[derive(Debug)]
pub struct Creature {
    pub id: u8,
    pub color: u8,
    pub creature_type: u8,
    pub me_scan: bool,
    pub foe_scan: bool,
    pub x: u32,
    pub y: u32,
    pub xv: u32,
    pub yv: u32,
}

impl From<Vec<&str>> for Creature {
    fn from(values: Vec<&str>) -> Self {
        let id = values[0].parse().unwrap();
        let color = values[1].parse().unwrap();
        let creature_type = values[2].parse().unwrap();

        Self {
            id,
            color,
            creature_type,
            me_scan: false,
            foe_scan: false,
            x: 0,
            y: 0,
            xv: 0,
            yv: 0,
        }
    }
}
