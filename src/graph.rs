use text_io::read;
const MAX_VERTEX: usize = 20;

pub struct Graph {
    vertex: Vec<char>,
    adjacency_matrix: [[i32; MAX_VERTEX]; MAX_VERTEX],
    vertex_count: usize,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertex: Vec::new(),
            adjacency_matrix: [[0; MAX_VERTEX]; MAX_VERTEX],
            vertex_count: 0,
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
                if source < self.vertex_count && destination < self.vertex_count && self.get_edge(source, destination) == 0{
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
            let vertex = std::char::from_u32(('a' as u32) + i as u32).unwrap();
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
}