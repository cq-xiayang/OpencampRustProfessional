// src/tests.rs
mod calc_time;

#[cfg(test)]
mod tests {
    use super::calc_time::time_info;
    use std::time::{Instant, Duration};

    
    const TEST_CASES: &[(&str, &str)] = &[
        
    ("2025-01-01", "1,3,1,364,27,0"), 
    ("2025-01-18", "3,6,18,347,10,1"), 

    
    ("2025-12-31", "53,3,365,0,29,1"), 
    ("2025-11-01", "44,6,305,60,89,2"), 

    
    ("2025-02-28", "9,5,59,306,0,2"), 
    ("2025-04-01", "14,2,91,274,62,0"), 

    
    ("2025-01-28", "5,2,28,337,1,0"), 
    ("2025-01-30", "5,4,30,335,0,0"), 

    
    ("2025-02-09", "7,7,40,325,12,1"), 
    ("2025-05-01", "18,4,121,244,92,3"), 
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_calc_time() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let start = Instant::now();
            let result = time_info(*input);
            let duration = start.elapsed();

            // 时间超0.2s，判定不合格
            if duration <= Duration::from_millis(200) && result == *expected {
                total_score += 10.0;
            }
        }

        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
