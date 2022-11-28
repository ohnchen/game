pub fn switch_lamp_func(inputs: u64, _: usize) -> bool {
    inputs != 0
}

pub fn and_func(inputs: u64, num_of_inputs: usize) -> bool {
    let mut temp = 1;
    let inputs_bin = format!("{:b}", inputs);

    while temp <= inputs {
        temp *= 2;
    }

    if inputs_bin.len() < num_of_inputs {
        return false;
    }

    if inputs != 0 && inputs & (temp - 1) == (temp - 1) {
        return true;
    }

    false
}

pub fn or_func(inputs: u64, _: usize) -> bool {
    if inputs == 0 {
        return false;
    }
    true
}

pub fn not_func(inputs: u64, _: usize) -> bool {
    if inputs == 0 {
        return true;
    }
    false
}
