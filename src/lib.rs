pub fn blueshift(cityname: &str) -> String {
    let digest = format!("#{:x}", md5::compute(cityname));
    return digest[..7].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_result_for_ord() {
        let result = blueshift("Austin");
        assert_eq!(result, "#54c326")
    }
    #[test]
    fn correct_result_for_msp() {
        let result = blueshift("Minneapolis");
        assert_eq!(result, "#c39584")
    }
    #[test]
    fn correct_result_for_jfk() {
        let result = blueshift("New York");
        assert_eq!(result, "#87809c")
    }
    #[test]
    fn correct_result_for_sfo() {
        let result = blueshift("San Francisco");
        assert_eq!(result, "#f4334f")
    }
    
}
