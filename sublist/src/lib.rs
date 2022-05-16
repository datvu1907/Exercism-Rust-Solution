#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let is_a_in_b = |a: &[T], b: &[T]| {
        a.len() == 0 || (a.len() < b.len() && b.windows(a.len()).any(|w| w == a))
    };
    if _first_list.len() == _second_list.len() && _first_list == _second_list {
        Comparison::Equal
    } else if is_a_in_b(_first_list, _second_list) {
        Comparison::Sublist
    } else if is_a_in_b(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
