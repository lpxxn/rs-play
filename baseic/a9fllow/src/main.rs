fn main() {
    let arr: [String; 8] = std::array::from_fn(|i| format!("{}-{}", i, i));

    for a in &arr {
        println!("a: {}", a);
    }
    for i in 0..arr.len() {
        // println macro will call Display trait, do not take ownership
        println!("arr[{}]: {}", i, arr[i]);
    }

    println!("arr: {:?}", arr);
    for (i, a) in arr.iter().enumerate() {
        println!("arr[{}]: {}", i, a);
    }
    println!("arr: {:?}", arr);

    let condition = true;
    let number = if condition { 5 } else { 8 };
    println!("The value of number is: {}", number);

    flow2();
    flow3();
}

fn flow2() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result);
}

fn flow3() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                println!("inner1 > 20 count: {}", count);
                // this would break only the inner1 loop
                break 'inner1; // break is also ok
            }
            count += 2;
        }
        count += 5;

        'inner2: loop {
            if count >= 30 {
                println!("inner2 count: {}", count);
                // this breaks the outer loop
                break 'outer;
            }

            // this will continue the outer loop
            continue 'outer;
        }
    }
    println!("count: {}", count);
}
