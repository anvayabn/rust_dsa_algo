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

    pub fn remove_duplicates(vec: &mut Vec<i32>) -> usize {
        if vec.is_empty() {
            return 0;
        }
    
        vec.sort();
        let mut i = 0;
    
        for j in 1..vec.len() {
            if vec[i] != vec[j] {
                i += 1;
                vec[i] = vec[j];
            }
        }
    
        i + 1
    }

    /* Given an array arr[] of size N. The task 
        is to find the sum of the contiguous subarray
         within a arr[] with the largest sum.   */

    /* TO-DO : learn and implement kadane algorithm  */
    
    // pub fn kadane_algorithm( vec: &Vec<i32>) {

    // }

}

mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(&nums, target), (0, 1));   
    }
    
    #[test]
    fn test_remove_duplicates() { 
        let mut nums = vec![2, 7, 11, 15, 2];
        let ans = 4; 

        assert_eq!(rd::remove_duplicates(&mut nums), ans); 
        
    }
}