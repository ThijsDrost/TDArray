pub fn pre_post_vals(prefix: &str, postfix: &str, num: usize) -> String {
    let mut output = format!("{prefix}1{postfix}", prefix=prefix, postfix=postfix);
    for i in 2..=num {
        output.push_str(&format!(", {prefix}{i}{postfix}", prefix=prefix, i=i, postfix=postfix));
    }
    output
}

pub fn const_x_vals_str(num: usize, name: &str) -> String {
    pre_post_vals(&format!("const {}", name), ": usize", num)
}

pub fn x_vals_str(num: usize, name: &str) -> String {
    pre_post_vals(name, "", num)
}

pub fn array_str(num: usize) -> String {
    format!("Array{}D<T, {}>", num, x_vals_str(num, "D"))
}

pub fn index_str(num: usize) -> String {
    format!("Index{}D<{}>", num, DI_vals_str(num))
}

pub fn DI_vals_str(num: usize) -> String {
    format!("{}, {}", x_vals_str(num, "D"), &x_vals_str(num, "I"))
}

pub fn const_DI_vals_str(num: usize) -> String {
    format!("{}, {}", const_x_vals_str(num, "D"), const_x_vals_str(num, "I"))
}

pub fn x_mult_str(num: usize, name: &str) -> String {
    let mut output = format!("{}1", name);
    for i in 2..=num {
        output.push_str(&format!("*{}{}", name, i));
    }
    output
}

pub fn x_eq_vals_str(num: usize, name1: &str, name2: &str) -> String {
    let mut output = format!("(({name1}1 == {name2}1) | ({name1}1 == 1) | ({name2}1 == 1)");
    for i in 2..=num {
        output.push_str(
            &format!(" | ({name1}{i} == {name2}{i}) | ({name1}{i} == 1) | ({name2}{i} == 1)"));
    }
    output.push_str(")");
    output
}