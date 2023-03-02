#[cfg(test)]
mod unit_tests {

    #[test]
    fn bench_day03() {
        let result = day03::day03().unwrap();
        assert_eq!(result, 157);
    }
}