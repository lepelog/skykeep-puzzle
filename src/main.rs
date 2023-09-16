use std::collections::{HashMap, hash_map::Entry, HashSet};

use enum_iterator::Sequence;
use rand::{SeedableRng, seq::SliceRandom};

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct OpenedGates : u8 {
        const STARTING = 1 << 0;
        const EARTH_TEMPLE = 1 << 1;
        const MINI_BOSS = 1 << 2;
        const FIRE_SANCTUARY = 1 << 3;
        const NON_EMPTY = 1 << 4;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
pub enum PuzzleMover {
    Start,
    LanayruMiningFacility,
    EarthTemple,
    MiniBoss,
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
        Direction::Up => if tile < 3 {
            None
        } else {
            Some((tile - 3, Direction::Down))
        },
        Direction::Left => if [0, 3, 6].contains(&tile) {
            None
        } else {
            Some((tile - 1, Direction::Right))
        },
        Direction::Down => if tile >= 6 {
            None
        } else {
            Some((tile + 3, Direction::Up))
        },
        Direction::Right => if [2, 5, 8].contains(&tile) {
            None
        } else {
            Some((tile + 1, Direction::Left))
        },
    }
}

// impl Room {
//     pub fn directions(&self) -> Direction {
//         match self {
//             Room::Start => Direction::Right | Direction::Down,
//             Room::Skyview => Direction::Left | Direction::Up,
//             Room::EarthTemple => Direction::Right | Direction::Down,
//             Room::LanayruMiningFacility => Direction::Down | Direction::Up,
//             Room::MiniBoss => Direction::Left | Direction::Down,
//             Room::AncientCistern => Direction::Right | Direction::Down,
//             Room::FireSanctuary => Direction::Left | Direction::Right,
//             Room::Sandship => Direction::Left,
//             Room::Empty => Direction::empty(),
//         }
//     }
// }

// #[repr(align(16))]
pub struct UniqueState {
    rooms: [Room; 9],
    pos_tile: u8,
    pos_direction: Direction,
    gates: OpenedGates,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoomAndPos {
    rooms: [Room; 9],
    pos_tile: u8,
    pos_direction: Direction,
}

#[derive(Debug, Sequence)]
pub enum Operations {
    ReachMoverStart,
    ReachMoverLanayruMiningFacility,
    ReachMoverEarthTemple,
    ReachMoverMiniBoss,
    MoveUp,
    MoveLeft,
    MoveDown,
    MoveRight,
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
            Entrance::EarthTempleRight => gates.contains(OpenedGates::EARTH_TEMPLE).then_some(EarthTempleDown),
            Entrance::EarthTempleDown => Some(EarthTempleRight),
            Entrance::LanayruMiningFacilityDown => Some(LanayruMiningFacilityUp),
            Entrance::LanayruMiningFacilityUp => Some(LanayruMiningFacilityDown),
            Entrance::MiniBossLeft => gates.contains(OpenedGates::MINI_BOSS).then_some(MiniBossDown),
            Entrance::MiniBossDown => Some(MiniBossLeft),
            Entrance::AncientCisternRight => Some(AncientCisternDown),
            Entrance::AncientCisternDown => Some(AncientCisternRight),
            Entrance::FireSanctuaryLeft => gates.contains(OpenedGates::FIRE_SANCTUARY).then_some(FireSanctuaryRight),
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
            EarthTempleRight =>(Room::EarthTemple, Direction::Right),
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
        matches!(self, StartRight | LanayruMiningFacilityDown | EarthTempleDown | MiniBossLeft)
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

