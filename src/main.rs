fn main() {
    print_repetition_count("abcabcabc");
    print_repetition_count("abccbaabccba");
    print_repetition_count("blendingblender");
    print_repetition_count("blendingblenderblendingblender");
    print_repetition_count("fakfakfak");
    print_repetition_count("booboobooboo");
}

fn print_repetition_count(input: &str){
    let count = count_repetition(&input);
    println!("{count}");
}

/// Count how many times a pattern is repeated in a string
fn count_repetition(input: &str) -> usize {
    let length = input.len();
    let half_length = length / 2;
    for pattern_length in 1..=length {
        // check if we can divide completely, skip iteration if not
        if length % pattern_length != 0 {
            // impossible to divide in half if greater
            if pattern_length > half_length {
                break;
            }
            continue;
        }

        let pattern = &input[..pattern_length];
        let mut fully_repeated = true;

        for start_index in (pattern_length..length).step_by(pattern_length) {
            let next_pattern = &input[start_index..start_index + pattern_length];

            // if equal, we won't set fully repeated to false
            if next_pattern == pattern {
                continue;
            }

            fully_repeated = false;
            break;
        }

        if fully_repeated {
            return length / pattern_length;
        }
    }
    return 1;
}
