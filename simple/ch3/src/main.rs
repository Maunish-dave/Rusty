fn main() {
    // let x = 1000;
    // let x = x + 1;

    // {
    //     let mut x = x * 2;
    //     println!("The value of x is {x}");

    //     x = x + 1;

    //     println!("The value of x is {x}");
    // }

    // println!("The value of x is {x}");

    // let spaces = "    ";
    // let spaces = spaces.len();

    // println!("value of space is {spaces}")

    // let mut spaces = "    ";
    // spaces = spaces.len();

    // let guess = "42".parse().expect("Not a number!");
    // let guess: u32 = "42  ".trim().parse().expect("not a number!");

    // let mut x: u8 = 255;
    // x = x + 1; 

    // let x = 2.1111;
    // let y: f32 = 3.34566;

    // addition
    // let sum = 5 + 10;

    // subtraction
    // let sub = 9.5 - 4.3;

    // multiplication
    // let mul = 4.5 * 3.45;

    // let div: f32 = 56.7/ 12.3; 
    // let floor = 2/3;

    // let rem = 43 % 5;

    // println!("{sum}, {sub}, {mul}, {div}, {floor}, {rem}");

    // let t = true;
    // let f: bool = false;
    // println!("value of t is {t} value of f is {f}");

    // println!("The value of space is {x} and value of y is {y}");

    // let x = 'c';
    // let z: char = 'Z';

    // println!("{x}, {z}");

    // let tup = (500, 'c', 1.0, "hello");
    // let  (a,  b, c, d) = tup;
    // println!("{}, {}, {}, {}", a, b, c, d);

    // let arr = [1,2,3,4,5];
    // let x = 5;
    // let _arr1: [i32; 5] = [1,2,3,4,x];

    // let arr2 = [0; 5];

    // println!("{}",arr2[0]);
    // println!("{}",arr[0]);

    // println!("This is main function");
    // function();
    // just_a_function(10, 'c');
    // // wrong_function();
    // right_function();

    // let x = function_that_returns(10);
    // println!("The value of x is {x}");

    // let number = 100;

    // if number < 150 {
    //     println!("contion satisfied");
    // }
    // else {
    //     println!("condition not satisfied");
    // }

    // let condition = false;

    // let number = if condition {5} else {6};

    // println!("{}", number);

    // let mut x = 1;

    // loop {
    //     println!(" print this" );
    //     x = x + 1;
    //     if x == 100{
    //         break;
    //     }
    // }

    // let mut cnt = 0;

    // let result = loop {
    //     cnt += 1;

    //     if cnt == 100 {
    //         break cnt * 2;
    //     }
    // };

    // println!("{} is the value",result);

    // let mut count = 0;

    // 'counting_up: loop {
    //     println!("coun = {count}");

    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");

    //         if remaining == 9 {
    //             break;
    //         }

    //         if count == 2{
    //             break 'counting_up;
    //         }
            
    //         remaining -= 1;
    //     }

    //     count +=1;
    // }

    // println!("Ent count = {count}");

    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // let a = [1,2,3,4,5,6];
    // let mut index = 0;

    // while index < a.len() {
    //     println!("value of arr{index} is {}",a[index]);
    //     index += 1;
    // }

    // let a = [1,2,3,4,5,6,7];
    // for i in a {
    //     println!("{}",i);
    // }

    for number in (1..4).rev(){
        println!("{number}");
    }

    println!("LIFTOFF!");

}

// fn function() {
//     println!("This is another function")
// }

// fn just_a_function(value: u32, ch: char){
//     println!("Just a function {value} and {ch}");
// }

// // fn wrong_function(){
// //     let x = let y = 6;
// // }

// fn right_function(){
//     let x = {
//         let y = 100;
//         y + 100
//     };

//     println!{"The value of x is {x}"}
// }

// fn function_that_returns(val: u32) -> u32 {
//     val
// }
