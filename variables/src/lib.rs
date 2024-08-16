// 不可变函数引用
pub fn umut_variables(x: i32) -> i32 {
    x + 1
}

pub fn mut_variables(x: &mut i32) -> i32 {
    *x = *x + 6;
    *x
}

#[cfg(test)]
mod tests {
    use crate::umut_variables;
    use crate::mut_variables;

    #[test]
    fn umut_variables_is_works() {
        assert_eq!(4, umut_variables(3));
    }

    #[test]
    fn mut_variables_is_works() {
        assert_eq!(11, mut_variables(&mut 5));
    }
}
