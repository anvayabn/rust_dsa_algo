
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

mod ceiling{

    /* return the target number or the greatest  */
    pub fn ceil(vec: &mut Vec<i32> , target: i32)->i32{ 

        // first sort the vector 
        vec.sort(); 

        //if the target number is not present 
        if target > vec[vec.len() - 1]{
            return -1;
        }

        let mut i = 0; 
        let mut j = (vec.len() - 1 ) as i32;
        
        while i <= j {

            let mid = i + (j - i)/2 ;
            if  vec[mid as usize] < target {
                i = mid + 1;
            }else if vec[mid as usize] > target {
                j = mid - 1; 
            }else {
                return mid;
            }
        }
        i as i32 

    }
}
mod test{
    use super::*;
    #[test]
    fn test_binary_test(){

        let mut nums = vec![1,2,4,5,6,7,8,10,22]; 
        let target = 5; 
        let ans = 3; 

        assert_eq!(binary_search(&mut nums, target), ans); 

    }

    #[test]
    fn test_ceil(){
        let mut nums = vec![1,2,4,5,6,7,8,10,22]; 
        let target = 9; 
        let ans = 7; 

        assert_eq!(ceiling::ceil(&mut nums, target), ans); 
    }
}
