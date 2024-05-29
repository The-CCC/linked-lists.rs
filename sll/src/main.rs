#![allow(unsafe_code)]
#![allow(dead_code)]
use std::ptr::null_mut;

struct NodeS {
    data: i32,
    next: *mut NodeS,
}
impl NodeD {
    fn init(data: i32) -> Box<NodeD> {
	Box::new(NodeD {
	    data,
	    prev: null_mut(),
	    next: null_mut(),
	})
    }

    fn insert(head: &mut *mut NodeD, data: i32, pos: i32) {
        unsafe {
            let new_node = Box::into_raw(NodeD::init(data));
            if pos == 0 {
                // Insert at the beginning
                if !(*head).is_null() {
                    (*new_node).next = *head;
                    (*(*head)).prev = new_node;
                }
                *head = new_node;
            }
	    else if pos == -1 {
                let mut temp = *head;
                if temp.is_null() {
                    // If the list is empty, insert as the only node
                    *head = new_node;
                }
		else {
                    // Traverse to the end of the list
                    while !(*temp).next.is_null() {
                        temp = (*temp).next;
                    }
                    (*temp).next = new_node;
                    (*new_node).prev = temp;
                }
            }
	    else {
                let mut temp = *head;
                let mut index = 0;
                while !temp.is_null() && index < pos {
                    temp = (*temp).next;
                    index += 1;
                }

                if temp.is_null() {
                    let mut temp = *head;
                    while !(*temp).next.is_null() {
                        temp = (*temp).next;
                    }
                    (*temp).next = new_node;
                    (*new_node).prev = temp;
                }
		else {
                    // Insert in the middle
                    (*new_node).next = temp;
                    (*new_node).prev = (*temp).prev;
                    if !(*temp).prev.is_null() {
                        (*(*temp).prev).next = new_node;
                    }
		    else {
                        *head = new_node;
                    }
                    (*temp).prev = new_node;
                }
            }
        }
    }
}


	
impl NodeS {
    fn init(data: i32) -> Self {
        NodeS {
            data,
            next: null_mut(),
        }
    }

    fn insert(head: &mut *mut NodeS, data: i32, pos: i32) {
        unsafe {
            let new_node = Box::into_raw(Box::new(NodeS::init(data)));

            if pos == 0 {
                (*new_node).next = *head;
                *head = new_node;
            }
	    else if pos == -1 {
                let mut temp = *head;
                if temp == null_mut() {
                    *head = new_node;
                }
		else {
                    while (*temp).next != null_mut() {
                        temp = (*temp).next;
                    }
                    (*temp).next = new_node;
                }
            }
	    else {
                let mut temp = *head;
                for _ in 0..=pos {
                    if temp == null_mut() {
                        panic!("Position out of bounds");
                    }
                    temp = (*temp).next;
                }
                if temp == null_mut() {
                    panic!("Position out of bounds");
                }
                (*new_node).next = (*temp).next;
                (*temp).next = new_node;
            }
        }
    }
}

fn main() {
    let mut head = null_mut();

    NodeS::insert(&mut head, 10, 0); // Insert at the beginning
    NodeS::insert(&mut head, 20, -1); // Insert at the end
    NodeS::insert(&mut head, 25, 1); // Insert at position 1
    NodeS::insert(&mut head, 26, 3); // Insert at position 1
    NodeS::insert(&mut head, 15, -1); // Insert at position 1
    NodeS::insert(&mut head, 235, -1); // Insert at position 1
    NodeS::insert(&mut head, 45, 6); // Insert at position 1
    NodeS::insert(&mut head, 75, -1); // Insert at position 1

    unsafe {
        let mut current = head;
        while current != null_mut() {
            print!("{} -> ", (*current).data);
            current = (*current).next;
        }
        println!("null");
    }
}
