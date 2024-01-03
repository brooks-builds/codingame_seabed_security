fn main() {
    let mut game_state = GameState::init();

    loop {
        let commands = game_state.turn();
        commands.iter().for_each(|command| println!("{command}"));
    }
}

#[derive(Debug)]
pub struct GameState {
    creatures: std::collections::HashMap<i32, Creature>,
    my_score: i32,
    foe_score: i32,
    my_drones: Vec<Drone>,
    foe_drones: Vec<Drone>,
    turns: usize,
}

impl GameState {
    pub fn init() -> Self {
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let creature_count = input_line.trim().parse().unwrap();

        let mut creatures = std::collections::HashMap::new();

        for _ in 0..creature_count {
            let mut input_line = String::new();
            std::io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.trim().split(' ').collect::<Vec<_>>();
            let creature: Creature = inputs.into();

            creatures.insert(creature.id, creature);
        }

        Self {
            creatures,
            my_score: 0,
            foe_score: 0,
            my_drones: vec![],
            foe_drones: vec![],
            turns: 0,
        }
    }

    pub fn turn(&mut self) -> Vec<Command> {
        self.turns += 1;
        self.get_input();

        let light = self.turns % 10 == 0;
        // let light = false;

        let (command, target_id) = 'drone_1_command: {
            let drone = &self.my_drones[0];

            if !drone.scans.is_empty() {
                break 'drone_1_command (
                    Command::Move {
                        x: drone.x,
                        y: 500,
                        light: false,
                    },
                    None,
                );
            }

            for radar_blip in drone.radar_blips.iter() {
                let radar_fish_id = radar_blip.creature_id;
                let direction = &radar_blip.direction;

                let Some(fish) = self.creatures.get(&radar_fish_id) else {
                    let [x, y] = drone.move_direction(direction);
                    eprintln!("didn't find fish, moving towards: {x} {y}");
                    break 'drone_1_command (Command::Move { x, y, light }, None);
                };

                if !fish.me_scan && fish.creature_type >= 0 {
                    let [x, y] = drone.move_direction(direction);
                    break 'drone_1_command (Command::Move { x, y, light }, Some(fish.id));
                }
            }
            (
                Command::Move {
                    x: 5000,
                    y: 0,
                    light,
                },
                None,
            )
        };

        let command_2 = 'drone_2_command: {
            let drone = &self.my_drones[1];

            if !drone.scans.is_empty() {
                break 'drone_2_command Command::Move {
                    x: drone.x,
                    y: 500,
                    light: false,
                };
            }

            for radar_blip in drone.radar_blips.iter() {
                let radar_fish_id = radar_blip.creature_id;
                if radar_fish_id == target_id.unwrap_or_default() {
                    continue;
                }

                let direction = &radar_blip.direction;

                let Some(fish) = self.creatures.get(&radar_fish_id) else {
                    let [x, y] = drone.move_direction(direction);
                    eprintln!("didn't find fish, moving towards: {x} {y}");
                    break 'drone_2_command Command::Move { x, y, light };
                };

                if !fish.me_scan && fish.creature_type >= 0 {
                    let [x, y] = drone.move_direction(direction);
                    break 'drone_2_command Command::Move { x, y, light };
                }
            }
            Command::Move {
                x: 5000,
                y: 0,
                light,
            }
        };

        vec![command, command_2]
    }

    fn get_input(&mut self) {
        self.my_score = self.get_score();
        self.foe_score = self.get_score();

        let my_scan_count = self.get_count();
        let _my_scanned_creature_ids = self.get_ids(my_scan_count);

        let foe_scan_count = self.get_count();
        let _foe_scanned_creature_ids = self.get_ids(foe_scan_count);

        let my_drone_count = self.get_count();
        self.my_drones.clear();
        for _ in 0..my_drone_count {
            let my_drone = Drone::new_from_input(true);
            self.my_drones.push(my_drone);
        }

        let foe_drone_count = self.get_count();
        for _ in 0..foe_drone_count {
            let foe_drone = Drone::new_from_input(false);
            self.foe_drones.push(foe_drone);
        }

        let drone_scan_count = self.get_count();
        for _ in 0..drone_scan_count {
            let scans = self.get_vector();
            let drone_id = scans[0];
            let Some(drone) = self
                .my_drones
                .iter_mut()
                .find(move |drone| drone.id == drone_id)
            else {
                continue;
            };

            drone.scans.push(scans[1]);
            let fish_id = scans[1];
            self.mark_creatures_as_scanned(vec![fish_id], true);
        }

        let visible_creature_count = self.get_count();
        for _ in 0..visible_creature_count {
            let creature_inputs = self.get_vector();

            let creature = self
                .creatures
                .entry(creature_inputs[0])
                .or_insert(Creature::default());

            creature.id = creature_inputs[0];
            creature.x = creature_inputs[1];
            creature.y = creature_inputs[2];
            creature.xv = creature_inputs[3];
            creature.yv = creature_inputs[4];
        }

        let radar_blip_count = self.get_count();
        self.my_drones
            .iter_mut()
            .for_each(|drone| drone.radar_blips.clear());
        for _ in 0..radar_blip_count {
            let radar_blips = self.get_input_line();
            let radar_blips = radar_blips.trim().split(' ').collect::<Vec<_>>();
            let drone_id = radar_blips[0]
                .parse::<i32>()
                .expect("Error getting radar blip drone id");
            let creature_id = radar_blips[1]
                .parse::<i32>()
                .expect("Error getting radar blip creature id");
            let direction = radar_blips[2].to_owned();

            let Some(drone) = self
                .my_drones
                .iter_mut()
                .find(move |drone| drone.id == drone_id)
            else {
                eprintln!("cannot find my drone :(");
                continue;
            };

            drone.radar_blips.push(RadarBlip {
                creature_id,
                direction,
            });
        }
    }

    fn get_score(&self) -> i32 {
        self.get_input_line().trim().parse().unwrap()
    }

    fn get_count(&self) -> i32 {
        self.get_input_line().trim().parse().unwrap_or(0)
    }

    fn get_ids(&self, scan_count: i32) -> Vec<i32> {
        let mut ids = vec![];

        for _ in 0..scan_count {
            ids.push(self.get_input_line().trim().parse().unwrap_or_default());
        }

        ids
    }

    fn get_id(&self) -> i32 {
        self.get_input_line().trim().parse().unwrap()
    }

    fn get_input_line(&self) -> String {
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).unwrap();
        input_line
    }

    fn get_vector(&self) -> Vec<i32> {
        self.get_input_line()
            .trim()
            .split(' ')
            .map(|coordinate| coordinate.parse().unwrap())
            .collect()
    }

    fn get_battery_level(&self) -> i32 {
        self.get_input_line().trim().parse().unwrap()
    }

    pub fn mark_creatures_as_scanned(&mut self, ids: Vec<i32>, my_scan: bool) {
        for id in ids {
            let Some(creature) = self.creatures.get_mut(&id) else {
                continue;
            };

            if my_scan {
                creature.me_scan = true;
            } else {
                creature.foe_scan = true;
            }
        }
    }

    fn get_drone_count(&self) -> i32 {
        self.get_input_line()
            .trim()
            .parse()
            .expect("error getting drone count")
    }

    fn find_unscanned_fish(&self) -> Option<&Creature> {
        for creature in self.creatures.values() {
            if !creature.me_scan {
                return Some(creature);
            }
        }

        None
    }
}

