use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct Room {
    index: usize,
    size: usize,
    contents: [char; 4],
}

impl Room {
    fn new(index: usize, size: usize) -> Self {
        Self {
            index,
            size,
            contents: ['.'; 4],
        }
    }

    fn amphipod(&self) -> char {
        match self.index {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            _ => unreachable!(),
        }
    }

    fn spot_for(&self, amphipod: char) -> Option<usize> {
        if amphipod != self.amphipod() {
            return None;
        }

        let mut first = 0;
        while first < self.size && self.contents[first] == '.' {
            first += 1;
        }

        if first == 0
            || self.contents[first..self.size]
                .iter()
                .any(|&c| c != amphipod)
        {
            None
        } else {
            Some(first - 1)
        }
    }

    fn first_amphipod(&self) -> Option<usize> {
        let mut first = 0;
        while first < self.size && self.contents[first] == '.' {
            first += 1;
        }

        if first == self.size
            || self.contents[first..self.size]
                .iter()
                .all(|&c| c == self.amphipod())
        {
            None
        } else {
            Some(first)
        }
    }

    fn ok(&self) -> bool {
        let amphipod = self.amphipod();
        self.contents[..self.size].iter().all(|&c| c == amphipod)
    }

    fn corridor_position(&self) -> usize {
        2 * (self.index + 1)
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
struct State {
    corridor: [char; 11],
    rooms: [Room; 4],
}

fn energy_required(steps: usize, amphipod: char) -> usize {
    steps
        * match amphipod {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        }
}

fn amphipod_dest_room(amphipod: char) -> usize {
    match amphipod {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => unreachable!(),
    }
}

fn path_clear(a: usize, b: usize, corridor: [char; 11]) -> bool {
    let (from, to) = if a < b { (a, b) } else { (b, a) };
    corridor[from..=to].iter().all(|&c| c == '.')
}

fn min_energy(rooms: [&str; 4]) -> usize {
    let mut q = BinaryHeap::new();

    let room_size = rooms[0].len();
    let mut initial_state = State {
        corridor: ['.'; 11],
        rooms: [
            Room::new(0, room_size),
            Room::new(1, room_size),
            Room::new(2, room_size),
            Room::new(3, room_size),
        ],
    };

    for (room, s) in initial_state.rooms.iter_mut().zip(rooms) {
        for (room_space, amphipod) in room.contents.iter_mut().zip(s.chars()) {
            *room_space = amphipod;
        }
    }

    q.push(Reverse((0, initial_state.clone())));

    let mut seen = HashMap::new();
    seen.insert(initial_state, 0);

    while let Some(Reverse((energy, state))) = q.pop() {
        let corridor = state.corridor;
        let rooms = state.rooms;

        if rooms.iter().all(|room| room.ok()) {
            return energy;
        }

        for room in rooms {
            if let Some(rp) = room.first_amphipod() {
                for cp in [0, 1, 3, 5, 7, 9, 10] {
                    let rcp = room.corridor_position();
                    if path_clear(rcp, cp, corridor) {
                        let steps = cp.abs_diff(rcp) + rp + 1;
                        let mut corridor = corridor.clone();
                        corridor[cp] = room.contents[rp];
                        let mut rooms = rooms.clone();
                        rooms[room.index].contents[rp] = '.';
                        let next_state = State { corridor, rooms };
                        let next_energy = energy + energy_required(steps, corridor[cp]);

                        if next_energy < *seen.get(&next_state).unwrap_or(&usize::MAX) {
                            q.push(Reverse((next_energy, next_state.clone())));
                            seen.insert(next_state, next_energy);
                        }
                    }
                }
            }
        }

        for (cp, &c) in corridor.iter().enumerate().filter(|&(_, &c)| c != '.') {
            let dest_room = rooms[amphipod_dest_room(c)];
            let rcp = dest_room.corridor_position();

            if let Some(rp) = dest_room.spot_for(c) {
                let x = if cp < rcp { cp + 1 } else { cp - 1 };
                if path_clear(rcp, x, corridor) {
                    let steps = cp.abs_diff(rcp) + rp + 1;
                    let mut rooms = rooms.clone();
                    rooms[dest_room.index].contents[rp] = c;
                    let mut corridor = corridor.clone();
                    corridor[cp] = '.';
                    let next_state = State { corridor, rooms };
                    let next_energy =
                        energy + energy_required(steps, rooms[dest_room.index].contents[rp]);

                    if next_energy < *seen.get(&next_state).unwrap_or(&usize::MAX) {
                        q.push(Reverse((next_energy, next_state.clone())));
                        seen.insert(next_state, next_energy);
                    }
                }
            }
        }
    }

    unreachable!()
}

fn main() {
    println!("part 1: {}", min_energy(["CD", "AC", "BA", "DB"]));
    println!("part 2: {}", min_energy(["CDDD", "ACBC", "BBAA", "DACB"]));
}
