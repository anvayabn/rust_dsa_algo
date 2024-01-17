mod list_op; 
fn main() {
    
    /* list operation  */

    /* intialise a vector  */
    let v = vec![2, 3, 5, 6, 7]; 
    let target = 11; 
    
    let tup = list_op::two_sum(&v, target); 

    println!("Tuple returned {:?}", tup); 

    let mut v1 = vec![2, 3, 6, 6, 7];

    let ret_value = list_op::rd::remove_duplicates(&mut v1); 

    println!("Ret vale removing duplicates {ret_value}");  
}
