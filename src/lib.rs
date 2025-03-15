use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;

#[wasm_bindgen]
pub fn find_path(start_x: i32, start_y: i32, goal_x: i32, goal_y: i32) -> JsValue {
    let path = find_path_internal(start_x, start_y, goal_x, goal_y);
    to_value(&path).unwrap()
}

pub fn find_path_internal(start_x: i32, start_y: i32, goal_x: i32, goal_y: i32) -> Vec<(i32, i32)> {
    let start = Node { x: start_x, y: start_y, cost: 0, heuristic: 0 };
    let mut open_set = BinaryHeap::new();
    open_set.push(start);
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    g_score.insert((start_x, start_y), 0);
    while let Some(current) = open_set.pop() {
        if current.x == goal_x && current.y == goal_y {
            return reconstruct_path(came_from, current);
        }
        for neighbor in get_neighbors(&current) {
            let tentative_g_score = g_score.get(&(current.x, current.y)).unwrap_or(&i32::MAX) + 1;
            if tentative_g_score < *g_score.get(&(neighbor.x, neighbor.y)).unwrap_or(&i32::MAX) {
                came_from.insert((neighbor.x, neighbor.y), (current.x, current.y));
                g_score.insert((neighbor.x, neighbor.y), tentative_g_score);
                let h = heuristic(neighbor.x, neighbor.y, goal_x, goal_y);
                open_set.push(Node { x: neighbor.x, y: neighbor.y, cost: tentative_g_score, heuristic: h });
            }
        }
    }
    Vec::new()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    x: i32,
    y: i32,
    cost: i32,
    heuristic: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors(node: &Node) -> Vec<Node> {
    vec![
        Node { x: node.x + 1, y: node.y, cost: node.cost + 1, heuristic: 0 },
        Node { x: node.x - 1, y: node.y, cost: node.cost + 1, heuristic: 0 },
        Node { x: node.x, y: node.y + 1, cost: node.cost + 1, heuristic: 0 },
        Node { x: node.x, y: node.y - 1, cost: node.cost + 1, heuristic: 0 },
    ]
}

fn heuristic(x: i32, y: i32, goal_x: i32, goal_y: i32) -> i32 {
    (goal_x - x).abs() + (goal_y - y).abs()
}

fn reconstruct_path(mut came_from: HashMap<(i32, i32), (i32, i32)>, current: Node) -> Vec<(i32, i32)> {
    let mut path = vec![(current.x, current.y)];
    let mut current = (current.x, current.y);
    while let Some(prev) = came_from.remove(&current) {
        path.push(prev);
        current = prev;
    }
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_equals_goal() {
        let path = find_path_internal(0, 0, 0, 0);
        assert_eq!(path, vec![(0, 0)]);
    }

    #[test]
    fn test_simple_path() {
        let path = find_path_internal(0, 0, 1, 0);
        assert_eq!(path, vec![(0, 0), (1, 0)]);
    }

    #[test]
    fn test_diagonal_path() {
        let path = find_path_internal(0, 0, 2, 2);
        assert_eq!(path.first(), Some(&(0, 0)));
        assert_eq!(path.last(), Some(&(2, 2)));
        assert_eq!(path.len(), 5);
        for pair in path.windows(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            assert_eq!((x1 - x2).abs() + (y1 - y2).abs(), 1);
        }
    }

    #[test]
    fn test_negative_coordinates() {
        let path = find_path_internal(-1, -1, 1, 1);
        assert_eq!(path.first(), Some(&(-1, -1)));
        assert_eq!(path.last(), Some(&(1, 1)));
        assert_eq!(path.len(), 5);
    }
}
