#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn check_sublist<T: PartialEq> (superlist: &[T], sublist: &[T]) -> bool{
    let mut temp = superlist.clone();
    while !temp.is_empty() {
        if temp.starts_with(sublist) {
            return true;
        }
        temp = &temp[1..];
    }
    false
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list, _second_list) {
        x if x.0 == x.1 => Comparison::Equal,
        // check if superlist
        x if x.0.len() > x.1.len() && check_sublist(x.0, x.1) => Comparison::Superlist,
        // check if sublist
        x if x.0.len() < x.1.len() && check_sublist(x.1, x.0) => Comparison::Sublist,
        _ => Comparison::Unequal
    }
}
