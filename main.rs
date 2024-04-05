use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Side {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    missionaries_left: u8,
    cannibals_left: u8,
    missionaries_right: u8,
    cannibals_right: u8,
    boat_side: Side,
}

impl State {
    fn is_valid(&self) -> bool {
        (self.missionaries_left == 0 || self.missionaries_left >= self.cannibals_left)
            && (self.missionaries_right == 0 || self.missionaries_right >= self.cannibals_right)
    }

    fn is_goal(&self) -> bool {
        self.missionaries_left == 0 && self.cannibals_left == 0
    }
}

fn get_next_states(current_state: &State) -> Vec<State> {
    let mut next_states = Vec::new();

    match current_state.boat_side {
        Side::Left => {
            for m in 0..=2 {
                for c in 0..=2 {
                    if m + c >= 1 && m + c <= 2 {
                        if m <= current_state.missionaries_left && c <= current_state.cannibals_left {
                            let new_state = State {
                                missionaries_left: current_state.missionaries_left - m,
                                cannibals_left: current_state.cannibals_left - c,
                                missionaries_right: current_state.missionaries_right + m,
                                cannibals_right: current_state.cannibals_right + c,
                                boat_side: Side::Right,
                            };
                            if new_state.is_valid() {
                                next_states.push(new_state);
                            }
                        }
                    }
                }
            }
        }
        Side::Right => {
            for m in 0..=2 {
                for c in 0..=2 {
                    if m + c >= 1 && m + c <= 2 {
                        if m <= current_state.missionaries_right && c <= current_state.cannibals_right {
                            let new_state = State {
                                missionaries_left: current_state.missionaries_left + m,
                                cannibals_left: current_state.cannibals_left + c,
                                missionaries_right: current_state.missionaries_right - m,
                                cannibals_right: current_state.cannibals_right - c,
                                boat_side: Side::Left,
                            };
                            if new_state.is_valid() {
                                next_states.push(new_state);
                            }
                        }
                    }
                }
            }
        }
    }

    next_states
}

fn breadth_first_search() -> Option<Vec<State>> {
    let initial_state = State {
        missionaries_left: 3,
        cannibals_left: 3,
        missionaries_right: 0,
        cannibals_right: 0,
        boat_side: Side::Left,
    };

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((vec![initial_state.clone()], initial_state));

    while let Some((path, state)) = queue.pop_front() {
        if state.is_goal() {
            return Some(path);
        }

        if !visited.contains(&state) {
            visited.insert(state.clone());
            let next_states = get_next_states(&state);
            for next_state in next_states {
                let mut new_path = path.clone();
                new_path.push(next_state.clone());
                queue.push_back((new_path, next_state));
            }
        }
    }

    None
}

fn main() {
    if let Some(solution) = breadth_first_search() {
        println!("Solution found:");
        for (index, state) in solution.iter().enumerate() {
            println!("Step {}: {:?}", index + 1, state);
        }
    } else {
        println!("No solution found.");
    }
}
