pub fn and_func(inputs: &[bool]) -> bool {
    for i in inputs {
        if !i {
            return false;
        }
    }
    true
}
