// JSON 값에서 안전하게 f64 값을 추출하는 Helper 함수
pub fn parse_f64(value: &serde_json::Value) -> f64 {
    if let Some(num) = value.as_f64() {
        num
    } else if let Some(str_value) = value.as_str() {
        str_value.parse::<f64>().unwrap_or(0.0)
    } else {
        0.0
    }
}