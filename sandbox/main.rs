fn main() {
    let version: u32 = read_version("3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2");
    println!("version:{}", version);
    // create a new empty vector. Vec is Rust's dynamic array type | new() creates an empty vector | mut makes the vector mutable
    
    
    let mut vec = Vec::new();
    // Add the integers 1 and 2 to the vector to contain [1, 2].
    vec.push(1);
    vec.push(2);
    // alternatively use macro vec to replace line 11 to 14
    // let vec = vec![1,2];
    // print the length of the vector 
    println!("len:{}",vec.len());
    // print the first element of the vector using index notation vec[0]
    println!("first element:{}", vec[0]);
}

// Enums
// enum Point {
//     Nothing,
//     TuplePoint(i32, i32),
//     StructPoint {
//         x: i32,
//         y: i32
//     }
// }

// fn get_point(n: u8) -> Point {
//     match n {
//         1 => Point::TuplePoint(-1, 1),
//         2 => Point::StructPoint {
//             x: -1,
//             y: 1
//         },
//         _ => Point::Nothing
//     }
// }

// fn main() {
//     let p = get_point(2);
//     match p {
//         Point::Nothing => println!("no point"),
//         Point::TuplePoint(resX, resY) => println!("x is {} and y is {}", x, y),
//         Point::StructPoint(resX, resY) => println!("x is {} and y is {}", x, y),
//     }
// }
