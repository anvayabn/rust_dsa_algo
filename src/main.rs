use std::collections::linked_list;

mod list_op;
mod bin_search;
mod linkedlist;
fn main() {
    
    /* list operation  */

    /* intialise a vector  */
    let v = vec![2, 3, 5, 6, 7]; 
    let target = 11; 
    
    let tup = list_op::two_sum(&v, target); 

    println!("Tuple returned {:?}", tup); 

    let mut v1 = vec![2, 3, 6, 6, 7];

    let mut ret_value = list_op::rd::remove_duplicates(&mut v1); 

    println!("Ret vale removing duplicates {ret_value}"); 

    let mut v1 = vec![2, 3, 6, 1, 7];

    ret_value = bin_search::binary_search(&mut v1, 1);  

    println!("The index of the target after binary search is : {ret_value}"); 


    println!("Creating linked list ..."); 

    /* Create 4 nodes  */
    let n4 = linkedlist::linked_list::Node::new(4); 
    let n3 =  linkedlist::linked_list::Node::new_wnext(3, n4); 
    let n2 =  linkedlist::linked_list::Node::new_wnext(2, n3); 
    let n1 =  linkedlist::linked_list::Node::new_wnext(1, n2); 
    
    linkedlist::linked_list::print_linked_list(&n1);

}
