use require_lifetimes::require_lifetimes;

#[require_lifetimes(!)]
pub fn example_a<'elided1>(_number: &'elided1 i32) -> (&'elided1 i32, &'elided1 i32) {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_b<'elided1, 'elided2, 'elided3, 'elided4>(_first_arg: &'elided1 i32, _second_arg: &'elided2 i32, _third_arg: &'elided3 Option<&'elided4 i32>) {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_c<'a>(_first_arg: &'a i32, _second_arg: &'a i32) -> &'a i32 {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_d<'a, 'b>(_first_arg: &'a i32, _second_arg: &'b i32) -> &'a i32 {
    unimplemented!()
}
