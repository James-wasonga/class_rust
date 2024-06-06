#[derive(Debug)]

pub struct Vehicle<T,X>{
    customer_one: T,
    customer_two: X,
}

impl <T,X> Vehicle <T,X>{
    pub fn new(customer_one: T, customer_two: X) -> Self{
        Vehicle{customer_one,customer_two}
    }

    pub fn swap(&mut self, other_customer: <T,X>){
        self.customer_one = other_customer.customer_one
    }


}

