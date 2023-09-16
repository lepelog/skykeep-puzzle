use std::collections::{hash_map::Entry, HashMap, HashSet};

use enum_iterator::Sequence;
use rand::{seq::SliceRandom, SeedableRng};

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct OpenedGates : u8 {
        const STARTING = 1 << 0;
        const EARTH_TEMPLE = 1 << 1;
        const MINI_BOSS = 1 << 2;
        const FIRE_SANCTUARY = 1 << 3;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }

    pub fn tile_move(&self) -> isize {
        match self {
            Direction::Up => -3,
            Direction::Left => -1,
            Direction::Down => 3,
            Direction::Right => 1,
        }
    }
}

#[derive(Debug, Sequence, Clone, Copy, PartialEq, Eq)]
pub enum ControlPanel {
    Start,
    LanayruMiningFacility,
    EarthTemple,
    MiniBoss,
}

impl ControlPanel {
    pub fn entrance(&self) -> Entrance {
        match self {
            ControlPanel::Start => Entrance::StartDown,
            ControlPanel::LanayruMiningFacility => Entrance::LanayruMiningFacilityDown,
            ControlPanel::EarthTemple => Entrance::EarthTempleDown,
            ControlPanel::MiniBoss => Entrance::MiniBossLeft,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence, Hash)]
pub enum Room {
    Start,
    Skyview,
    EarthTemple,
    LanayruMiningFacility,
    MiniBoss,
    AncientCistern,
    FireSanctuary,
    Sandship,
    Empty,
}

pub fn do_move(tile: u8, direction: Direction) -> Option<(u8, Direction)> {
    match direction {
        Direction::Up => {
            if tile < 3 {
                None
            } else {
                Some((tile - 3, Direction::Down))
            }
        }
        Direction::Left => {
            if [0, 3, 6].contains(&tile) {
                None
            } else {
                Some((tile - 1, Direction::Right))
            }
        }
        Direction::Down => {
            if tile >= 6 {
                None
            } else {
                Some((tile + 3, Direction::Up))
            }
        }
        Direction::Right => {
            if [2, 5, 8].contains(&tile) {
                None
            } else {
                Some((tile + 1, Direction::Left))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoomAndPos {
    rooms: [Room; 9],
    pos_tile: u8,
    pos_direction: Direction,
}

#[derive(Debug, Sequence)]
pub enum Operations {
    Reach(ControlPanel),
    Move(Direction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence, Hash)]
pub enum Entrance {
    StartDown,
    StartRight,
    SkyviewLeft,
    SkyviewUp,
    EarthTempleRight,
    EarthTempleDown,
    LanayruMiningFacilityDown,
    LanayruMiningFacilityUp,
    MiniBossLeft,
    MiniBossDown,
    AncientCisternRight,
    AncientCisternDown,
    FireSanctuaryLeft,
    FireSanctuaryRight,
    SandshipLeft,
}

impl Entrance {
    pub fn from_room_direction(room: Room, direction: Direction) -> Option<Self> {
        use Entrance::*;
        Some(match (room, direction) {
            (Room::Start, Direction::Down) => StartDown,
            (Room::Start, Direction::Right) => StartRight,
            (Room::Skyview, Direction::Up) => SkyviewUp,
            (Room::Skyview, Direction::Left) => SkyviewLeft,
            (Room::EarthTemple, Direction::Down) => EarthTempleDown,
            (Room::EarthTemple, Direction::Right) => EarthTempleRight,
            (Room::LanayruMiningFacility, Direction::Up) => LanayruMiningFacilityUp,
            (Room::LanayruMiningFacility, Direction::Down) => LanayruMiningFacilityDown,
            (Room::MiniBoss, Direction::Left) => MiniBossLeft,
            (Room::MiniBoss, Direction::Down) => MiniBossDown,
            (Room::AncientCistern, Direction::Down) => AncientCisternDown,
            (Room::AncientCistern, Direction::Right) => AncientCisternRight,
            (Room::FireSanctuary, Direction::Left) => FireSanctuaryLeft,
            (Room::FireSanctuary, Direction::Right) => FireSanctuaryRight,
            (Room::Sandship, Direction::Left) => SandshipLeft,
            _ => return None,
        })
    }

    pub fn traverse_room(&self, gates: OpenedGates) -> Option<Entrance> {
        use Entrance::*;
        match self {
            Entrance::StartDown => Some(StartRight),
            Entrance::StartRight => gates.contains(OpenedGates::STARTING).then_some(StartDown),
            Entrance::SkyviewLeft => Some(SkyviewUp),
            Entrance::SkyviewUp => Some(SkyviewLeft),
            Entrance::EarthTempleRight => gates
                .contains(OpenedGates::EARTH_TEMPLE)
                .then_some(EarthTempleDown),
            Entrance::EarthTempleDown => Some(EarthTempleRight),
            Entrance::LanayruMiningFacilityDown => Some(LanayruMiningFacilityUp),
            Entrance::LanayruMiningFacilityUp => Some(LanayruMiningFacilityDown),
            Entrance::MiniBossLeft => gates
                .contains(OpenedGates::MINI_BOSS)
                .then_some(MiniBossDown),
            Entrance::MiniBossDown => Some(MiniBossLeft),
            Entrance::AncientCisternRight => Some(AncientCisternDown),
            Entrance::AncientCisternDown => Some(AncientCisternRight),
            Entrance::FireSanctuaryLeft => gates
                .contains(OpenedGates::FIRE_SANCTUARY)
                .then_some(FireSanctuaryRight),
            Entrance::FireSanctuaryRight => Some(FireSanctuaryLeft),
            Entrance::SandshipLeft => None,
        }
    }

    pub fn to_room_direction(&self) -> (Room, Direction) {
        use Entrance::*;
        match self {
            StartDown => (Room::Start, Direction::Down),
            StartRight => (Room::Start, Direction::Right),
            SkyviewUp => (Room::Skyview, Direction::Up),
            SkyviewLeft => (Room::Skyview, Direction::Left),
            EarthTempleDown => (Room::EarthTemple, Direction::Down),
            EarthTempleRight => (Room::EarthTemple, Direction::Right),
            LanayruMiningFacilityUp => (Room::LanayruMiningFacility, Direction::Up),
            LanayruMiningFacilityDown => (Room::LanayruMiningFacility, Direction::Down),
            MiniBossLeft => (Room::MiniBoss, Direction::Left),
            MiniBossDown => (Room::MiniBoss, Direction::Down),
            AncientCisternDown => (Room::AncientCistern, Direction::Down),
            AncientCisternRight => (Room::AncientCistern, Direction::Right),
            FireSanctuaryLeft => (Room::FireSanctuary, Direction::Left),
            FireSanctuaryRight => (Room::FireSanctuary, Direction::Right),
            SandshipLeft => (Room::Sandship, Direction::Left),
        }
    }

    pub fn has_control_panel(&self) -> bool {
        use Entrance::*;
        matches!(
            self,
            StartRight | LanayruMiningFacilityDown | EarthTempleDown | MiniBossLeft
        )
    }

    pub fn open_gate(&self) -> Option<OpenedGates> {
        match self {
            Entrance::StartDown => Some(OpenedGates::STARTING),
            Entrance::EarthTempleDown => Some(OpenedGates::EARTH_TEMPLE),
            Entrance::MiniBossDown => Some(OpenedGates::MINI_BOSS),
            Entrance::FireSanctuaryRight => Some(OpenedGates::FIRE_SANCTUARY),
            _ => None,
        }
    }
}

fn main() {
    let mut rng = rand_pcg::Pcg64::from_entropy();
    let mut rooms = [
        Room::Start,
        Room::Skyview,
        Room::EarthTemple,
        Room::LanayruMiningFacility,
        Room::MiniBoss,
        Room::AncientCistern,
        Room::FireSanctuary,
        Room::Sandship,
        Room::Empty,
    ];
    rooms.shuffle(&mut rng);

    print_rooms(&rooms);
    match verify_rooms(&rooms) {
        Ok(()) => {
            println!("beatable: {rooms:?}");
        }
        Err(e) => {
            println!("not beatable ({e}): {rooms:?}");
        }
    }
}

fn verify_rooms(rooms: &[Room; 9]) -> Result<(), &'static str> {
    // print_rooms(rooms);
    // check that we can enter at all
    let Some(_) = Entrance::from_room_direction(rooms[7], Direction::Down) else {
        return Err("no down first room");
    };
    // we need to find any control panel
    let Some((panel_dir, panel_tile)) = follow_chain(
        rooms,
        OpenedGates::empty(),
        7,
        Direction::Down,
        &mut |entrance, tile| {
            entrance
                .has_control_panel()
                .then_some((entrance.to_room_direction().1, tile))
        },
    ) else {
        return Err("no control panel");
    };

    let mut state_to_gate: HashMap<RoomAndPos, OpenedGates> = HashMap::new();

    // let mut counter: usize = 0;
    // let mut max_depth = 0;
    let mut unreachable_entrances: HashSet<Entrance> = enum_iterator::all::<Entrance>().collect();
    let mut stash: Vec<(RoomAndPos, Operations)> = Vec::new();

    let mut current_pos_room = RoomAndPos {
        pos_tile: panel_tile,
        pos_direction: panel_dir,
        rooms: *rooms,
    };

    let mut current_operation: Operations = Operations::first().unwrap();
    let mut current_gates = OpenedGates::empty();
    let beatable = 'main_loop: loop {
        // max_depth = max_depth.max(stash.len());
        // counter += 1;
        // if (counter % 10000) == 0 {
        //     println!("{counter}, {}", state_to_gate.len());
        //     print_rooms(&current_pos_room.rooms);
        // }
        // perform operation
        let op_result = match current_operation {
            Operations::Reach(panel) => {
                let panel_entrance = panel.entrance();
                if let Some(panel_tile) = follow_chain_both(
                    &current_pos_room.rooms,
                    current_gates,
                    current_pos_room.pos_tile,
                    current_pos_room.pos_direction,
                    &mut |entrance, tile| (panel_entrance == entrance).then_some(tile),
                ) {
                    Ok(RoomAndPos {
                        rooms: current_pos_room.rooms,
                        pos_direction: panel_entrance.to_room_direction().1,
                        pos_tile: panel_tile,
                    })
                } else {
                    Err(())
                }
            }
            Operations::Move(direction) => {
                // if we move up into the empty space, we swap with the tile that is down
                let empty_tile = current_pos_room
                    .rooms
                    .iter()
                    .position(|r| r == &Room::Empty)
                    .unwrap() as u8;
                if let Some((other_tile, _)) = do_move(empty_tile, direction) {
                    if other_tile != current_pos_room.pos_tile {
                        let mut rooms = current_pos_room.rooms;
                        rooms.swap(other_tile.into(), empty_tile.into());
                        Ok(RoomAndPos {
                            rooms,
                            pos_tile: current_pos_room.pos_tile,
                            pos_direction: current_pos_room.pos_direction,
                        })
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            }
        };
        match op_result {
            // operation could be performed, see if this is a new state or if we can reach more gates now
            Ok(new_room_pos) => {
                // try to open gates and reach entrances
                follow_chain_both::<()>(
                    &new_room_pos.rooms,
                    current_gates,
                    new_room_pos.pos_tile,
                    new_room_pos.pos_direction,
                    &mut |e, _| {
                        if let Some(gate) = e.open_gate() {
                            current_gates |= gate;
                        }
                        unreachable_entrances.remove(&e);
                        None
                    },
                );
                if unreachable_entrances.is_empty() {
                    break true;
                }
                match state_to_gate.entry(new_room_pos.clone()) {
                    Entry::Occupied(mut occupied) => {
                        if occupied.get().contains(current_gates) {
                            // we already found this state, with better gates
                            // copied from err segment
                            if let Some(nex_op) = current_operation.next() {
                                current_operation = nex_op;
                                continue 'main_loop;
                            } else {
                                while let Some((stack_room_pos, stack_op)) = stash.pop() {
                                    if let Some(next_op) = stack_op.next() {
                                        current_pos_room = stack_room_pos;
                                        current_operation = next_op;
                                        current_gates = state_to_gate
                                            .get(&current_pos_room)
                                            .cloned()
                                            .unwrap_or(OpenedGates::empty());
                                        continue 'main_loop;
                                    }
                                }
                                // we have reached the end of the stack
                                break false;
                            }
                        } else {
                            // we have better gates now, continue
                            occupied.insert(current_gates);
                        }
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert(current_gates);
                    }
                }
                // this is now our new state, push the current one to the stack and restart operation
                stash.push((new_room_pos.clone(), current_operation));
                current_operation = Operations::first().unwrap();
                current_pos_room = new_room_pos;
            }
            // operation couldn't be performed, try the next one
            // if there isn't one, pop one from the stack
            // if there isn't one, we're done
            Err(()) => {
                if let Some(nex_op) = current_operation.next() {
                    current_operation = nex_op;
                    continue 'main_loop;
                } else {
                    while let Some((stack_room_pos, stack_op)) = stash.pop() {
                        if let Some(next_op) = stack_op.next() {
                            current_pos_room = stack_room_pos;
                            current_operation = next_op;
                            current_gates = state_to_gate
                                .get(&current_pos_room)
                                .cloned()
                                .unwrap_or(OpenedGates::empty());
                            continue 'main_loop;
                        }
                    }
                    // we have reached the end of the stack
                    break false;
                }
            }
        }
    };

    // let beatable = verify_rec(&mut state_to_gate, pos_room, gates, &mut counter, &mut unreachable_entrances);

    // println!("count: {counter}");
    // println!("depth: {max_depth}");
    // println!("beatable: {}", unreachable_entrances.is_empty());

    if beatable {
        Ok(())
    } else {
        Err("unreachable entrances")
    }
}

fn follow_chain_both<T>(
    rooms: &[Room; 9],
    gates: OpenedGates,
    tile: u8,
    direction: Direction,
    check: &mut impl FnMut(Entrance, u8) -> Option<T>,
) -> Option<T> {
    follow_chain(rooms, gates, tile, direction, check).or_else(|| {
        if let Some((tile, direction)) = do_move(tile, direction) {
            follow_chain(rooms, gates, tile, direction, check)
        } else {
            None
        }
    })
}

fn follow_chain<T>(
    rooms: &[Room; 9],
    gates: OpenedGates,
    mut tile: u8,
    mut direction: Direction,
    check: &mut impl FnMut(Entrance, u8) -> Option<T>,
) -> Option<T> {
    loop {
        let Some(pos) = Entrance::from_room_direction(rooms[tile as usize], direction) else {
            return None;
        };
        if let Some(val) = check(pos, tile) {
            return Some(val);
        }
        let Some(pos) = pos.traverse_room(gates) else {
            return None;
        };
        if let Some(val) = check(pos, tile) {
            return Some(val);
        }
        direction = pos.to_room_direction().1;
        if let Some((new_tile, new_dir)) = do_move(tile, direction) {
            tile = new_tile;
            direction = new_dir;
        } else {
            return None;
        };
    }
}

fn print_rooms(rooms: &[Room; 9]) {
    fn room_str(r: Room) -> &'static str {
        match r {
            Room::Start => "STR",
            Room::Skyview => "SV ",
            Room::EarthTemple => "ET ",
            Room::LanayruMiningFacility => "LMF",
            Room::MiniBoss => "BOS",
            Room::AncientCistern => "AC ",
            Room::FireSanctuary => "FS ",
            Room::Sandship => "SSH",
            Room::Empty => "   ",
        }
    }
    for chunk in rooms.chunks_exact(3) {
        for r in chunk {
            print!("{} ", room_str(*r));
        }
        println!();
    }
}
