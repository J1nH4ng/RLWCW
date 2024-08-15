// 不可变函数引用
pub fn umut_variables(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use crate::umut_variables;

    #[test]
    fn umut_variables_is_works() {
        assert_eq!(4, umut_variables(3));
    }
}
