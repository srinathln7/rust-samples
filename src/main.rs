use std::any::type_name_of_val;

mod os;

fn sum(x: u128, y: u128) -> u128 {
    x + y
}

fn compute_30_60_90_tri_side_len(short_side: f64) -> (f64, f64, f64) {
    (
        short_side,
        short_side * 2.0,
        short_side * 3_f64.sqrt(), // "_f64" is an optional type postfix syntax
    )
}

fn incr(a: &mut isize, b: &isize) {
    *a += *b;
}

fn count_total_bytes(byte_slice: &[u8]) -> usize {
    let mut cnt = 0;
    // Underscore indicates unused variable
    for _ in byte_slice {
        cnt += 1;
    }
    // Oops - we didn't need to loop, there's a built-in length method!
    assert_eq!(cnt, byte_slice.len());
    cnt
}

// classic struct
struct Student {
    name: String,
    level: u8,
    remote: bool,
}

// tuple struct
struct Grades(char, char, char, char, f32);

// Unit struct
struct Unit;

#[derive(Debug)]
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEPress(KeyPress),
}

fn divide_by_7(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }

    num / 7
}

fn goodbye(str: &str) -> bool {
    println!("\n{}", str);
    return true;
}

// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // Unit struct without fields or datatypes
    Manual,
    SemiAuto,
    Automatic,
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    // Use the values of the input arguments
    // All new cars always have zero mileage
    let car: Car = Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0,
    };

    return car;
}

