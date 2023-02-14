fn main() {
    //test


    // 1: 10
    println!("1: {} = {}", 10, 10);
    
    // 2: (+ 5 3 4)
    println!("2: {} = {}", 5 + 3 + 4, 12);

    // 3: (- 9 1)
    println!("3: {} = {}", 9 - 1, 8);
    
    // 4: (/ 6 2)
    println!("4: {} = {}", 6 / 2, 3);
    
    // 5: (+ (* 2 4) (- 4 6))
    println!("5: {} = {}", (2 * 4) + ( 4 - 6), 6);
    
    // 6: (define a 3)
    let a = 3;
    println!("6: {} = {}", a, 3);

    // 7: (define b (+ a 1))
    let b =  a + 1;
    println!("7: {} = {}", b, 4);
    
    // 8: (+ a b (* a b))
    println!("8: {} = {}", (a * b) + a + b, 19);

    // 9: (= a b)
    println!("9: {} = {}", (a == b), false);
    
    // 10: (if (and (> b a) (< b (* a b)))
    //         b
    //         a)
    let mut result = 0;
    if (b > a) && (b < (a * b)) {
        result = b;
    } else {
        result = a;
    }
    println!("10: {} = {}", result, b);

    // 11: (cond ((= a 4) 6)
    //           ((= b 4) (+ 6 7 a))
    //           (else 25))
    let mut result = 0;
    if a == 4 {
        result = 6;
    } else if b == 4 {
        result = 6 + 7 + a;
    } else {
        result = 25;
    }
    println!("11: {} = {}", result, 16);

    // 12: (+ 2 (if (> b a) b a))
    let mut result = 0;
    if b > a {
        result = b + 2;
    } else {
        result = a + 2;
    }
    println!("12: {} = {}", result, 6);

    // 13: (* cond ((> a b) a)
    //             ((< a b) b)
    //             (else -1)
    //     (+ a 1))
    let mut result = 0;
    if a > b {
        result = (a + 1) * a;
    } else if a < b {
        result = (a + 1) * b;
    } else {
        result = (a + 1) * -1;
    }
    println!("13: {} = {}", result, 16);

 }