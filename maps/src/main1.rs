use std::collections::HashMap;

fn main() {
    let vector = vec![3,3];
    let result = two_sum(vector, 6);
    println!("{:?}", result);
}



    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut maps: HashMap<i32,i32> = HashMap::with_capacity(nums.len());
        for i in 0..nums.len() {
            let a: &i32 = nums.get(i).unwrap();
            if maps.contains_key(&(target - a)){
                return vec![*maps.get(&(target - a)).unwrap(), i as i32];
            }
            maps.insert(*a,i as i32);
        }
        // println!("{:?}", maps);
        return Vec::new();
    }