fn compute(a: u32, b: u32) -> u32 {
    // 显示指定变量为 u32 类型或者不显示指定，由编译器推导其类型
    // let multiplier:u32 = 4;
    let multiplier = 4;
    a + b * multiplier
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
