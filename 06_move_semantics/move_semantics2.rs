fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec.clone(); // Clone the input vector to create a new one
    new_vec.push(88);
    new_vec
}

fn main() {
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(&vec0);

    println!("vec0: {:?}", vec0); // Original vector remains unchanged
    println!("vec1: {:?}", vec1); // Modified vector with an additional element

    if vec0 == [22, 44, 66] {
        println!("vec0 is correct!");
    } else {
        println!("vec0 is incorrect!");
    }

    if vec1 == [22, 44, 66, 88] {
        println!("vec1 is correct!");
    } else {
        println!("vec1 is incorrect!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(&vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}