    match verify_rooms(&rooms) {
        Ok(()) => {
            println!("possible: {:?}", rooms);
        },
        Err(e) => {
            println!("impossible ({}): {:?}", e, rooms);
        }
    }
}

fn verify_rooms(rooms: &[Room; 9]) -> Result<(), &'static str> {
    print_rooms(rooms);
    // check that we can enter at all
    let Some(entrance) = Entrance::from_room_direction(rooms[7], Direction::Down) else {
        return Err("no down first room");
    };
    println!("{:?}", entrance);
    // we need to find any control panel
    let Some((panel, panel_tile)) = follow_chain(rooms, OpenedGates::empty(), 7, Direction::Down, &mut |entrance, tile| {
        entrance.has_control_panel().then_some((entrance, tile))
    }) else {
        return Err("no control panel");
    };
    println!("found panel {:?}", panel);

    let mut gates = OpenedGates::NON_EMPTY;
    // try to open gates
    follow_chain::<()>(rooms, gates, 7, Direction::Down, &mut |e, _| {
        if let Some(gate) = e.open_gate() {
            gates |= gate;
        }
        None
    });

    let mut state_to_gate: HashMap<RoomAndPos, OpenedGates> = HashMap::new();

    let (room, dir) = panel.to_room_direction();
    let pos_room = RoomAndPos {
        pos_tile: panel_tile,
        pos_direction: dir,
        rooms: rooms.clone(),
    };

    // state_to_gate.insert(pos_room, gates);

    let mut counter = 0;
    let mut unreachable_entrances: HashSet<Entrance> = enum_iterator::all::<Entrance>().collect();
    let beatable = verify_rec(&mut state_to_gate, pos_room, gates, &mut counter, &mut unreachable_entrances);

    println!("{counter}");
    println!("beatable: {beatable}");

    Ok(())
}

fn verify_norec(
    state_to_gate: &mut HashMap<RoomAndPos, OpenedGates>,
)

