#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    pub fn test_calculate_synergy_uptime(){
        assert_eq!(add(1, 2), 3);
    }
}