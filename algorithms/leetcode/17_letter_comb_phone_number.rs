impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let map: std::collections::HashMap<char, &str> = [
            ('1', ""),
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .cloned()
        .collect();
        
        let mut result: Vec<String> = Vec::new();
        for num in digits.chars() {
            let letters_of_number = map.get(&num).unwrap();
            let mut temp: Vec<String> = vec![];
            for letter in letters_of_number.chars() {
                if result.len() == 0 {
                    temp.push(letter.to_string());
                } else {
                    for item in result.iter() {
                        temp.push(item.to_owned() + &letter.to_string());
                    }
                }
            }
            result = temp;
        }
        result
    }
}