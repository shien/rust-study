
fn test_for(x: i32) {
    println!("## test_for");
    for y in 0..x {
        println!("for_y: {}", y); // y: i32
    }

    for (i,j) in (5..10).enumerate() {
        println!("for_enumerate: i = {} and j = {}", i, j);
    }
}

fn test_loop(x: i32) {
    println!("## test_loop");
    let mut y = x;
    loop {
        y = y - 1;
        println!("loop_y: {}", y);
        if y < 0 {
            break;
        }
    }
}

fn testtest(x: i32) -> i32 {
    println!("In testtest: {}", x);
    x + 1
}

fn test_array() {
    println!("## test_array");
    let a = [1000.25; 20];
    println!("Array: {}", a[19]);
    println!("Array Length: {}", a.len());
}

fn test_slice() {
    println!("## test_slice");
    let a = [100; 20];
    let complete = &a[..];
    println!("complate.get {:?}",complete.get(1));
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

fn test_func_ref() {
    println!("## test_func_ref");
    let (foo, var) = (5, 4);
    let x: i32 = 3;

    let y = testtest(x);
    println!("testtest returning value is : {}", y);

    println!("You guessed: {}", foo);
    println!("You guessed: {}", var);
    println!("You guessed: {}", x);
}

fn main() {

    test_loop(2);

    test_for(5);

    test_func();

    test_func_ref();

    test_array();

    test_tuple();

    test_slice();

    test_if(2);
}
