use chrono::Datelike;
use chrono::NaiveDate;
use chrono::{Duration, Weekday};

pub fn time_info(time: &str) -> String {
    let parts: Vec<&str> = time.split('-').collect();
    if parts.len() != 3 {
        return "Invalid date format".to_string();
    }

    let year: i32 = parts[0].parse().unwrap_or(0);
    let month: u32 = parts[1].parse().unwrap_or(0);
    let day: u32 = parts[2].parse().unwrap_or(0);

    // 1. 当前是第几周（ISO周数）
    let week_number = calculate_iso_week_number(year, month, day);

    // 2. 周几（1=周一，7=周日）
    let weekday = calculate_weekday(year, month, day);

    // 3. 当天是本年的第几天
    let day_of_year = calculate_day_of_year(year, month, day);

    // 4. 当年还剩多少天
    let days_left_in_year = if is_leap_year(year) {
        366 - day_of_year
    } else {
        365 - day_of_year
    };

    // 5. 距离过年还有多少天
    let days_to_spring_festival = calculate_days_to_spring_festival(year, month, day);

    // 6. 距离下一次A股开盘还有多少天
    let days_to_next_trading_day = calculate_days_to_next_trading_day(year, month, day);

    // 返回格式化字符串
    format!(
        "{},{},{},{},{},{}",
        week_number,
        weekday,
        day_of_year,
        days_left_in_year,
        days_to_spring_festival,
        days_to_next_trading_day
    )
}

// 判断是否是闰年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 == 0) || year % 400 == 0
}

// 计算某年某月某日是星期几（1=周一，7=周日）
fn calculate_weekday(year: i32, month: u32, day: u32) -> u32 {
    // Zeller's Congruence算法
    let mut m = month as i32;
    let mut y = year;
    if m < 3 {
        m += 12;
        y -= 1;
    }

    let h = (day as i32 + (13 * (m + 1)) / 5 + y + y / 4 - y / 100 + y / 400) % 7;
    ((h + 5) % 7 + 1) as u32
}

// 计算某年某月某日是该年的第几天
fn calculate_day_of_year(year: i32, month: u32, day: u32) -> u32 {
    let mut day_of_year = day;
    let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    for m in 0..(month - 1) {
        day_of_year += month_days[m as usize];
    }

    if month > 2 && is_leap_year(year) {
        day_of_year += 1;
    }

    day_of_year
}

// 计算ISO周数
fn calculate_iso_week_number(year: i32, month: u32, day: u32) -> u32 {
    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    date.iso_week().week() as u32
}

// 计算距离下一个春节的天数
fn calculate_days_to_spring_festival(year: i32, month: u32, day: u32) -> u32 {
    // ... 需要实现春节日期计算逻辑 ...
    // 这里暂时返回一个占位值
    let spring_festival = get_spring_festival_date(year);
    let current_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();

    if current_date < spring_festival {
        (spring_festival - current_date).num_days() as u32
    } else {
        let next_spring_festival = get_spring_festival_date(year + 1);
        (next_spring_festival - current_date).num_days() as u32
    }
}

// 获取某年的春节日期（根据农历计算，这里使用固定日期作为示例）
fn get_spring_festival_date(year: i32) -> NaiveDate {
    match year {
        2023 => NaiveDate::from_ymd_opt(year, 1, 22).unwrap(),
        2024 => NaiveDate::from_ymd_opt(year, 2, 10).unwrap(),
        2025 => NaiveDate::from_ymd_opt(year, 1, 29).unwrap(),
        2026 => NaiveDate::from_ymd_opt(year, 2, 17).unwrap(),
        _ => panic!("Spring festival date not defined for year {}", year),
    }
}

// 计算距离下一个A股交易日的天数
fn calculate_days_to_next_trading_day(year: i32, month: u32, day: u32) -> u32 {
    // ... 需要实现A股交易日计算逻辑 ...
    // 这里暂时返回一个占位值
    let current_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let mut days = 1;

    loop {
        let next_date = current_date + Duration::days(days as i64);
        if is_trading_day(next_date) {
            return days;
        }
        days += 1;
    }
}

fn is_trading_day(date: NaiveDate) -> bool {
    //调休 要正常上班
    let special_trading_days = vec![
        NaiveDate::from_ymd_opt(2025, 1, 18).unwrap(),
        NaiveDate::from_ymd_opt(2025, 1, 19).unwrap(),  // 2025-01-19 调休
        // Add other special trading days here
    ];
    // 2025年中国的法定节假日
    let holidays_2025 = [
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),  // 元旦
        NaiveDate::from_ymd_opt(2025, 1, 28).unwrap(), // 除夕
        NaiveDate::from_ymd_opt(2025, 1, 29).unwrap(), // 春节
        NaiveDate::from_ymd_opt(2025, 1, 30).unwrap(),
        NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
        NaiveDate::from_ymd_opt(2025, 2, 1).unwrap(),
        NaiveDate::from_ymd_opt(2025, 2, 2).unwrap(),
        NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
        NaiveDate::from_ymd_opt(2025, 4, 4).unwrap(), // 清明节
        NaiveDate::from_ymd_opt(2025, 4, 5).unwrap(),
        NaiveDate::from_ymd_opt(2025, 4, 6).unwrap(),
        NaiveDate::from_ymd_opt(2025, 5, 1).unwrap(), // 劳动节
        NaiveDate::from_ymd_opt(2025, 5, 2).unwrap(),
        NaiveDate::from_ymd_opt(2025, 5, 3).unwrap(),
        NaiveDate::from_ymd_opt(2025, 6, 7).unwrap(), // 端午节
        NaiveDate::from_ymd_opt(2025, 6, 8).unwrap(),
        NaiveDate::from_ymd_opt(2025, 6, 9).unwrap(),
        NaiveDate::from_ymd_opt(2025, 10, 1).unwrap(), // 国庆节
        NaiveDate::from_ymd_opt(2025, 10, 2).unwrap(),
        NaiveDate::from_ymd_opt(2025, 10, 3).unwrap(),
        NaiveDate::from_ymd_opt(2025, 10, 4).unwrap(),
        NaiveDate::from_ymd_opt(2025, 10, 5).unwrap(),
        NaiveDate::from_ymd_opt(2025, 10, 6).unwrap(),
        NaiveDate::from_ymd_opt(2025, 10, 7).unwrap(),
    ];

    // 如果是调休日，则是交易日
    if special_trading_days.contains(&date) {
        return true;
    }

    // 如果是节假日，则不是交易日
    if holidays_2025.contains(&date) {
        return false;
    }
    // 正常工作日判断
    let weekday = date.weekday();
    weekday != Weekday::Sat && weekday != Weekday::Sun
}
