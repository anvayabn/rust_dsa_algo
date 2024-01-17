
/* Binary search the list to find the 
    target element and return the index  */

pub fn binary_search(vec: &mut Vec<i32> , target: i32) -> usize { 

    // sort the vector 
    vec.sort(); 

    /* init the start and end pointers */
    let mut start = 0 ; 
    let mut end = vec.len() - 1 ; 

    while start <= end { 
        // find middle element 
        let mid = start + (end - start) / 2 ; 

        if target < vec[mid] { 
            end = mid - 1; 
        }else if  target > vec[mid] { 
            start = mid + 1; 
        }else {
            return mid; 
        }
    }

    1000 
} 
