use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::time::Instant;

use super::entities::Entity;
use super::world::{EntityDirectionOptions, World};

#[derive(Debug, Clone, PartialEq)]
struct PathfindingNode {
    x: usize,
    y: usize,
    parent: Option<[usize; 2]>,
    depth: usize,
    f: f32,
}

impl PathfindingNode {
    pub fn new(x: usize, y: usize, parent: Option<[usize; 2]>, goal_x: usize, goal_y: usize, depth: usize) -> Self {
        let h = ((goal_x as isize - x as isize).abs() + (goal_y as isize - y as isize).abs()) as f32;
        let f = depth as f32 + h * 1.0001;
        Self { x, y, parent, depth, f }
    }
}

impl Eq for PathfindingNode {}

impl PartialOrd for PathfindingNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Ord for PathfindingNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(PartialEq, Clone)]
enum OpenClosed {
    Open,
    Closed,
}

pub fn pathfind_by_block(entity_id: usize, world: &World, entity: &Entity, entitiesref: HashMap<usize, Entity>) -> EntityDirectionOptions {
    let start_time = Instant::now();

    let player = world.player.borrow();
    let (player_x, player_y) = ((player.x / 32.0).floor() as usize, (player.y / 32.0).floor() as usize);
    let (entity_x, entity_y) = ((entity.x / 32.0).floor() as usize, (entity.y / 32.0).floor() as usize);

    let distance = ((player_x as isize - entity_x as isize).abs() + (player_y as isize - entity_y as isize).abs()) as f32;

    let mut nodes: HashMap<[usize; 2], PathfindingNode> = HashMap::new();
    let mut open_set = BinaryHeap::new();
    let mut closed_set: HashMap<[usize; 2], bool> = HashMap::new();

    let start_node = PathfindingNode::new(entity_x, entity_y, None, player_x, player_y, 0);
    open_set.push(start_node.clone());
    nodes.insert([entity_x, entity_y], start_node);

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];


    while let Some(current) = open_set.pop() {
        if current.x == player_x && current.y == player_y {
            let mut path = Vec::new();
            let mut node = current;

            while let Some(parent) = node.parent {
                path.push((node.x, node.y));
                node = nodes[&parent].clone();
            }

            path.reverse();
            if path.len() < 2 {
                return EntityDirectionOptions::None;
            }
            println!("Pathfinding took: {:?}", start_time.elapsed().as_nanos());
            return match (path[1].0 as isize - path[0].0 as isize, path[1].1 as isize - path[0].1 as isize) {
                (1, 0) => EntityDirectionOptions::Right,
                (-1, 0) => EntityDirectionOptions::Left,
                (0, 1) => EntityDirectionOptions::Down,
                (0, -1) => EntityDirectionOptions::Up,
                _ => EntityDirectionOptions::None,
            };
        }

        closed_set.insert([current.x, current.y], true);

        for &(dx, dy) in &directions {
            let nx = current.x.wrapping_add_signed(dx);
            let ny = current.y.wrapping_add_signed(dy);

            let distance_from_goal = ((player_x as isize - nx as isize).abs() + (player_y as isize - ny as isize).abs()) as f32;
            if distance_from_goal > distance + 10.0 {
                continue;
            }

            if closed_set.contains_key(&[nx, ny]) {
                continue;
            }
            if nx > 100000000 || ny > 100000000 {
                continue;
            }
            if world.check_collision(true, Some(entity_id), nx * 32, ny * 32, 32, 32, true, Some(entitiesref.clone())) {
                continue;
            }

            let neighbor = PathfindingNode::new(nx, ny, Some([current.x, current.y]), player_x, player_y, current.depth + 1);

            if let Some(existing) = nodes.get(&[nx, ny]) {
                if neighbor.f < existing.f {
                    nodes.insert([nx, ny], neighbor.clone());
                    open_set.push(neighbor);
                }
            } else {
                nodes.insert([nx, ny], neighbor.clone());
                open_set.push(neighbor);
            }
        }
    }

    EntityDirectionOptions::None
}

pub fn pathfind_high_granularity(entity_id: usize, world: &World, entity: &Entity, entitiesref: HashMap<usize, Entity>) -> EntityDirectionOptions {
    let start_time = Instant::now();

    let player = world.player.borrow();
    let (player_x, player_y) = ((player.x / 4.0).floor() as usize, (player.y / 4.0).floor() as usize);
    let (entity_x, entity_y) = ((entity.x / 4.0).floor() as usize, (entity.y / 4.0).floor() as usize);

    let distance = ((player_x as isize - entity_x as isize).abs() + (player_y as isize - entity_y as isize).abs()) as f32;

    let mut nodes: HashMap<[usize; 2], PathfindingNode> = HashMap::new();
    let mut open_set = BinaryHeap::new();
    let mut closed_set: HashMap<[usize; 2], bool> = HashMap::new();

    let start_node = PathfindingNode::new(entity_x, entity_y, None, player_x, player_y, 0);
    open_set.push(start_node.clone());
    nodes.insert([entity_x, entity_y], start_node);

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some(current) = open_set.pop() {
        if current.x == player_x && current.y == player_y {
            let mut path = Vec::new();
            let mut node = current;

            while let Some(parent) = node.parent {
                path.push((node.x, node.y));
                node = nodes[&parent].clone();
            }

            path.reverse();
            if path.len() < 2 {
                return EntityDirectionOptions::None;
            }
            println!("Pathfinding took: {:?}", start_time.elapsed().as_nanos());
            return match (path[1].0 as isize - path[0].0 as isize, path[1].1 as isize - path[0].1 as isize) {
                (1, 0) => EntityDirectionOptions::Right,
                (-1, 0) => EntityDirectionOptions::Left,
                (0, 1) => EntityDirectionOptions::Down,
                (0, -1) => EntityDirectionOptions::Up,
                _ => EntityDirectionOptions::None,
            };
        }

        closed_set.insert([current.x, current.y], true);

        for &(dx, dy) in &directions {
            let nx = current.x.wrapping_add_signed(dx);
            let ny = current.y.wrapping_add_signed(dy);

            let distance_from_goal = ((player_x as isize - nx as isize).abs() + (player_y as isize - ny as isize).abs()) as f32;
            if distance_from_goal > distance + 10.0 {
                continue;
            }

            if closed_set.contains_key(&[nx, ny]) {
                continue;
            }

            if world.check_collision(true, Some(entity_id), nx * 4, ny * 4, 32, 32, true, Some(entitiesref.clone())) {
                continue;
            }

            let neighbor = PathfindingNode::new(nx, ny, Some([current.x, current.y]), player_x, player_y, current.depth + 1);

            if let Some(existing) = nodes.get(&[nx, ny]) {
                if neighbor.f < existing.f {
                    nodes.insert([nx, ny], neighbor.clone());
                    open_set.push(neighbor);
                }
            } else {
                nodes.insert([nx, ny], neighbor.clone());
                open_set.push(neighbor);
            }
        }
    }

    EntityDirectionOptions::None
}
