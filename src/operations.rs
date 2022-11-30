const SINGLE_OUTPUT: usize = 0;

pub fn switch_lamp_func(inputs: &[bool]) -> Vec<bool> {
    vec![inputs[0]]
}

pub fn and_func(inputs: &[bool]) -> Vec<bool> {
    if inputs.iter().all(|&x| x) {
        return vec![true];
    }
    vec![false]
}

pub fn or_func(inputs: &[bool]) -> Vec<bool> {
    if inputs.iter().all(|&x| !x) {
        return vec![false];
    }
    vec![true]
}

pub fn not_func(inputs: &[bool]) -> Vec<bool> {
    if inputs.iter().all(|&x| !x) {
        return vec![true];
    }
    vec![false]
}

pub fn nand_func(inputs: &[bool]) -> Vec<bool> {
    vec![!and_func(inputs)[SINGLE_OUTPUT]]
}

pub fn xor_func(inputs: &[bool]) -> Vec<bool> {
    if and_func(inputs)[SINGLE_OUTPUT] || inputs.iter().all(|&x| !x) {
        return vec![false];
    }
    vec![true]
}

// [TODO] remove this function and implement functionality for adding custom gates
pub fn add_func(inputs: &[bool]) -> Vec<bool> {
    vec![
        xor_func(&[inputs[2], xor_func(&[inputs[0], inputs[1]])[0]])[0],
        or_func(&[
            and_func(&[inputs[0], inputs[1]])[0],
            and_func(&[inputs[2], xor_func(&[inputs[0], inputs[1]])[0]])[0],
        ])[0],
    ]
}
