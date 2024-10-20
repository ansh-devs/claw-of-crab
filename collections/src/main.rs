
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{BinaryHeap, HashMap,HashSet};

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

    // ===================== Maps =============================

    let mut mp = HashMap::new();
    mp.insert(1,1);
    mp.insert(2,2);
    mp.insert(3,3);

    // will override the value if already present for the same key. 
    mp.insert(3,9);
    
    // removes the Key Value Pair with the corresponding key
    let val = mp.remove(&1);
    println!("{:?}\n",val);

    // removes the entry and returns the key value which has been removed.
    let val = mp.remove_entry(&2);
    println!("{:?}\n",val);

    // check if the key exists in the Map.
    println!("{:?}\n",mp.contains_key(&2));
    

    
    // ===================== Sets =============================

    let mut hs = HashSet::new();
    
    hs.insert(1);
    hs.insert(2);
    hs.insert(3);
    hs.insert(4);

    // Iteration on the Set
    for it in hs.iter() {
        println!("Iterator : {}",it);
    }

    let mut hs2 = HashSet::new();
    hs2.insert(1);
    hs2.insert(6);
    hs2.insert(7);
    hs2.insert(8);

    // returns the elements which are present in the both Sets
    for x in hs.intersection(&hs2) {
        println!("Intersection : {}",x);
    }

    // Shorthand way of the above.
    let intersection = &hs & &hs2;
    for x in intersection {
        println!("ShortHand Intersection : {}",x);    
    }

    
    // returns the elements with distinct occurence in both the Sets.
    let union = &hs | &hs2;
    for x in union {
        println!("Union : {}",x);
        
    }

    // returns the elements which are in the first Set But Not in Second Set.
    let diff = &hs - &hs2;
    for x in diff {
        println!("{}",x);
    }

    
    // returns bool value of true if the set is empty else returns false.
    println!("{}",hs.is_empty());

    // returns the length of the set.
    println!("{}",hs.len());

}
