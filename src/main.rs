fn testtest(x: i32) -> i32 {
    println!("In testtest: {}", x);
    x + 1
}

fn test_array() {
    let a = [1000.25; 20];
    println!("Array: {}", a[19]);
    println!("Array Length: {}", a.len());
}


fn test_tuple() {
    let (x, y, z) = (1, 2, 3);
    println!("test_tuple x is {}", x);
    println!("test_tuple y is {}", y);
    println!("test_tuple z is {}", z);
    let y = (1, 2);
    println!("test_tuple y is {}", y.0);
}

fn test_if(x: i32) {
    if x == 1 {
        println!("x == {}", x);
    } else if x == 2 {
        println!("x == {}", x);
    } else {
        println!("else {}", x);
    }

    let y = if x == 1 {
        3
    } else {
        4
    };
    println!("test_if: y == {}", y);
}


fn test_func() {
    let f: fn(i32) -> i32 = testtest;
    let x: i32 = 3;
    let z = f(x);
    println!("Func availability: {}", z);
}

fn main() {
    let (foo, var) = (5, 4);
    let x: i32 = 3;

    test_func();
    let y = testtest(x);
    println!("testtest returning value is : {}", y);

    test_array();

    test_tuple();

    println!("You guessed: {}", foo);
    println!("You guessed: {}", var);
    println!("You guessed: {}", x);

    test_if(2);
}
