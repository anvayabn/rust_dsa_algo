pub mod linked_list {

    pub struct Node{
        pub data: i32, 
        pub next: Option<Box<Node>>,
    }

    impl Node{

        pub fn new(data: i32) -> Node {
            Node {data, next: None}
        }

        pub fn new_wnext(data: i32, next: Node) -> Node{
            Node {data, next: Some(Box::new(next))}
        }
    }

    pub fn print_linked_list(head: &Node) {
        let mut current = head;

        while let Some(node) = current.next.as_ref() {
            println!("{}", current.data);
            current = node;
        }

        println!("{}", current.data); // Print the last node's data
    }
}