fn verify_rec(
    state_to_gate: &mut HashMap<RoomAndPos, OpenedGates>,
    room_and_pos: RoomAndPos,
    mut gates: OpenedGates,
    counter: &mut usize,
    unreachable_entrances: &mut HashSet<Entrance>,
) -> bool {
    *counter += 1;
    // try to open gates
    follow_chain::<()>(&room_and_pos.rooms, gates, 7, Direction::Down, &mut |e, _| {
        if let Some(gate) = e.open_gate() {
            gates |= gate;
        }
        unreachable_entrances.remove(&e);
        None
    });
    if unreachable_entrances.is_empty() {
        return true;
    }
    let gates = match state_to_gate.entry(room_and_pos.clone()) {
        Entry::Occupied(current_gates) => {
            if current_gates.get().contains(gates) {
                return false;
            }
            current_gates.get().clone()
        },
        Entry::Vacant(entry) => {
            entry.insert(gates);
            gates
        }
    };
    let RoomAndPos { rooms, pos_tile, pos_direction } = room_and_pos;
    // print_rooms(rooms);
    for operation in enum_iterator::all::<Operations>() {
        match operation {
            Operations::ReachMoverStart => {
                if let Some(panel_tile) = follow_chain_both(&rooms, gates, pos_tile, pos_direction, &mut |e, panel_tile| {
                    (e == Entrance::StartDown).then_some(panel_tile)
                }) {
                    let new_room_pos = RoomAndPos { rooms, pos_tile: panel_tile, pos_direction: Direction::Down };
                    if verify_rec(state_to_gate, new_room_pos, gates, counter, unreachable_entrances) {
                        return true;
                    }
                }
            },
            Operations::ReachMoverLanayruMiningFacility => {
                if let Some(panel_tile) = follow_chain_both(&rooms, gates, pos_tile, pos_direction, &mut |e, panel_tile| {
                    (e == Entrance::LanayruMiningFacilityDown).then_some(panel_tile)
                }) {
                    let new_room_pos = RoomAndPos { rooms, pos_tile: panel_tile, pos_direction: Direction::Down };
                    if verify_rec(state_to_gate, new_room_pos, gates, counter, unreachable_entrances) {
                        return true;
                    }
                }
            },
            Operations::ReachMoverEarthTemple => {
                if let Some(panel_tile) = follow_chain_both(&rooms, gates, pos_tile, pos_direction, &mut |e, panel_tile| {
                    (e == Entrance::EarthTempleDown).then_some(panel_tile)
                }) {
                    let new_room_pos = RoomAndPos { rooms: rooms.clone(), pos_tile: panel_tile, pos_direction: Direction::Down };
                    if verify_rec(state_to_gate, new_room_pos, gates, counter, unreachable_entrances) {
                        return true;
                    }
                }
            },
            Operations::ReachMoverMiniBoss => {
                if let Some(panel_tile) = follow_chain_both(&rooms, gates, pos_tile, pos_direction, &mut |e, panel_tile| {
                    (e == Entrance::MiniBossLeft).then_some(panel_tile)
                }) {
                    let new_room_pos = RoomAndPos { rooms: rooms.clone(), pos_tile: panel_tile, pos_direction: Direction::Down };
                    if verify_rec(state_to_gate, new_room_pos, gates, counter, unreachable_entrances) {
                        return true;
                    }
                }
            },
            Operations::MoveUp => {
                // if we move up into the empty space, we swap with the tile that is down
                let empty_tile = rooms.iter().position(|r| r == &Room::Empty).unwrap() as u8;
                if let Some((other_tile, _)) = do_move(empty_tile, Direction::Down) {
                    if other_tile != pos_tile {
                        let mut rooms = rooms.clone();
                        rooms.swap(other_tile.into(), empty_tile.into());
                        if verify_rec(state_to_gate, RoomAndPos { rooms, pos_tile, pos_direction }, gates, counter, unreachable_entrances) {
                            return true;
                        }
                    }
                }
            },
            Operations::MoveLeft => {
                let empty_tile = rooms.iter().position(|r| r == &Room::Empty).unwrap() as u8;
                if let Some((other_tile, _)) = do_move(empty_tile, Direction::Right) {
                    if other_tile != pos_tile {
                        let mut rooms = rooms.clone();
                        rooms.swap(other_tile.into(), empty_tile.into());
                        if verify_rec(state_to_gate, RoomAndPos { rooms, pos_tile, pos_direction }, gates, counter, unreachable_entrances) {
                            return true;
                        }
                    }
                }
            },
            Operations::MoveDown => {
                let empty_tile = rooms.iter().position(|r| r == &Room::Empty).unwrap() as u8;
                if let Some((other_tile, _)) = do_move(empty_tile, Direction::Up) {
                    if other_tile != pos_tile {
                        let mut rooms = rooms.clone();
                        rooms.swap(other_tile.into(), empty_tile.into());
                        if verify_rec(state_to_gate, RoomAndPos { rooms, pos_tile, pos_direction }, gates, counter, unreachable_entrances) {
                            return true;
                        }
                    }
                }
            },
            Operations::MoveRight => {
                let empty_tile = rooms.iter().position(|r| r == &Room::Empty).unwrap() as u8;
                if let Some((other_tile, _)) = do_move(empty_tile, Direction::Left) {
                    if other_tile != pos_tile {
                        let mut rooms = rooms.clone();
                        rooms.swap(other_tile.into(), empty_tile.into());
                        if verify_rec(state_to_gate, RoomAndPos { rooms, pos_tile, pos_direction }, gates, counter, unreachable_entrances) {
                            return true;
                        }
                    }
                }
            },
        }
    }
    false
}

fn follow_chain_both<T>(rooms: &[Room; 9], gates: OpenedGates, mut tile: u8, mut direction: Direction, check: &mut impl FnMut(Entrance, u8) -> Option<T>) -> Option<T> {
    follow_chain(rooms, gates, tile, direction, check).or_else(|| {
        if let Some((tile, direction)) = do_move(tile, direction) {
            follow_chain(rooms, gates, tile, direction, check)
        } else {
            None
        }
    })
}

fn follow_chain<T>(rooms: &[Room; 9], gates: OpenedGates, mut tile: u8, mut direction: Direction, check: &mut impl FnMut(Entrance, u8) -> Option<T>) -> Option<T> {
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
