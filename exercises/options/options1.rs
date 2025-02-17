// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.



// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
    fn maybe_icecream(time_of_day: u16) -> Option<u16> {
        // Check if the time is valid (0-23)
        if time_of_day > 23 {
            return None;
        }
    
        // Determine the amount of ice cream left
        if time_of_day < 22 {
            Some(5)
        } else {
            Some(0)
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn check_icecream() {
            assert_eq!(maybe_icecream(9), Some(5));
            assert_eq!(maybe_icecream(10), Some(5));
            assert_eq!(maybe_icecream(23), Some(0));
            assert_eq!(maybe_icecream(22), Some(0));
            assert_eq!(maybe_icecream(25), None);
        }
    
        #[test]
        fn raw_value() {
            // Fix this test by using unwrap_or to handle the Option
            let icecreams = maybe_icecream(12).unwrap_or(0);
            assert_eq!(icecreams, 5);
        }
    }
