use std::collections::VecDeque;

//A trait in Rust is a lot like an interface. Structs can use them to gain functionality.
//Traits can also hold generic types.
//This fulfills the second part of the preparatory homework for the maze project.
pub trait Agenda<T> {
    //Checks if an agenda is empty
    fn is_empty(&self) -> bool;

    //Returns the number of elements in the agenda.
    fn size(&self) -> usize;

    //Adds an item to the agenda
    fn add(&mut self, item: T);

    //Removes and returns an item from the agenda.
    //Will return 'None' if the agenda is empty.
    fn remove(&mut self) -> Option<T>;

    //Returns a reference to the next item without removing it.
    //Returns 'None' if the agenda is empty.
    fn peek(&self) -> Option<&T>;
}

//A stack that implements the Agenda trait with LIFO (last in first out).
pub struct MyStack<T> {
    data: Vec<T>,
}

//Defining Methods for the MyStack that are separate from the Agenda trait.
impl<T> MyStack<T> {
    //Creates a new stack that is empty
    pub fn new() -> Self {
        MyStack { data: Vec::new() }
    }
}

//Defining methods for MyStack that implement the Agenda trait
impl<T> Agenda<T> for MyStack<T> {
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn add(&mut self, item: T) {
        //Pushing to the top of the stack
        self.data.push(item);
    }

    fn remove(&mut self) -> Option<T> {
        //Popping data off the top of the stack
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        //Peeking at the top of the stack
        self.data.last()
    }
}

//Using VecDeque (which is a double ended queue in Rust) to create MyQueue
//Uses FIFO (first in first out)
pub struct MyQueue<T> {
    data: VecDeque<T>,
}

//Methods specific to MyQueue
impl<T> MyQueue<T> {
    //A constructor
    pub fn new() -> Self {
        MyQueue {
            data: VecDeque::new(),
        }
    }
}

//Methods for MyQueue that implement the Agenda trait
impl<T> Agenda<T> for MyQueue<T> {
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn add(&mut self, item: T) {
        //Adding the item to the end of the queue
        self.data.push_back(item);
    }

    fn remove(&mut self) -> Option<T> {
        //Removes from the front of the queue
        self.data.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        //Peek at the front of the queue
        self.data.front()
    }
}
