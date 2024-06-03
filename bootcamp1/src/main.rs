// //function that returns a slice
    fn main() {
        
        let my_string = String::from("Hello, Rust!");
        let slice = get_slice(&my_string, 7, 11);

        println!("Slice: {}", slice);


        let my_best = String::from("Hello there");
        let slice = best_slice(&my_best, 7, 11);
    }

    fn get_slice(s: &str, start: usize, end: usize) -> &str {
        &s[start..end]
    }


    fn best_slice(s: &str, start: usize, end: usize) -> &str {
       &s[start..end]
    }

  



