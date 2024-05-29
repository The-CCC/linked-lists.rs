#![allow(unsafe_code)]
#![allow(dead_code)]
use std::ptr::null_mut;

struct Node {
    data: i32,
    prev: *mut Node,
    next: *mut Node
}

impl Node {
    fn new(data: i32) -> Self {
        Node {
            data,
            prev: null_mut(),
            next: null_mut(),
        }
    }
}

struct DLL {
    head: *mut Node,
    tail: *mut Node,
}


impl DLL {
    fn new() -> Self {
	DLL {
	    head: null_mut(),
	    tail: null_mut()
	}
    }

    fn ins_at_beg(&mut self, data: i32) {
	unsafe {
	    let new_node = Box::into_raw(Box::new(Node::new(data)));
	    if ! self.head.is_null() {
		(*new_node).next = self.head;
		(*self.head).prev = new_node;
	    }
	    // if no element the 1 element is also the last
	    else { self.tail  = new_node; }
	    self.head = new_node;
	}
    }

    fn ins_at_end(&mut self, data: i32) {
	unsafe {
	    // similar stuff
	    let new_node = Box::into_raw(Box::new(Node::new(data)));
	    if ! self.tail.is_null() {
		(*new_node).prev = self.tail;
		(*self.tail).next = new_node;
	    }
	    else { self.head = new_node; }
	    self.tail = new_node;
	}
    }

    fn ins_at_index(&mut self, data: i32, index: i8) {
	unsafe {
	    if index == 0 { self.ins_at_beg(data); }

            let mut current = self.head;
            let mut i = 0;

            // Traverse the list to find the position or reach the end
            while !current.is_null() && i < index {
		current = (*current).next;
		i += 1;
            }

            // iF we have reached the end of the list insert at the end
            if current.is_null() {
		self.ins_at_end(data);
		return;
	    }

	    // else insert at pos
            let new_node = Box::into_raw(Box::new(Node::new(data)));

            (*new_node).prev = (*current).prev;
            (*new_node).next = current;

            if !(*current).prev.is_null() {
		(*(*current).prev).next = new_node;
            }
	    else {
		self.head = new_node; //  current is the head node
            }
            
            (*current).prev = new_node;
	}
    }


    fn print_list(&self) {
	unsafe {
	    let mut current = self.head;
	    while !current.is_null() {
		print!("{} → ", (*current).data);
		current = (*current).next;
	    }
	    println!("null");
	}
    }
}

fn main(){
    let mut my_dll = DLL::new();
    my_dll.ins_at_beg(1);
    my_dll.ins_at_beg(4);
    my_dll.ins_at_end(3);
    my_dll.ins_at_index(2,3);
    my_dll.print_list();
}
