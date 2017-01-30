fn main() {
    let mut count = 1;
    loop {
        count += 2;
        if is_prime(count) {
            if contain_number(count, 140390) && contain_number(count, 310190) {
                println!("{}", count);
                break;
            }
        }
    }
}

fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    } else if number <= 3 {
        return true;
    } else if number % 2 == 0 || number % 3 == 0 {
        return false;
    }
    let mut i = 5;
    loop {
        if number % i == 0 || number % i+2 == 0 {
            return false;
        }
        i += 6;
        if i*i > number {
            return true;
        }
    }
}

fn contain_number(number: i32, a: i32) -> bool {
    let mut done = false;
    let mut tmp = number;
    while !done {
        if tmp % 10000 == a {
            return true;
        }
        if tmp / 10 == 0 {
            done = true;
        } else {
            tmp = tmp / 10;
        }
    }
    return false;
}
