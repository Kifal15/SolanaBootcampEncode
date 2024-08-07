//Lesson 2 rust example 

/*fn main() {
    let mut array = [1, 2, 3, 4];
    let reversed_array = inplace(&mut array);
    println!("{:?}", reversed_array);
}

fn inplace(arr: &mut [i32]) -> &[i32] {
    arr.reverse();
    arr // Return the modified array as a slice
}
*/


fn main(){

    let mut array = [ 1,2,3,4];
     inplace(&mut array);
println!("{:?}",array);


let outplacerev = outplace(&array);
println!("{:?}",outplacerev)
}


fn inplace(arr : &mut [i32]){
    arr.reverse()
}
