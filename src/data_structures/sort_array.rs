use rand;

pub fn build_rand_vec(num_elements: usize) -> Vec<usize> {
    let mut vec = Vec::with_capacity(num_elements);
    for _ in 0..vec.capacity() {
        vec.push(rand::random());
    };

    vec
}

pub fn sort_array(mut arr: Vec<usize>) -> () {
    for i in 0..arr.len() {
        let mut swaps = 0;
        for i in 1..arr.len() {
            if arr[i-1] > arr[i] {
                arr.swap(i - 1, i);
                swaps += 1;
            }
        }
        if swaps == 0{
            break;
        }
    }
    println!("{:#?}", arr);
}
