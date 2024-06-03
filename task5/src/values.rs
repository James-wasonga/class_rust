pub fn largest_integer(list: &[i32]) -> i32{
    let mut largest: i32 = list[0];

    for &i in list.iter(){
        if i > largest{
            largest = i;
        }
    }
    largest
}

//comparing the largest character 

pub fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &i in list.iter(){
        if i > largest{
            largest = i;
        }
    }
    largest
}


//crating a function for the Generics


// pub fn largest<Z: PartialOrd + Copy > (list: &[Z] ) -> Z{
//     let mut largest: Z = list[0];

//     for &i: Z in list.iter(){
//         if i > largest{
//         largest = i;
//         }
//     }
//     largest
// } 