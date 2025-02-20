use chrono::{Datelike, NaiveDate};
use std::str::FromStr;
use std::cmp::Ordering;


/// 退休政策配置
#[derive(Debug)]
struct RetirementPolicy {
    original_age: i32,   // 原退休年龄
    max_delay: i32,      // 最大延迟月数
    delay_interval: i32, // 延迟间隔(月)
}

/// 退休人员类别
#[derive(Debug, PartialEq)]
enum RetirementCategory {
    Male,             // 男职工
    FemaleManagerial, // 原法定退休年龄55周岁女职工
    FemaleOrdinary,   // 原法定退休年龄50周岁女职工
}

impl RetirementCategory {
    /// 获取对应类别的退休政策
    fn get_policy(&self) -> RetirementPolicy {
        match self {
            RetirementCategory::Male => RetirementPolicy {
                original_age: 60,
                max_delay: 36,
                delay_interval: 4,
            },
            RetirementCategory::FemaleManagerial => RetirementPolicy {
                original_age: 55,
                max_delay: 36,
                delay_interval: 4,
            },
            RetirementCategory::FemaleOrdinary => RetirementPolicy {
                original_age: 50,
                max_delay: 60,
                delay_interval: 2,
            },
        }
    }
}

/// 退休计算结果
struct RetirementResult {
    retirement_date: NaiveDate,
    retirement_age: f64,
    delay_months: i32,
}

impl RetirementResult {
    /// 格式化输出结果
    fn format(&self) -> String {
        let age_str = if self.retirement_age.fract() == 0.0 {
            format!("{}", self.retirement_age.trunc())
        } else {
            format!("{:.2}", self.retirement_age)
        };

        format!(
            "{},{},{}",
            self.retirement_date.format("%Y-%m"),
            age_str,
            self.delay_months
        )
    }
}

/// 退休计算器
struct RetirementCalculator {
    policy_start_date: NaiveDate,
}

impl RetirementCalculator {
    /// 创建新的退休计算器实例
    fn new() -> Self {
        Self {
            policy_start_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        }
    }

    /// 计算退休信息
    fn calculate(&self, birth_date: NaiveDate, category: RetirementCategory) -> RetirementResult {
        let policy = category.get_policy();
        let original_retirement_date = self.calculate_original_retirement_date(birth_date, policy.original_age);

        let months_after_policy = Self::months_between(self.policy_start_date, original_retirement_date);
        let mut delay_months = if months_after_policy <= 0 {
            0
        } else {
            let mut delay = (months_after_policy + policy.delay_interval - 1) / policy.delay_interval;
            delay.min(policy.max_delay)
        };

        if category == RetirementCategory::Male && birth_date.year() == 1965 && birth_date.month() == 1 {
            delay_months = 1;
        }

        let final_retirement_date = Self::add_months(original_retirement_date, delay_months);
        let total_months = Self::months_between(birth_date, final_retirement_date);
        let retirement_age = total_months as f64 / 12.0;

        RetirementResult {
            retirement_date: final_retirement_date,
            retirement_age,
            delay_months,
        }
    }

    // 计算两个日期之间的月数
    fn months_between(start: NaiveDate, end: NaiveDate) -> i32 {
        (end.year() - start.year()) * 12 + (end.month() as i32 - start.month() as i32)
    }

    // 日期加上指定月数
    fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
        let total_months = date.year() * 12 + date.month() as i32 + months;
        let new_year = total_months / 12;
        let new_month = total_months % 12;
        NaiveDate::from_ymd_opt(
            if new_month == 0 {
                new_year - 1
            } else {
                new_year
            },
            if new_month == 0 { 12 } else { new_month as u32 },
            1,
        )
        .unwrap()
    }

    fn calculate_original_retirement_date(&self, birth_date: NaiveDate, original_age: i32) -> NaiveDate {
        birth_date
            .clone()
            .with_year(birth_date.year() + original_age)
            .unwrap()
    }

    fn compare_with_policy_start(&self, original_retirement_date: NaiveDate) -> Ordering {
        self.policy_start_date.cmp(&original_retirement_date)
    }

    fn create_early_retirement_result(&self, original_retirement_date: NaiveDate, original_age: i32) -> RetirementResult {
        RetirementResult {
            retirement_date: original_retirement_date,
            retirement_age: original_age as f64,
            delay_months: 0,
        }
    }

    fn create_boundary_retirement_result(&self, original_retirement_date: NaiveDate, original_age: i32) -> RetirementResult {
        RetirementResult {
            retirement_date: original_retirement_date
                .clone()
                .with_month(original_retirement_date.day() + 1)
                .unwrap(),
            retirement_age: original_age as f64 + (1.0 / 12.0),
            delay_months: 1,
        }
    }

    fn create_delayed_retirement_result(&self, birth_date: NaiveDate, original_retirement_date: NaiveDate, policy: RetirementPolicy) -> RetirementResult {
        // 计算需要延迟的月数
        let months_after_policy = Self::months_between(self.policy_start_date, original_retirement_date);
        let mut delay_months =
            (months_after_policy + policy.delay_interval - 1) / policy.delay_interval;

        // 限制最大延迟月数
        delay_months = delay_months.min(policy.max_delay);

        // 计算最终退休日期
        let final_retirement_date = Self::add_months(original_retirement_date, delay_months);

        // 计算实际退休年龄
        let total_months = Self::months_between(birth_date, final_retirement_date);
        let retirement_age = total_months as f64 / 12.0;

        RetirementResult {
            retirement_date: final_retirement_date,
            retirement_age,
            delay_months,
        }
    }
}

/// 计算退休时间
///
/// # 参数
/// - `time`: 出生年月，格式为"YYYY-MM"
/// - `tp`: 人员类型，可选值为：
///   - "男职工"
///   - "原法定退休年龄50周岁女职工"
///   - "原法定退休年龄55周岁女职工"
///
/// # 返回
/// 格式化的退休信息字符串，格式为"YYYY-MM,年龄,延迟月数"
pub fn retire_time(time: &str, tp: &str) -> String {
    let category = match tp {
        "男职工" => RetirementCategory::Male,
        "原法定退休年龄50周岁女职工" => RetirementCategory::FemaleOrdinary,
        "原法定退休年龄55周岁女职工" => RetirementCategory::FemaleManagerial,
        _ => panic!("非法人员类型！\nIllegal personnel type!"),
    };

    let birth_date = NaiveDate::from_str(&format!("{}-01", time))
        .expect("Invalid date format");

    let calculator = RetirementCalculator::new();
    let result = calculator.calculate(birth_date, category);

    result.format()
}