use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::env;
use std::fs;
use std::io::{self, BufRead};


const MAX_STEPS: usize = 100_000_000;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North, 
    South,
    West,
    East
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    location: (usize, usize),
    cost: usize,
    straight: usize,
    direction: Option<Direction>,
    route: Vec<(usize, usize)>
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.location.cmp(&other.location))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn shortest_path_cost(terrain: &Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: BTreeSet<(usize, usize, Option<Direction>, usize)> = BTreeSet::new();
    // as in everything in the state other than the cost
    heap.push(State { location: start, cost: 0, straight: 0, direction: None, route: vec![] });
    let mut steps: usize = 0;

    while let Some(State { location, cost, straight, direction, route }) = heap.pop() {
        if location == end && straight > 3 {
            for (y, row) in terrain.iter().enumerate() {
                for (x, value) in row.iter().enumerate() {
                    if route.contains(&(x, y)) { print!("*"); } else { print!("{}", value); }
                }
                println!();
            }
            return Some(cost); 
        }
        if visited.contains(&(location.0, location.1, direction, straight)) { continue; } 
        visited.insert((location.0, location.1, direction, straight));

        let new_directions: Vec<(Direction, usize)> = match direction {
            Some(Direction::North) => vec![
                (Direction::North, straight + 1),
                (Direction::West, 1),
                (Direction::East, 1)
            ],
            Some(Direction::South) => vec![
                (Direction::South, straight + 1),
                (Direction::West, 1),
                (Direction::East, 1)
            ],
            Some(Direction::West) => vec![
                (Direction::West, straight + 1),
                (Direction::North, 1),
                (Direction::South, 1)
            ],
            Some(Direction::East) => vec![
                (Direction::East, straight + 1),
                (Direction::North, 1),
                (Direction::South, 1)
            ],
            None => vec![
                (Direction::North, 1),
                (Direction::South, 1),
                (Direction::West, 1),
                (Direction::East, 1),
            ]
        };

        for (d, s) in new_directions {
            if let Some(new_location) = match d {
                Direction::North => if location.1 > 0 { Some((location.0, location.1 - 1)) } else { None },
                Direction::South => if location.1 < terrain.len() - 1 { Some((location.0, location.1 + 1)) } else { None },
                Direction::West  => if location.0 > 0 { Some((location.0 - 1, location.1)) } else { None },
                Direction::East  => if location.0 < terrain[0].len() - 1 { Some((location.0 + 1, location.1)) } else { None },
            } {
                if !visited.contains(&(new_location.0, new_location.1, Some(d), s)) && s < 11 && (direction == None || Some(d) == direction || straight > 3) {
                    heap.push(State { 
                        location: new_location, 
                        cost: cost + terrain[new_location.1][new_location.0], 
                        straight: s, 
                        direction: Some(d),
                        route: [route.clone(), vec![location]].concat()
                    });
                }
            }
        }
        steps += 1;
        if steps > MAX_STEPS { panic!("Max steps reached!") };

    }

    None
}


fn main() {
    let path = env::args().nth(1).expect("Required parameter path missing!");

    let data: Vec<Vec<usize>> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .map(|l| {
            let text = l.expect("Could not read line!");
            text.split("").into_iter().filter_map(|c| c.parse::<usize>().ok()).collect()
        })
        .collect();

    let min_cost = shortest_path_cost(&data, (0, 0), (data.len() - 1, data[0].len() - 1));
    println!("Min cost to exit: {}", min_cost.unwrap());

}


#[cfg(test)]
mod tests {
    use crate::shortest_path_cost;

    #[test]
    fn test_shortest_path() {
        let terrain: Vec<Vec<usize>> = vec![
            vec![1,1,1,1,1,1,1,1,1,1,1,1],
            vec![9,9,9,9,9,9,9,9,9,9,9,1],
            vec![9,9,9,9,9,9,9,9,9,9,9,1],
            vec![9,9,9,9,9,9,9,9,9,9,9,1],
            vec![9,9,9,9,9,9,9,9,9,9,9,1],
        ];
        let cost = shortest_path_cost(&terrain, (0, 0), (terrain[0].len() - 1, terrain.len() - 1));
        assert_eq!(cost, Some(71));

        let terrain: Vec<Vec<usize>> = vec![
            vec![2,4,1,3,4,3,2,3,1,1,3,2,3],
            vec![3,2,1,5,4,5,3,5,3,5,6,2,3],
            vec![3,2,5,5,2,4,5,6,5,4,2,5,4],
            vec![3,4,4,6,5,8,5,8,4,5,4,5,2],
            vec![4,5,4,6,6,5,7,8,6,7,5,3,6],
            vec![1,4,3,8,5,9,8,7,9,8,4,5,4],
            vec![4,4,5,7,8,7,6,9,8,7,7,6,6],
            vec![3,6,3,7,8,7,7,9,7,9,6,5,3],
            vec![4,6,5,4,9,6,7,9,8,6,8,8,7],
            vec![4,5,6,4,6,7,9,9,8,6,4,5,3],
            vec![1,2,2,4,6,8,6,8,6,5,5,6,3],
            vec![2,5,4,6,5,4,8,8,8,7,7,3,5],
            vec![4,3,2,2,6,7,4,6,5,5,5,3,3],
        ];
        let cost = shortest_path_cost(&terrain, (0, 0), (terrain[0].len() - 1, terrain.len() - 1));
        assert_eq!(cost, Some(94));

        let terrain: Vec<Vec<usize>> = vec![
            vec![1, 6, 2, 3],
            vec![1, 1, 1, 7],
            vec![1, 9, 1, 1],
            vec![2, 5, 2, 1],
        ];
        let cost = shortest_path_cost(&terrain, (0, 0), (3, 3));
        assert_eq!(cost, None);
    }

}
