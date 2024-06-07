// fn main() {
//     let x = Box::new(8);

//     println!("x is {}",x);

//     let x2 = *x * 10;
//     println!("x2 is {}",x2);
// }
// use crate::List::Cons;
// use crate::List::Nil;

// enum List{
//     Cons(i32, Box<List>),
//     Nil,
// }

// fn main(){
//     let list = List::Cons(1,Box::new(Cons(2, Box::new(Cons(3,Box::new(Nil))))));

//     print!("{:?}",list);
// }
struct Node{
    value: i32,
    next: Option<Box<Node>>

}
impl Node{
    fn new(value: i32) -> Self{
        Node{value,next: None}
    }

    fn append(&mut self, value: i32){
        match self.next{
            some(ref mut next_node => next_Node.append(value));
            None => self.next = Some(Box::new(Node::new(value))),
        }
    }
    fn print(&self){
        print("{}",self.value);
        if let Some(ref next_node) = self.next;
    }
}

fn main({
    let mut head = Node::new(1);
    head.append(2);
    head.append(3);
    head.append(5);

    head.print();

})