mod graph;

use graph::*;
use text_io::read;

fn main() {
    let mut graphs: Graph = Graph::new();
    loop {
        println!("\n--Menu:--");
        println!("1. Create a graph");
        println!("2. Print graph");
        println!("3. Set a new edge value");
        println!("4. Print an edge value");
        println!("5. Dijkstra");
        println!("6. Kruskal");
        println!("0. Exit");
        println!("Option: ");
        let option: usize = read!();
        std::process::Command::new("clear").status().unwrap();

        match option {
            1 => {
                graphs = Graph::new();
                graphs.add_edges();
                std::process::Command::new("clear").status().unwrap();
            },
            2 => {
                graphs.print_graph();
            },
            3 => graphs.set_edge(),
            4 => {
                println!("\nType the vertex of the edge: ");
                let source: usize = read!();
                let destination: usize = read!();
                println!("\nEdge value: {}", graphs.get_edge(source, destination));
            },
            5 => {
                println!("Enter the index of the source vertex:");
                let source: usize = read!();
                println!("Enter the index of the destination:");
                let destination: usize = read!();
                graphs.dijkstra(source, destination);
            },
            6 => {
                graphs.kruskal();
            }
            0 => {
                println!("\nThanks for using my program!");
                break
            },
            _ => println!("\nInvalid option!\n"),
        }
    }
}
