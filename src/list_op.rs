/*  given a list of integers nums and target 
integers return the indices which add up to target */
pub fn two_sum(vec: &Vec<i32>, target: i32)->(usize, usize){ 
    
    println!("Running two sum function"); 
    let vec_len = vec.len();
    println!("Vector length is {vec_len}");

    // initialise two pointers 
    let mut i = 0; 
    let mut j = vec_len - 1; 

    while i < j {
        if  vec[i] + vec[j] == target 
        {
            return (i, j);  
        }
        else if vec[i] + vec[j] < target
        { 
            i+=1;
        }
        else 
        {
            j-=1; 
        }    
    } 
    (i, j)
}

pub mod rd{

    // Remove the duplicates using two pointer 
    pub fn remove_duplicates(vec: &mut Vec<i32>)->usize{ 

        /* TO-DO : Add sorting to the list 
                     (sort in non-decreasing order)  */
        vec.sort(); 

        let mut i = 0 ; 

        /* TO-DO :  j need not be used 
                            so can be optimised */
        for mut j in 0..(vec.len() - 1) {
            if vec[i] == vec[j] { 
                j+=1; 
            }else{
                vec[i+1] = vec[j]; 
                i+=1; 
            } 

        }
        i+1
    }

    /* Given an array arr[] of size N. The task 
        is to find the sum of the contiguous subarray
         within a arr[] with the largest sum.   */

    /* TO-DO : learn and implement kadane algorithm  */
    
    // pub fn kadane_algorithm( vec: &Vec<i32>) {

    // }

}