use std::collections::LinkedList;

// LinkedList is list but instead of being continuous in memory the are build with node's which
// hold addres of next node and data of that node
//
// ┏━━━┓   ┏━━━┓   ┏━━━┓   ┏━━━┓  
// ┃PTR┃ ⟶ ┃PTR┃ ⟶ ┃PTR┃ ⟶ ┃PTR┃ ⟶ NULL
// ┃   ┃   ┃   ┃   ┃   ┃   ┃   ┃  
// ┃VAL┃   ┃VAL┃   ┃VAL┃   ┃VAL┃  
// ┗━━━┛   ┗━━━┛   ┗━━━┛   ┗━━━┛  
//
// (LinkedList in rust standard library is actualy double linked list (node have two pointers (to
// next node and provious node)))
//
// if you want to insert or remove from the middle of LinkedList you dont need to shift all the
// data you can change the pointers e.g.: if inserting change the pointer of one node before to new
// one and set pointer on new node to point to next node for removeing just free node and change
// pointer to point to next after removed node

fn main() {
    let mut linklist: LinkedList<u32> = LinkedList::new();

    // you can push to it from both sides 
    linklist.push_front(32);
    println!("ll (push_front) : {linklist:?}");
    linklist.push_back(333);
    println!("ll (push_back) : {linklist:?}");

    linklist.push_back(333);
    linklist.push_back(333);

    // you can pop from both sides
    linklist.pop_back();
    println!("ll (pop_back) : {linklist:?}");
    linklist.pop_front();
    println!("ll (pop_front) : {linklist:?}");

    // you can append other LinkedList to existing one
    let mut linklist2: LinkedList<u32> = LinkedList::new();
    linklist2.push_front(3);
    linklist.append(&mut linklist2);

    // you can get the Option<T> from back or front of LinkedList
    println!("for reference : {linklist:?}");
    println!("back : {:?}", linklist.back());
    println!("front : {:?}", linklist.front());

    // you can split linked list at given index e.g.:
    // evrythin from the specified index to the end is moved into new variable
    let linked_split = linklist.split_off(1);
    println!("LinkedList : {linklist:?}");
    println!("linked_split : {linked_split:?}");

    // you can remove evrything from LinkedList
    linklist.clear();
    println!("LinkedList : {linklist:?}");
}
