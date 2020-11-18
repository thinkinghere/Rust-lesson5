fn sum_integer_sets(t: &[u32]) -> Option<u32> {
    // 集合中的元素不为空的时候进行遍历
    if t.len() > 0 {
        let mut total:u32 = 0;
        // 遍历迭代器
        for item in t.iter() {
            // 模式匹配将集合中的元素相加
            match total.checked_add(*item) {
                Some(s) => total = s,
                None => return None,
            } 
        }
        Some(total)
    }
    else {
        None
    }
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7,8];  // 集合测试数据

    println!("array sum is {:?}",sum_integer_sets(&arr));

}