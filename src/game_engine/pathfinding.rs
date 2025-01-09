use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use super::entity_components::{CollisionBox, PositionComponent};
use super::world::World;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityDirectionOptions{
    Up,
    Down,
    Left,
    Right,
    None
}

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

pub fn pathfind_by_block(position_component: PositionComponent, collision_component: CollisionBox, entity_id: usize, world: &World) -> EntityDirectionOptions {
    let player = world.player.borrow();
    let ex = (position_component.x + collision_component.x_offset);
    let ey = (position_component.y + collision_component.y_offset);
    let ew = collision_component.w;
    let eh = collision_component.h;

    let (player_x, player_y) = ((player.x.floor() / 32.0).floor() as usize, (player.y.floor() / 32.0).floor() as usize);
    let (entity_x, entity_y) = ((ex / 32.0).floor() as usize, (ey / 32.0).floor() as usize);
    let entity_x_offset = ex - entity_x as f32 * 32.0;
    let entity_y_offset = ey - entity_y as f32 * 32.0;

    let distance = ((player_x as isize - entity_x as isize).abs() + (player_y as isize - entity_y as isize).abs()) as f32;

    let mut nodes: HashMap<[usize; 2], PathfindingNode> = HashMap::new();
    let mut open_set = BinaryHeap::new();
    let mut closed_set: HashMap<[usize; 2], bool> = HashMap::new();

    let start_node = PathfindingNode::new(entity_x, entity_y, None, player_x, player_y, 0);
    let start_node_clone = start_node.clone();
    open_set.push(start_node.clone());
    nodes.insert([entity_x, entity_y], start_node);

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];


    while let Some(current) = open_set.pop() {
        let distance_from_goal = f32::sqrt(((player_x as f32 - current.x as f32).powf(2.0) as f32 + (player_y as f32 - current.y as f32).powf(2.0)) as f32);

        if distance_from_goal < 2.0 {
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
            
            return match (path[0].0 as isize - start_node_clone.x as isize, path[0].1 as isize - start_node_clone.y as isize) {
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
            if world.check_collision(true, Some(entity_id), ((nx * 32) as f32 + entity_x_offset).floor() as usize, ((ny * 32) as f32 + entity_y_offset).floor() as usize, ew as usize, eh as usize, true) {
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
pub fn pathfind_high_granularity(position_component: PositionComponent, collision_component: CollisionBox, entity_id: usize, world: &World) -> EntityDirectionOptions {
    let player = world.player.borrow();

    let ex = (position_component.x + collision_component.x_offset);
    let ey = (position_component.y + collision_component.y_offset);
    let ew = collision_component.w;
    let eh = collision_component.h;


    let (player_x, player_y) = ((player.x.floor() / 4.0).floor() as usize, (player.y.floor() / 4.0).floor() as usize);
    let (entity_x, entity_y) = ((ex / 4.0).floor() as usize, (ey / 4.0).floor() as usize);
    let entity_x_offset = ex - entity_x as f32 * 4.0;
    let entity_y_offset = ey - entity_y as f32 * 4.0;


    let distance = ((player_x as isize - entity_x as isize).abs() + (player_y as isize - entity_y as isize).abs()) as f32;

    let mut nodes: HashMap<[usize; 2], PathfindingNode> = HashMap::new();
    let mut open_set = BinaryHeap::new();
    let mut closed_set: HashMap<[usize; 2], bool> = HashMap::new();

    let start_node = PathfindingNode::new(entity_x, entity_y, None, player_x, player_y, 0);
    let start_node_clone = start_node.clone();
    open_set.push(start_node.clone());
    nodes.insert([entity_x, entity_y], start_node);

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some(current) = open_set.pop() {
        let cur_distance_from_goal = ((player_x as isize - current.x as isize).abs() + (player_y as isize - current.y as isize).abs()) as f32;
        if cur_distance_from_goal < 12.0 {
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
           
            return match (path[0].0 as isize - start_node_clone.x as isize, path[0].1 as isize - start_node_clone.y as isize) {
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

            if nx > 100000000 || ny > 100000000 {
                continue;
            }


            let distance_from_goal = ((player_x as isize - nx as isize).abs() + (player_y as isize - ny as isize).abs()) as f32;
            
            if distance_from_goal > distance + 10.0 {
                continue;
            }

            if closed_set.contains_key(&[nx, ny]) {
                continue;
            }

            if world.check_collision(false, Some(entity_id), (nx as f32 * 4.0 + entity_x_offset).floor() as usize, (ny as f32 * 4.0 + entity_y_offset).floor() as usize, ew as usize, eh as usize, true) {
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
