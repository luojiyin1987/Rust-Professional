use num_bigint::BigUint;
use num_traits::{One, Zero, ToPrimitive};
use rand::Rng;
use num_prime::nt_funcs::is_prime; 
use num_prime::Primality;

// 辅助函数：计算最大公约数
fn gcd(a: BigUint, b: BigUint) -> BigUint {
    if b.is_zero() {
        a
    } else {
        gcd(b.clone(), a % b)
    }
}

// 辅助函数：Pollard's Rho 算法


fn pollards_rho(n: BigUint) -> BigUint {
    if n.is_one() {
        return n;
    }

    let mut rng = rand::thread_rng();
    // 将 i32 转换为 u32
    let mut x = BigUint::from(rng.gen_range(2u32..=1000));
    let c = BigUint::from(rng.gen_range(2u32..=1000));
    let mut y = x.clone();
    let mut d = BigUint::one();

    while d.is_one() {
        x = (&x * &x + &c) % &n;
        y = (&y * &y + &c) % &n;
        y = (&y * &y + &c) % &n;
        d = gcd(if &x > &y { &x - &y } else { &y - &x }, n.clone());
    }

    d
}

// 主函数：找到最大质因数
pub fn find_max_prime_factor(n: u128) -> u128 {
    let mut n = BigUint::from(n);
    let mut max_prime = BigUint::one();

    // 处理2的因数
    while &n % 2u128 == BigUint::zero() {
        max_prime = BigUint::from(2u128);
        n /= 2u128;
    }

    // 使用 Pollard's Rho 算法分解
    while &n > &BigUint::one() {
        if is_prime(&n, None) == Primality::Yes { // 使用 is_prime 函数并检查返回值
            if &n > &max_prime {
                max_prime = n.clone();
            }
            break;
        }

        let d = pollards_rho(n.clone());
        if &d > &max_prime {
            max_prime = d.clone();
        }
        n /= d;
    }

    max_prime.to_u128().unwrap() // 使用 ToPrimitive trait 的 to_u128 方法
}