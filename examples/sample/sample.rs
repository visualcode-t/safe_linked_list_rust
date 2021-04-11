use safe_linked_list_rust::LinkedList;

fn main() {
    //Create a new LinkedList of i32 values.
    let mut list = LinkedList::<i32>::new();
    //Add multiple values to the list:
    for i in 1..10 {
        list.add(i);
    }
    //Print the head (1) by chaining next and prev together after retrieving the head element.
    println!("Chaining..");
    let first = list.head();
    println!("Value:{}",first.next().next().next().prev().prev().prev().value);
    //iterate forward through the list.
    println!("Forward..");
    for i in list.iter() {
        if list.is_head(&i) {
            println!("Head:{}", i.value); //Display the head value
        }else if list.is_tail(&i) {
            println!("Tail:{}",i.value); //Display the tail value.
        }
        i.mutate(4 * i.value); //multiply the value by 4 and set it using mutate.
    }
    //iterate backward through the list.
    println!("Backward..");
    for i in list.iter().rev() {
        if list.is_head(&i) {
            println!("Head:{}", i.value); //Display the head value
        }else if list.is_tail(&i) {
            println!("Tail:{}",i.value); //Display the tail value.
        }
    }
}