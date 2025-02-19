pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 解析输入字符串，提取数字和原进制
    let (num_str, from_base) = parse_input(num_str);

    // 将输入字符串从原进制转换为十进制
    let decimal = u32::from_str_radix(&num_str, from_base).unwrap_or(0);

    // 将十进制数转换为目标进制
    let mut result = String::new();
    let mut num = decimal;

    if num == 0 {
        return "0".to_string();
    }

    while num > 0 {
        let remainder = num % to_base;
        let digit = if remainder < 10 {
            (remainder as u8 + b'0') as char
        } else {
            (remainder as u8 - 10 + b'a') as char
        };
        result.push(digit);
        num /= to_base;
    }

    result.chars().rev().collect()
}

// 解析输入字符串，提取数字和原进制
fn parse_input(input: &str) -> (String, u32) {
    let parts: Vec<&str> = input.split('(').collect();
    if parts.len() != 2 {
        panic!("Invalid input format");
    }
    let num_str = parts[0].to_string();
    let base_str = parts[1].trim_end_matches(')');
    let from_base = base_str.parse().unwrap_or(10);
    (num_str, from_base)
}
