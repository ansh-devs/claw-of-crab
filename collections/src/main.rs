
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BinaryHeap;

fn main() {

    // ================== Vectors =============================

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
    // ================== Vectors =============================
    


    //  ================== Binary Heap =========================
    let mut bheap= BinaryHeap::new();

    bheap.push(1);
    bheap.push(2);
    bheap.push(3);
    bheap.push(4);
    bheap.push(5);
    bheap.pop();

    println!("{:?}\n",bheap);
    println!("{:?}\n",bheap.peek());
    

    // ================== Binary Heap =========================

}
