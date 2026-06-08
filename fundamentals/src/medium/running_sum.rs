/*
  Problem 24: Vec — Running Sum

  Write a function that takes a Vec<i32> and returns a new Vec<i32> where each element is the running sum up to that index.
  For example, [1, 2, 3] becomes [1, 3, 6].

  Run the tests for this problem with:
    cargo test --test running_sum_test
*/

pub fn running_sum(v: Vec<i32>) -> Vec<i32> {
  let mut new_vec = vec![];
  let vec_size = v.len() as i32;
  if vec_size == 1 {
    new_vec.push(*v.iter().nth(0).unwrap_or(&0));
  }
  else{
    for i in 0..=vec_size -1 {
      if i == 0 {
    new_vec.push(*v.iter().nth(i as usize).unwrap_or(&0));
  }else{
      new_vec.push(v.iter().nth(i as usize).unwrap_or(&0) + new_vec.iter().nth(i as usize - 1).unwrap_or(&0));
  }
    }
  }
    new_vec
}
