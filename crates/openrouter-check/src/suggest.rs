use std::collections::HashSet;

pub(crate) fn closest_name<'a>(name: &str, known_names: &'a HashSet<String>) -> Option<&'a str> {
    known_names
        .iter()
        .map(|known_name| (known_name.as_str(), edit_distance(name, known_name)))
        .filter(|(_, distance)| *distance <= 3)
        .min_by_key(|(known_name, distance)| (*distance, known_name.len()))
        .map(|(known_name, _)| known_name)
}

fn edit_distance(left: &str, right: &str) -> usize {
    let right_chars: Vec<_> = right.chars().collect();
    let mut previous: Vec<_> = (0..=right_chars.len()).collect();
    let mut current = vec![0; right_chars.len() + 1];

    for (left_index, left_char) in left.chars().enumerate() {
        current[0] = left_index + 1;

        for (right_index, right_char) in right_chars.iter().enumerate() {
            let insertion = current[right_index] + 1;
            let deletion = previous[right_index + 1] + 1;
            let substitution = previous[right_index] + usize::from(left_char != *right_char);
            current[right_index + 1] = insertion.min(deletion).min(substitution);
        }

        std::mem::swap(&mut previous, &mut current);
    }

    previous[right_chars.len()]
}
