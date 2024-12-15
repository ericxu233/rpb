use num_traits::Float;
use rayon::prelude::*;
use std::sync::{Arc, Weak, Mutex};
use crate::common::geometry::Point2d;

impl Point2d<f64> {
    // Finds the min x and y values of two points
    fn minv(&self, other: &Self) -> Self {
        Point2d {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    // Finds the max x and y values of two points
    fn maxv(&self, other: &Self) -> Self {
        Point2d {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    // Finds the center x and y values of two points
    fn centerv(&self, other: &Self) -> Self {
        Point2d {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    fn get_max_dim(&self, other: &Self) -> f64 {
        (self.x - other.x).abs().max((self.y - other.y).abs())
    }

    // Find the center value and dimension, but it picks x/y that has larger difference
    fn get_split_point(&self, other: &Self) -> (f64, usize) {
        let diff_x = (self.x - other.x).abs();
        let diff_y = (self.y - other.y).abs();
        if diff_x >= diff_y {
            ((self.x + other.x) / 2.0, 0) // 0 for x-axis
        } else {
            ((self.y + other.y) / 2.0, 1) // 1 for y-axis
        }
    }

    fn get_dimension(&self, d: usize) -> f64 {
        match d {
            0 => self.x,
            1 => self.y,
            _ => panic!("Invalid dimension for 2D point"),
        }
    }

    fn get_distance(&self, other: &Self) -> f64 { // same as get_diameter from pbbs
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}


pub struct Node {
    pub n: usize,
    pub diameter: f64,
    pub max_dim: f64,
    pub id: usize,
    pub parent: Mutex<Weak<Node>>,
    pub left: Option<Arc<Node>>, 
    pub right: Option<Arc<Node>>, 
    pub vertex: Option<(Point2d<f64>, usize)>,
    pub center: Point2d<f64>,
    pub b: (Point2d<f64>, Point2d<f64>),
    pub interactions: Mutex<Vec<Weak<Node>>>,
}



impl Node {
    // Create a new leaf node
    pub fn new_leaf(p: &[(Point2d<f64>, usize)], idty: usize) -> Arc<Self> {
        let point = &p[0].0; // Access the single Point2d<f64> directly
        Arc::new(Node {
            n: 1,
            diameter: 0.0, // No diameter for a single point
            max_dim: 0.0, // No dimension difference for a single point
            id: idty,
            parent: Mutex::new(Weak::new()),
            left: None,
            right: None,
            vertex: Some(p[0].clone()), 
            center: point.clone(), // Center is the point itself
            b: (point.clone(), point.clone()), // Bounding box is the single point
            interactions: Mutex::new(Vec::new()),
        })
    }
    

    // Create a new internal node
    pub fn new_internal(l: Arc<Node>, r: Arc<Node>, idty: usize) -> Arc<Node> {
        let n = l.n + r.n;
        let bbox = (l.b.0.minv(&r.b.0), l.b.1.maxv(&r.b.1));
        let new_node = Arc::new(Node {
            n,
            diameter: bbox.0.get_distance(&bbox.1),
            max_dim: bbox.0.get_max_dim(&bbox.1),
            id: idty,
            parent: Mutex::new(Weak::new()),
            left: Some(l.clone()),
            right: Some(r.clone()),
            vertex: None,
            center: bbox.0.centerv(&bbox.1),
            b: bbox,
            interactions: Mutex::new(Vec::new()),
        });

        // Set the parent for left and right child nodes
        l.set_parent(&new_node);
        r.set_parent(&new_node);

        new_node
    }

    pub fn set_parent(&self, parent: &Arc<Node>) {
        *self.parent.lock().unwrap() = Arc::downgrade(parent);
    }

    pub fn add_interaction(&self, other: &Arc<Node>) {
        self.interactions.lock().unwrap().push(Arc::downgrade(other));
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none()
    }
}

pub fn get_box(v: &[&Point2d<f64>]) -> (Point2d<f64>, Point2d<f64>) {
    let n = v.len();

    if n == 0 {
        panic!("Input vector cannot be empty");
    }

    let (min_point, max_point) = v
        .par_iter()
        .fold(
            || (v[0].clone(), v[0].clone()),
            |acc, curr_pnt| {
                (acc.0.minv(*curr_pnt), acc.1.maxv(*curr_pnt))
            },
        )
        .reduce(
            || (v[0].clone(), v[0].clone()),
            |(min1, max1), (min2, max2)| {
                (min1.minv(&min2), max1.maxv(&max2))
            },
        );

    (min_point, max_point)
}

pub fn well_separated (a: &Arc<Node>, b: &Arc<Node>, s: f64) -> bool {
    // Diameter of the smallest sphere that can capture each box
    let diameter = Float::max(a.diameter, b.diameter);

    // Distance between the centers of the two boxes
    let d = a.center.get_distance(&b.center);

    // Check if the distance between the two balls is larger than 0.5 * s * diameter
    d - diameter >= 0.5 * s * diameter
}


pub fn wsr_children(l: &Arc<Node>, r: &Arc<Node>, s: f64, k: usize) {
    if well_separated(l, r, s) {
        if l.n <= k {
            l.add_interaction(r);
        }
        if r.n <= k {
            r.add_interaction(l);
        }
    } else {
        if let (Some(l_left), Some(l_right), Some(r_left), Some(r_right)) =
            (&l.left, &l.right, &r.left, &r.right)
        {
            if l.max_dim > r.max_dim {
                rayon::join(
                    || wsr_children(r, l_left, s, k),
                    || wsr_children(r, l_right, s, k),
                );
            } else {
                rayon::join(
                    || wsr_children(l, r_left, s, k),
                    || wsr_children(l, r_right, s, k),
                );
            }
        }
    }
}


pub fn wsr(t: &Arc<Node>, s: f64, k: usize) {
    if t.is_leaf() {
        return;
    }

    if let (Some(ref left), Some(ref right)) = (&t.left, &t.right) {
        // Process well-separated children
        wsr_children(left, right, s, k);

        // Recursively call `wsr` on children
        if t.n > 1000 {
            rayon::join(
                || wsr(left, s, k),
                || wsr(right, s, k),
            );
        } else {
            wsr(left, s, k);
            wsr(right, s, k);
        }
    }
}


pub fn build_recursive(
    points: &[(Point2d<f64>, usize)],
    id_offset: usize,
) -> Arc<Node> {
    if points.is_empty() {
        panic!("Passed in slice of size 0 when building tree.");
    }
    if points.len() == 1 {
        return Node::new_leaf(points, id_offset);
    }

    // Compute the bounding box directly
    let bbox = points.iter().fold(
        (points[0].0.clone(), points[0].0.clone()),
        |(min, max), (pt, _)| (min.minv(pt), max.maxv(pt)),
    );

    // Splitting depends on dimension that gives larger difference between the bounding boxes
    let (split_point, d) = bbox.0.get_split_point(&bbox.1);

    // Parallel computation of the split flags
    let flags_left: Vec<bool> = points
        .par_iter()
        .map(|(p, _)| p.get_dimension(d) < split_point)
        .collect();

    // Count the number of points in the left subtree
    let split_index = flags_left.iter().filter(|&&x| x).count();

    // Split the points into left and right subtrees
    let (tmp_left, tmp_right) = points.split_at(split_index);

    if points.len() > 1000 {
        // Parallel construction for large input
        let (left, right): (Arc<Node>, Arc<Node>) = rayon::join(
            || build_recursive(tmp_left, id_offset),
            || build_recursive(tmp_right, id_offset + split_index),
        );
        Node::new_internal(left, right, (split_index + id_offset) * 2)
    } else {
        // Sequential construction for small input
        let left = build_recursive(tmp_left, id_offset);
        let right = build_recursive(tmp_right, id_offset + split_index);
        Node::new_internal(left, right, (split_index + id_offset) * 2)
    }
}

fn collect_leaf_nodes(node: &Arc<Node>, leaves: &mut Vec<Arc<Node>>) {
    if node.is_leaf() {
        leaves.push(node.clone());
    } else {
        if let Some(ref left_node) = node.left {
            collect_leaf_nodes(left_node, leaves);
        }
        if let Some(ref right_node) = node.right {
            collect_leaf_nodes(right_node, leaves);
        }
    }
}

fn gather_leaves(root: &Arc<Node>) -> Vec<Arc<Node>> {
    let mut leaves = Vec::new();
    collect_leaf_nodes(root, &mut leaves);
    leaves
}

fn update_nearest(
    query: &Point2d<f64>,
    neighbors: &mut Vec<usize>,
    distances: &mut Vec<f64>,
    vertex: &(Point2d<f64>, usize), // Vertex contains the point and its original index in input vector
    k: usize,
) {
    let dist = query.get_distance(&vertex.0); // Calculate distance to the candidate point
    if dist < distances[0] {
        neighbors[0] = vertex.1; // Update with the candidate point's index
        distances[0] = dist;
        for i in 1..k {
            if distances[i - 1] < distances[i] {
                distances.swap(i - 1, i);
                neighbors.swap(i - 1, i);
            } else {
                break;
            }
        }
    }
}

fn parallel_knn_search(
    leaves: Vec<Arc<Node>>,
    queries: Vec<(Point2d<f64>, usize)>,
    k: usize,
) -> Vec<Vec<usize>> {
    queries
        .into_par_iter()
        .map(|(query, query_idx)| {
            let mut neighbors = vec![query_idx; k]; // Initialize with the query index
            let mut distances = vec![f64::INFINITY; k];

            for leaf in &leaves {
                if let Some(vertex) = &leaf.vertex {
                    if vertex.1 != query_idx {
                        update_nearest(&query, &mut neighbors, &mut distances, vertex, k);
                    } 
                }
            }

            neighbors
        })
        .collect()
}

pub fn ann(inp: &[Point2d<f64>], k: usize, res: &mut Vec<Vec<usize>>) {
    // Prepare indexed points
    let indexed_points: Vec<(Point2d<f64>, usize)> = inp
        .iter()
        .enumerate()
        .map(|(idx, point)| (*point, idx))
        .collect();

    // Build the tree
    let root = build_recursive(&indexed_points, 0);

    // Compute well-separated realizations (part of tree-build process)
    wsr(&root, 2.1, k);

    // Gather all leaf nodes (Points reside in leaf nodes only)
    let leaves = gather_leaves(&root);

    // Perform parallel knn search
    *res = parallel_knn_search(leaves, indexed_points, k);
}