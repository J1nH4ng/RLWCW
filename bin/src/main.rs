use variables;

fn main() {
    let umut_variables_x: i32 = 10;
    println!("{umut_variables_x} is umut and value + 1 = {}", variables::umut_variables(umut_variables_x));

    let mut mut_variables_y: i32 = 10;
    println!("{mut_variables_y} is mutable and value = {}", variables::mut_variables(&mut mut_variables_y));
}
