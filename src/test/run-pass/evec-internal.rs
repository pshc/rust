fn main() {
    let x : [int]/5 = [1,2,3,4,5]/5;
    let _y : [int]/5 = [1,2,3,4,5]/_;
    let mut z = [1,2,3,4,5]/_;
    z = x;
    assert z[0] == 1;
    assert z[4] == 5;
}
