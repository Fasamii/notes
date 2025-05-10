// vecDeque is contiginuus memory simmilar to vec but instead of len it have start and end which
// indicade initialized memory inside vecDeque heap data evrything around start and end is not
// initialized but allocated
//
// main fuctionality is the feature: if data overflow the end of heap allocated memory with end it
// can wrap around to the begining of memory e.g.:
// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃    ||||||||||||||||||||||||||         ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
//      ↑                        ↑
//      start                    end
// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃|||      ||||||||||||||||||||||||||||||┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
//    ↑      ↑                                
//    end    start                            
//
// this makes it capable of acting as a stack (LIFO) {last in first out} and 
// queque (FIFO) {first in first out}
// you can push and pop from both side's of data structure so it can be queque and stack at the
// same time
//
// it can be less efficient that vec because of gap in memory and wraping from the end to start (not
// contiginuus memory) so cpu not always will fetch data that we would actualy use and overhead of
// evry acces to value in vecDeque because of complicated index accesing logic
//
// you cannot turn vecDeque into slice because it isnt contiginuus

use std::{collections::{self, vec_deque}, vec};

fn main() {
    let mut vecdeq: std::collections::VecDeque<u32> = std::collections::VecDeque::new();

    // you can push to start or end of vecDeque
    println!("{vecdeq:?}");
    vecdeq.push_back(99);
    println!("push back (99) : {vecdeq:?}");
    vecdeq.push_front(23);
    println!("push from (23) : {vecdeq:?}");
    vecdeq.push_back(11);
    println!("push back (11) : {vecdeq:?}");

    // or pop from start or end
    vecdeq.pop_back();
    println!("pop back : {vecdeq:?}");
    vecdeq.pop_front();
    println!("pop front : {vecdeq:?}");

    // you can even insert to vecdeq so you arent limited to sides of the data
    vecdeq.insert(2, 32);
}
