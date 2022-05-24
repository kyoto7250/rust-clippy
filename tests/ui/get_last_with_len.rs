// run-rustfix

#![warn(clippy::get_last_with_len)]

fn dont_use_last() {
    let x = vec![2, 3, 5];
    let _ = x.get(x.len() - 1); // ~ERROR Use x.last()

    let y = [2, 3, 5];
    let _ = y.get(y.len() - 1); // ~ERROR Use y.last()

    let z = &[2, 3, 5];
    let _ = z.get(z.len() - 1); // ~ERROR Use z.last()
}

fn indexing_two_from_end() {
    let x = vec![2, 3, 5];
    let _ = x.get(x.len() - 2);

    let y = [2, 3, 5];
    let _ = y.get(y.len() - 2);

    let z = &[2, 3, 5];
    let _ = z.get(z.len() - 2);
}

fn index_into_last() {
    let x = vec![2, 3, 5];
    let _ = x[x.len() - 1];

    let y = [2, 3, 5];
    let _ = y[y.len() - 1];

    let z = &[2, 3, 5];
    let _ = z[z.len() - 1];
}

fn use_last_with_different_vec_length() {
    let x = vec![2, 3, 5];
    let y = vec!['a', 'b', 'c'];
    let _ = x.get(y.len() - 1);
}

fn main() {
    dont_use_last();
    indexing_two_from_end();
    index_into_last();
    use_last_with_different_vec_length();
}
