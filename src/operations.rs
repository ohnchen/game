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

pub fn add_func(_inputs: &[bool]) -> Vec<bool> {
    vec![false, true]
}
