use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait Metric {
    fn distance(&self, other: &Self) -> usize;

    /// This method provides desired radius of space partition.
    /// It allows to provide a suitable tree balance
    /// in case you have some knowledge of data distribution
    /// Use with caution cause wrong radius can cause tree degradation.
    fn partition_radius(&self) -> Option<usize> {
        None
    }
}

#[derive(Debug)]
pub struct Node<Point>
where
    Point: Metric + Copy,
{
    pub p: Point,
    pub radius: usize,
    pub near: Option<Box<Node<Point>>>,
    pub far: Option<Box<Node<Point>>>,
}

impl<Point> Node<Point>
where
    Point: Metric + Copy,
{
    pub fn insert_point(&mut self, node: Box<Node<Point>>) {
        let distance = node.p.distance(&self.p);
        if distance < self.radius {
            if let Some(leaf) = &mut self.near {
                leaf.insert_point(node)
            } else {
                self.near = Some(node);
            }
        } else {
            if let Some(leaf) = &mut self.far {
                leaf.insert_point(node)
            } else {
                self.far = Some(node);
            }
        }
    }
}

impl<Point> From<Vec<Point>> for Node<Point>
where
    Point: Metric + Copy,
{
    fn from(points: Vec<Point>) -> Self {
        let root_p = points[0];
        let mut distances: Vec<(usize, Point)> = Vec::new();
        for p in points.iter().skip(1) {
            distances.push((root_p.distance(p), *p));
        }
        let radius = match root_p.partition_radius() {
            Some(r) => r,
            None => {
                if distances.len() > 0 {
                    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    distances[distances.len() / 2].0
                } else {
                    0
                }
            }
        };
        let mut near_points: Vec<Point> = Vec::new();
        let mut far_points: Vec<Point> = Vec::new();
        for (dist, p) in distances {
            if dist < radius {
                near_points.push(p);
            } else {
                far_points.push(p);
            }
        }
        let near = if near_points.len() > 0 {
            Some(Box::new(near_points.into()))
        } else {
            None
        };
        let far = if far_points.len() > 0 {
            Some(Box::new(far_points.into()))
        } else {
            None
        };

        Node {
            p: root_p,
            radius,
            near,
            far,
        }
    }
}

struct Closest<T>
{
    distance: usize,
    data: T,
}

impl<T> PartialEq for Closest<T>
{
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<T> Eq for Closest<T> {}

impl<T> Ord for Closest<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(Ordering::Equal)
    }
}

impl<T> PartialOrd for Closest<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

#[derive(Debug)]
pub struct VPTree<Point>
where
    Point: Metric + Copy,
{
    size: usize,
    pub root: Node<Point>,
}

impl<Point> VPTree<Point>
where
    Point: Metric + Copy + Hash + Eq,
{
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, p: Point) {
        let root = &mut self.root;
        root.insert_point(Box::new(Node {
            p,
            radius: p.partition_radius().unwrap_or(0),
            near: None,
            far: None,
        }));
        self.size += 1;
    }


    pub fn height(&self) -> usize {
        self.node_height(&self.root, 0)
    }

    pub fn search_closest(&self, sample: Point, count: usize) -> HashSet<Point> {
        let mut closest: BinaryHeap<Closest<Point>> = BinaryHeap::with_capacity(count + 1);
        let mut tau = usize::MAX; // Farthest point, found so far

        self.search(&self.root, sample, count, &mut closest, &mut tau);

        closest.iter().map(|c| c.data).collect()
    }

    fn search(
        &self,
        node: &Node<Point>,
        sample: Point,
        count: usize,
        closest: &mut BinaryHeap<Closest<Point>>,
        tau: &mut usize,
    ) {
        let distance = sample.distance(&node.p);
        if distance < *tau {
            closest.push(Closest {
                distance,
                data: node.p,
            });
            if closest.len() > count {
                closest.pop();
                *tau = closest.peek().unwrap().distance;
            }
        }

        if distance < node.radius {
            if distance - *tau <= node.radius {
                if let Some(leaf) = &node.near {
                    self.search(leaf, sample, count, closest, tau);
                }
            }
            if distance + *tau >= node.radius {
                if let Some(leaf) = &node.far {
                    self.search(leaf, sample, count, closest, tau);
                }
            }
        } else {
            if distance + *tau >= node.radius {
                if let Some(leaf) = &node.far {
                    self.search(leaf, sample, count, closest, tau);
                }
            }
            if distance - *tau <= node.radius {
                if let Some(leaf) = &node.near {
                    self.search(leaf, sample, count, closest, tau);
                }
            }
        }
    }

    pub fn search_in_radius(&self, sample: Point, radius: usize) -> HashSet<Point> {
        let mut results = HashSet::new();
        let mut nodes_to_search = Vec::new();

        nodes_to_search.push(&self.root);

        while let Some(node) = nodes_to_search.pop() {
            let distance = sample.distance(&node.p);
            if distance <= radius {
                results.insert(node.p);
            }
            if distance - radius <= node.radius {
                if let Some(leaf) = &node.near {
                    nodes_to_search.push(leaf);
                }
            }
            if distance + radius > node.radius {
                if let Some(leaf) = &node.far {
                    nodes_to_search.push(leaf);
                }
            }
        }

        results
    }

    fn node_height(&self, node: &Node<Point>, height: usize) -> usize {
        let mut results = vec![height];
        if let Some(leaf) = &node.near {
            results.push(self.node_height(leaf, height + 1));
        }
        if let Some(leaf) = &node.far {
            results.push(self.node_height(leaf, height + 1));
        }
        *results.iter().max().unwrap()
    }
}

impl<Point> From<Vec<Point>> for VPTree<Point>
where
    Point: Metric + Copy + Hash + Eq,
{
    fn from(points: Vec<Point>) -> Self {
        VPTree {
            size: points.len(),
            root: Node::from(points),
        }
    }
}