pub enum Command {
    Move { x: i32, y: i32, light: bool },
    Wait { light: bool },
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Move { x, y, light } => write!(f, "MOVE {x} {y} {}", if *light { 1 } else { 0 }),
            Self::Wait { light } => write!(f, "WAIT {}", if *light { 1 } else { 0 }),
        }
    }
}

#[derive(Debug, Default)]
pub struct Creature {
    pub id: i32,
    pub color: i32,
    pub creature_type: i32,
    pub me_scan: bool,
    pub foe_scan: bool,
    pub x: i32,
    pub y: i32,
    pub xv: i32,
    pub yv: i32,
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

#[derive(Debug, Default)]
pub struct Drone {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    emergency: i32,
    pub battery: i32,
    pub mine: bool,
    radar_blips: Vec<RadarBlip>,
    scans: Vec<i32>,
}

impl Drone {
    pub fn new_from_input(mine: bool) -> Self {
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.trim().split(' ').collect::<Vec<_>>();

        Self {
            id: inputs[0].parse().unwrap(),
            x: inputs[1].parse().unwrap(),
            y: inputs[2].parse().unwrap(),
            emergency: inputs[3].parse().unwrap(),
            battery: inputs[4].parse().unwrap(),
            mine,
            radar_blips: vec![],
            scans: vec![],
        }
    }

    pub fn move_direction(&self, direction: &str) -> [i32; 2] {
        eprintln!("Moving towards: {direction} from {} {}", self.x, self.y);
        match direction {
            "TL" => [self.x - 600, self.y - 600],
            "TR" => [self.x + 600, self.y - 600],
            "BL" => [self.x - 600, self.y + 600],
            "BR" => [self.x + 600, self.y + 600],
            _ => [self.x, self.y],
        }
    }
}

#[derive(Debug, Default)]
struct RadarBlip {
    creature_id: i32,
    direction: String,
}
