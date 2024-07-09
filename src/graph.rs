use text_io::read;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

const MAX_VERTEX: usize = 20;

pub struct Graph {
    vertex: Vec<char>,
    adjacency_matrix: [[i32; MAX_VERTEX]; MAX_VERTEX],
    pub vertex_count: usize,
    distances: Vec<i32>,
    visited: Vec<bool>,
}

#[derive(Eq)]
struct VertexDistance {
    vertex: usize,
    distance: i32,
}

#[derive(Eq, PartialEq, Clone)]
struct Edge {
    source: usize,
    destination: usize,
    value: i32,
}

impl PartialEq for VertexDistance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Ord for VertexDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for VertexDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertex: Vec::new(),
            adjacency_matrix: [[0; MAX_VERTEX]; MAX_VERTEX],
            vertex_count: 0,
            distances: vec![i32::MAX; MAX_VERTEX],
            visited: vec![false; MAX_VERTEX],
        }
    }

    pub fn add_edges(&mut self) {
        println!("Total vertex: ");
        let total_vertex: usize = read!();
        self.vertex_count = total_vertex;
        self.vertex_creation();

        loop {
            println!("Type '-1' to stop or type the vertex source: ");
            let source: isize = read!();

            if source < 0 {
                break;
            } else {
                let source = source as usize;
                let destination: usize = read!();
                let value: i32 = read!();
                if source < self.vertex_count && destination < self.vertex_count && self.get_edge(source, destination) == 0 {
                    self.adjacency_matrix[source as usize][destination] = value;
                    self.adjacency_matrix[destination][source as usize] = value;
                } else {
                    println!("Invalid source or destination vertex.");
                }
            }
        }
    }

    fn vertex_creation(&mut self) {
        for i in 0..self.vertex_count {
            let vertex = std::char::from_u32(('A' as u32) + i as u32).unwrap();
            self.vertex.push(vertex);
        }
    }

    pub fn get_edge(&self, first: usize, second: usize) -> i32 {
        self.adjacency_matrix[first][second]
    }

    pub fn set_edge(&mut self) {
        println!("Vertex of the edge to change: ");
        let source: usize = read!();
        let destination: usize = read!();
        println!("New value: ");
        let value: i32 = read!();
        self.adjacency_matrix[source][destination] = value;
    }

    pub fn print_graph(&self) {
        println!("\nAdjacency Matrix:");
        for i in 0..self.vertex_count {
            print!("{}", self.vertex[i]);
            for j in 0..self.vertex_count {
                print!("{:4} ", self.get_edge(i, j));
            }
            println!();
        }
    }

    pub fn dijkstra(&mut self, source: usize, destination: usize) {
        self.distances[source] = 0;
        let mut min_heap = BinaryHeap::new();
        min_heap.push(VertexDistance { vertex: source, distance: 0 });

        while let Some(VertexDistance { vertex, distance }) = min_heap.pop() {
            if self.visited[vertex] {
                continue;
            }
            self.visited[vertex] = true;

            if vertex == destination {
                println!("From vertex {} to vertex {} is: {}", self.vertex[source], self.vertex[destination], distance);
                self.reset();
                return;
            }

            for neighbor in 0..self.vertex_count {
                if self.adjacency_matrix[vertex][neighbor] > 0 && !self.visited[neighbor] {
                    let new_distance = distance + self.adjacency_matrix[vertex][neighbor];
                    if new_distance < self.distances[neighbor] {
                        self.distances[neighbor] = new_distance;
                        min_heap.push(VertexDistance { vertex: neighbor, distance: new_distance });
                    }
                }
            }
        }

        println!("There's no connection between vertex {} and vertex {}!", self.vertex[destination], self.vertex[source]);
        self.reset();
    }

    fn reset(&mut self) {
        self.visited.iter_mut().for_each(|v| *v = false);
        self.distances.iter_mut().for_each(|d| *d = i32::MAX);
    }

    pub fn kruskal(&mut self) {
        let mut edges: Vec<Edge> = Vec::new();

        for i in 0..self.vertex_count {
            for j in i + 1..self.vertex_count {
                if self.adjacency_matrix[i][j] > 0 {
                    edges.push(Edge {
                        source: i,
                        destination: j,
                        value: self.adjacency_matrix[i][j],
                    });
                }
            }
        }

        edges.sort();

        let mut parent = (0..self.vertex_count).collect::<Vec<_>>();
        let mut rank = vec![0; self.vertex_count];
        
        let mut mst = Vec::new();
        let mut total_value = 0;

        for edge in edges {
            if Self::find(&mut parent, edge.source) != Self::find(&mut parent, edge.destination) {
                Self::union(&mut parent, &mut rank, edge.source, edge.destination);
                mst.push(edge.clone());
                total_value += edge.value;
            }
        }

        println!("\nMinimum Spanning Tree:");
        for edge in mst {
            println!("{} --> {} = {}", self.vertex[edge.source], self.vertex[edge.destination], edge.value);
        }

        println!("Total weight of the MST: {}", total_value);
    }

    fn find(parent: &mut [usize], u: usize) -> usize {
        if parent[u] != u {
            parent[u] = Self::find(parent, parent[u]);
        }
        parent[u]
    }

    fn union(parent: &mut [usize], rank: &mut [usize], u: usize, v: usize) {
        let root_u = Self::find(parent, u);
        let root_v = Self::find(parent, v);

        if root_u != root_v {
            if rank[root_u] > rank[root_v] {
                parent[root_v] = root_u;
            } else if rank[root_u] < rank[root_v] {
                parent[root_u] = root_v;
            } else {
                parent[root_v] = root_u;
                rank[root_u] += 1;
            }
        }
    }
}