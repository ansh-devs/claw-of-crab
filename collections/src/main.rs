use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut nums:Vec<i32> = vec![];
    nums.push(23);
    nums.push(1);
    nums.push(47);
    nums.push(35);
    nums.push(13);
    nums.push(9);
    nums.push(3);
    nums.push(79);
    nums.push(50);
    nums.push(60);
    let popper = nums.pop();
    print!("{:?}\n",popper);
    
    let two = nums[1];
    print!("{:?}\n",two);

    
    let first = nums.first();
    print!("{:?}\n",first);
    
    nums.sort();
    print!("{:?}\n",nums);

    nums.reverse();
    print!("{:?}\n",nums);

    nums.shuffle(&mut thread_rng());
    println!("{:?}\n",nums);
}
