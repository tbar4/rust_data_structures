mod data_structures;
use data_structures::sort_array::*;
use data_structures::singly_linked_list::*;
use data_structures::doubly_linked_list::*;

fn main() {
    // Sort Arrays
    let vec = build_rand_vec(50);
    sort_array(vec);

    // Linked List
    let mut trans_log = TransactionLog::new_empty();
    trans_log.append(String::from("First Node"));
    trans_log.append(String::from("Second Node"));
    trans_log.append(String::from("Third Node"));
    trans_log.append(String::from("Fourth Node"));
    trans_log.append(String::from("Fifth Node"));
    println!("Printing full Log:\n\n\n{:#?}", trans_log);
    trans_log.pop();
    println!("Printing popped log: \n\n\n{:#?}", trans_log);
}
