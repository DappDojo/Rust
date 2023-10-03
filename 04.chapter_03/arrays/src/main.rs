fn main() {
    // Arrays in rust have fixed length
    // Data allocated on the stack rather than the heap
    let a = [1, 2, 3, 4, 5];
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    // Array with type declaration
    let a: [i32; 5] = [5, 4, 3, 2, 1];

    // Init array with same values => [3, 3, 3, 3, 3]
    let b = [3; 5];

    let first = a[0];

}
