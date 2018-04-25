enum Directions {
    up,
    down,
    left,
    right
}

const MAX_NUMBER: u8 = 20;

fn main() {
    let mut x = 45; // mutable

    let y = 34; //immutable


    println!("The value of x is {}", x);
    println!("The value off y is {}", y);

    x = 60;

    println!("The value of x is {}", x);

    // data types

    let _x32 = 45;  // by default 32 bit
    println!("Then value x32 is {}", _x32);
    let _x64: i64 = 45; // 64 bit
    println!("The value of x64 is {}", _x64);
    let _x_un_signed: u64 = 45; // unsigned non negative number / variable
    println!("The of x unsigned is {}", _x_un_signed);

    let _y32 = 6.7; // floating point by default 32bit f32
    println!("The value of y32 {}", _y32);
    let _y64: f64 = 6.7; // floatin point by 64 bit f64
    println!("The value of y64 {}", _y64);

    let _co: bool = true;

    println!("The value of co is {}", _co);

    // if and else

    let _xa = 20; 

    if _xa > 30 {
        println!("Value is greater than 30");
    } else {
        println!("Value is not greater than 30");
    }


    if _xa < 30 {
        println!("Value is less than 30");
    } else {
        println!("Value is greater than 30");
    }

    if _xa != 30 {
        println!("Value is not equal to 30");
    } else {
        println!("Value is equal to 30");
    }

    if _xa == 30 {
        println!("Value is equal to 30");
    } else {
        println!("Value is not equal to 30");
    }

    if _xa == 30 {
        println!("Value is equal to 30");
    } else if _xa == 20 {
        println!("Value is equal to 20");
    } else {
        println!("Value is not equal to 30");
    }

    // infinite loop 

    let mut value_a = 0;

    loop {
        value_a += 1;

        if value_a == 7 {
            continue;
        }

        if value_a > 10 {
            break;
        }

        println!("The value of n is {}", value_a);
    }

    // while loop 

    let mut value_n = 0;

    while value_n <= 50 {

        println!("The value of value_n is {}", value_n);

        if value_n % 5 == 0 {
            println!("The value of value_n is multiple of 5 {}", value_n);
        }
        value_n += 1;
    }

    // for loop

    for i in 1..11 {
        println!("The value of i is {}", i);
    }


    let numbers  = 30..50;

    for number in numbers {
        println!("value of number is  {}", number);
    }

    let animals = vec!["Elephant", "Tiger", "Lion"];

   
    // call with iterator.
    // Prevent ownership of the value inside the vector
    // for bieng moved to the for loop
    // this is used to access the value of animals in the below for loop
    for animal in animals.iter() {
        println!("Calling iterator on animals return {}", animal);
    }

    // for loop indexed
    for (index, animal) in animals.iter().enumerate() {
        println!("Animal name is  {} with index {}", animal, index);
    }

    for animal in animals {
        println!("The value of animal is {}", animal);
    }

    // enums;
    let player_direction: Directions = Directions::up;

    match player_direction {
        Directions::up => {
         println!("We are heading up");   
        },

        Directions::down => {
            println!("We are heading down");
        }

        Directions::left => {
            println!("We are heading left");
        }

        Directions::right => {
            println!("We are heading right");
        }
    }

    for i in 11..MAX_NUMBER {
        println!("The value i between MAX_NUMBER AND 11 is {}", i)
    }

    // tuples

    let tuple1 = (20, 25, 30, 35);

    let tuple2 = ("Rehan Kodekar", 27, 5.8, false);

    println!("{}",tuple1.2);

    println!("{}", tuple2.0);
    println!("{}", tuple2.1);
    println!("{}", tuple2.2);
    println!("{}", tuple2.3);

    let (name, age, height, writer) = tuple2;

    println!("name is {}", name);
    println!("age is {}", age);
    println!("height is {}", height);
    println!("is is writer {}", writer);
}
