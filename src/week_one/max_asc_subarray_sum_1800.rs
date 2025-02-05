use std::cmp::max;

pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut sum: i32= 0;
    let mut temp_sum: i32= 0;
    let mut prev = nums[0];
    for i in 0..nums.len() {
        if nums[i] <= prev {
            if sum > temp_sum {
                temp_sum = sum;
            }
            sum = 0;
        }
        sum += nums[i];
        prev = nums[i];
    }

    max(sum, temp_sum)
}

pub fn one_liner_sol(nums: Vec<i32>) -> i32 {

    nums.chunk_by(|a, b| b > a)
        .map(|c| c.iter().sum())
        .max().unwrap()


}

pub fn max_ascending_sum_type_two(nums: Vec<i32>) -> i32 {

    let mut ans=nums[0];
    let mut curr: i32= 0;
    let mut dec: bool = true;
    for i in 1..nums.len(){
        if nums[i]>nums[i-1] {
            if dec==true {
                dec=false;
                curr+=nums[i]+nums[i-1];
            }else {
                curr+=nums[i];
            }
            ans=max(ans,curr);
        }else {
            dec=true; curr=0;
        }
    }
    ans
}