fn main() {
    println!("Hello {}, Welcome to Rust!", "Srinath");

    let a;
    a = 1;
    println!("Value of a is {}", a);

    // throw error due to variable immutability

    // a = 2;
    // println!("Value of a is {}", a);

    // Variable shadowing -  We don't define the variable as mutable because each let operation
    // creates a new variable named `n` while shadowing the previous variable binding.
    let n = 2;
    println!("Value of n is {}", n);

    let n = n + 10;
    println!("Value of n is {}", n);

    let n = n * 10;
    println!("Value of n is {}", n);

    // Addition
    println!("1 + 2 = {}", 1i32 + 2);

    // Subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // ^ Try changing `1i32` to `1u32` to see why the type is important

    // Integer Division
    println!("9 / 2 = {}", 9u32 / 2);

    // Float Division
    println!("9 / 2 = {}", 9.0 / 2.0);

    // Multiplication
    println!("3 * 6 = {}", 3 * 6);

    // Declare variable to store result of "greater than" test, Is 1 > 4? -- false
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    let a_char = 'A';

    // Convert the char to its Unicode code point value (u32)
    let code_point = a_char as u32;

    // Print the original char and its Unicode code point
    println!("Character: {}", a_char);
    println!("Unicode Code Point: {}", code_point);

    // Define a char containing a Unicode emoji (🚀)
    let rocket_emoji = '🚀';

    // Convert the char to its Unicode code point value (u32)
    let emoji_code_point = rocket_emoji as u32;

    // Print the original char (emoji) and its Unicode code point
    println!("Emoji: {}", rocket_emoji);
    println!("Unicode Code Point (Emoji): {}", emoji_code_point);

    let student_1 = Student {
        name: String::from("Srinath"),
        remote: true,
        level: 1,
    };
    let mark_1 = Grades('A', 'A', 'A', 'A', 5.0);

    println!(
        "{}, Level: {}. Remote: {}. Grades: {}, {}, {}, {}, Average: {}",
        student_1.name,
        student_1.level,
        student_1.remote,
        mark_1.0,
        mark_1.1,
        mark_1.2,
        mark_1.3,
        mark_1.4
    );

    let click = MouseClick { x: 100, y: 50 };
    println!("Mouse click location: {}, {}", click.x, click.y);

    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\n Key press: {},{}", keys.0, keys.1);

    // Instantiate the WebEvent enum variants

    // Set the boolean WELoad value to true
    let we_load = WebEvent::WELoad(true);

    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);

    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEPress(keys);

    println!(
        "\nWeb Event enum struct: \n\n {:#?} \n\n {:#?}  \n\n {:#?}",
        we_load, we_click, we_key
    );

    let num = 28;
    println!("{} divided by 7 is: {} ", num, divide_by_7(num));

    goodbye("Formal: Good bye.");
    goodbye("Casual: See you later");

    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!(
        "Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!(
        "Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!(
        "Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    // Arrays

    // Initialize array elements using comma-separated list of values
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    println!("Days in every week {:?}", days);

    // Declare array, specify length = 5, specify first element value = "0"
    // Declaration initializes every array element with value = "0"
    // Short form of: let bytes = ["0", "0", "0", "0", "0"]
    let bytes = [0; 5];
    println!("Bytes buffer: {:?}", bytes);

    let first = days[0];
    let second = days[1];
    println!("First = {:?}, Second = {:?}", first, second);

    // Declare MUTABLE array, number of days in february changes
    let mut february = [28; 1];
    println!("February days {:?}", february[0]);

    february[0] = 29;
    println!("February Leap days {:?}", february[0]);

    // Vectors

    // Declare a vector with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);

    let zeroes = vec![0; 5];
    println!("Zeros {:?}", zeroes);

    // Create empty vector, make vector mutable so it can grow and shrink
    let mut fruit = Vec::new();

    // Push some values onto the end of the vector
    // Adding values changes the type from generic to the date type of the value: String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");

    println!("Fruits: {:?}", fruit);
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    // Declare vector with three values
    let mut index_vec = vec![1, 2, 3];
    println!("Vector: {:?}", index_vec);

    index_vec[2] = index_vec[2] + 1;
    println!("Vector: {:?}", index_vec);

    // Try to access the vector with an out-of-bounds index = 10
    // Program compiles, but panics and stops at the invalid expression
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 10'
    // let beyond = index_vec[10];
    let mut val = 42;
    let step = 2;

    // Reference to integers
    let a = &mut val;
    let b = &step;

    println!("Before incr: a == {}, b == {}", a, b);
    incr(a, b);
    println!("After incr: a={}, b=={}", a, b);

    // First, primitive types were inferred from a function signature.
    // If the function sum wasn't part of the program, let a = 1; would be equivalent to let a: i32 = 1;. i32,
    // a 4-byte signed integer, is Rust's default integer type. But, because of the line let c = sum(a, b), the
    // compiler realized that a is actually a u128, a 16-byte unsigned integer.
    let a = 1;
    let b = 3;
    let c = sum(a, b);
    println!("a is a {} with value {:?}", type_name_of_val(&a), a);
    println!("b is a {} with value {:?}", type_name_of_val(&b), b);
    println!("c is a {} with value {:?}", type_name_of_val(&c), c);

    // Second, the type of a dynamic collection was inferred from the type of the item stored
    let mut list = Vec::new();
    list.push(a);
    list.push(b);
    list.push(c);
    println!(
        "list is a {} with value {:?}",
        type_name_of_val(&list),
        list
    );

    let tri_sides = compute_30_60_90_tri_side_len(10.0);
    // Tuple constant indexing:  Each float is accessible by constant position, but not by a variable
    assert_eq!(tri_sides.0, 10.0);
    assert_eq!(tri_sides.1, 20.0);
    assert_eq!(tri_sides.2, 17.32050807568877);

    // Tuple destructuring: Instead of assigning to a single tuple variable, we destructure and assign each tuple item to its own named variable
    let (a, b, c) = compute_30_60_90_tri_side_len(10.0);
    assert_eq!(a, 10.0);
    assert_eq!(b, 20.0);
    assert_eq!(c, 17.32050807568877);

    // Explicit array type declaration
    let numbers: [u64; 3] = [42, 1337, 0];
    println!(
        "number is a {} with value {:?}",
        type_name_of_val(&numbers),
        numbers
    );

    // Inferred array type (`[&str; 4]`, array of read-only string references)
    let operating_systems = ["Linux", "FreeBSD", "Tock", "VxWorks"];
    println!(
        "operating_systems is a {} with value {:?}",
        type_name_of_val(&operating_systems),
        operating_systems
    );

    // Initialization of all elements (1,000 of them) to a single value (0)
    let mut buffer = [0; 1_000];

    // Index-based write access
    for i in 0..1_000 {
        assert_eq!(buffer[i], 0); // Should have been zero-initialized
        buffer[i] = i; // Overwrite with new value
    }
    assert_eq!(buffer[0], 0);
    assert_eq!(buffer[1], 1);
    assert_eq!(buffer[2], 2);

    // Iterator-based write access
    for num in buffer.iter_mut() {
        *num += 7; // "*" is a dereference for write
    }
    assert_eq!(buffer[0], 7);
    assert_eq!(buffer[1], 8);
    assert_eq!(buffer[2], 9);

    // References are crucial for systems programming. Recall that they enable pass-by-reference semantics
    // (hand off a "pointer"), instead of pass-by-value (copy the entire value). That level of control is essential,
    // it enables performant manipulation of large values. The programmer can choose when to perform a shallow copy
    // (duplicate only a reference) and when to perform a deep copy (duplicate all data). The former means less time
    // spent copying bytes and less total memory used.
    let mut x = 3;
    let y = 5;
    incr(&mut x, &y);
    assert_eq!(x, 8);
    assert_eq!(y, 5);

    // Array of 5 items
    let mut buffer_overflow_defenses = [
        "stack canary",
        "ASLR",
        "NX bit",
        "CFI",
        "Intel CET",
        "ARM MTE",
    ];

    // Create an immutable slice of the first 3
    // [..=2] is inclusive range notation, equivalent to [..3]
    let basic_defenses = &buffer_overflow_defenses[..=2];
    assert_eq!(basic_defenses, &["stack canary", "ASLR", "NX bit"]);

    // Create an mutable slice of the last 2
    let advanced_defenses = &mut buffer_overflow_defenses[4..];
    assert_eq!(advanced_defenses, &mut ["Intel CET", "ARM MTE"]);

    // Modify via slice
    advanced_defenses[1] = "safe Rust!";
    // Notice both slice and it's "backing storage" are updated
    assert_eq!(advanced_defenses, &mut ["Intel CET", "safe Rust!"]);
    assert_eq!(buffer_overflow_defenses[5], "safe Rust!");

    let byte_arr: [u8; 4] = [0xC, 0xA, 0xF, 0xE];

    // Vec init shorthand
    let mut byte_vec = vec![0xB, 0xA, 0xD];

    // Push more data dynamically
    byte_vec.push(0xF);
    byte_vec.push(0x0);
    byte_vec.push(0x0);
    byte_vec.push(0xD);

    // Note both types can be borrowed as &[u8]
    assert_eq!(count_total_bytes(&byte_arr), 4);
    assert_eq!(count_total_bytes(&byte_vec), 7);

    // From mod `os`
    let mut my_proc_stopped = os::Proc::new(1);
    os::Proc::set_state(&mut my_proc_stopped, os::State::Stopped);

    let mut my_proc_sleeping = os::Proc::new(3);
    os::Proc::set_state(&mut my_proc_sleeping, os::State::Sleeping);

    let mut my_proc_running = os::Proc::new(2);
    os::Proc::set_state(&mut my_proc_running, os::State::Running);

    let mut proc_queue = vec![my_proc_stopped, my_proc_sleeping, my_proc_running];

    // pub fn sort(&mut self)
    // where
    //     T: Ord,
    // {
    //     // ...code here
    // }

    proc_queue.sort();
    println!("{:#?}", proc_queue);
}
