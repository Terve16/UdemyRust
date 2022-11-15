// Arrays: Fixed length, only same datatype

fn main() {

    let _array = [1, 2, 3];

    println!("{:?}", _array);

    println!("{:#?}", _array);

    println!("{}", _array[0]);
    println!("{}", _array[1]);
    println!("{}", _array[2]);

    let _array2: [u32; 10] = [1337; 10];

    println!("{:#?}", _array2);

    // println!("{}", _array2[15]);
}
