fn main() {
    let array: [u32; 3] = [1u32, 2, 3];
    let first_element = array[0];
    let element = array[100] // you get a warning for trying to access an out of bounds element

    let len = "Some text".len();
    [1, 2][len]; // you get a panic which crashes the program as the compiler doesn't know the element you are trying to access is out of bounds

    let tuple: (u32, u8, bool) = (1u32, 2, true);
    let first_el: u32 = tuple.0;
    
}
