// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.


fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> { // 这里传入的是引用，不会修改原来的值
    v.iter().map(|element| { //map 是迭代器方法，用来“映射”每个元素
        // |element| { ... } 是闭包（类似匿名函数）
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element * 2 // 这里闭包对每个元素做 element * 2
    }).collect() // collect() 会把迭代器生成的值收集到一个集合里
}

// 一种等价写法
fn vec_map_another(v: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for element in v.iter() {
        new_vec.push(element * 2);
    }
    new_vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}

fn main() {
    let mut v = vec![1,2,3,4,5];
    v = vec_loop(v);
    println!("v * 2:");
    println!("{:?}", v);
    

    let n = vec_map(&v);
    println!("n:");
    println!("{:?}", n);

    println!("v:");
    println!("{:?}", v);

}