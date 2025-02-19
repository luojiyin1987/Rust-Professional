fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

// 找到两个质数，使得它们的和等于给定的偶数
fn find_prime_pair(even: u32) -> Option<(u32, u32)> {
    for i in 2..=even / 2 {
        if is_prime(i) && is_prime(even - i) {
            return Some((i, even - i));
        }
    }
    None
}

pub fn goldbach_conjecture() -> String {
    let numbers = vec![5777, 5993];
    let mut result = String::new();

    for num in numbers {
        if num % 2 != 0 {
            result.push_str(&format!("{},", num));
            continue;
        }

        if let Some((_a, _b)) = find_prime_pair(num) {
            result.push_str(&format!("{},", num));
        } else {
            result.push_str(&format!("{},", num));
        }
    }

    if result.ends_with(',') {
        result.pop();
    }

    result
}
