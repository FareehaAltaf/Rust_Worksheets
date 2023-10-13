/*
    15.Design a sorting library in Rust that provides a trait called Sortable with a
    method sort for sorting arrays or vectors. Implement this trait for various
    data types and sorting algorithms (e.g., Bubble Sort, Quick Sort).
*/
trait Sortable {
    fn bubble_sort(&mut self); // T: Ord trait bounds to ensure that the elements in the vector can be compared
    fn quick_sort(&mut self);
}

impl<T: Ord  + Clone> Sortable for Vec<T>  { 
    fn bubble_sort(&mut self) {
        let n = self.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if self[j] > self[j + 1] {
                    self.swap(j, j + 1);
                }
            }
        }
    }

    fn quick_sort(&mut self){
        let n = self.len();
        if n < 2 {
            return;
        }
        let pivot = self.pop().unwrap(); // pop out the last element, and store as a pivot point
        let mut left = Vec::new();
        let mut right = Vec::new();

        for i in 0..n-1 { // iterate through the vector and compare each element to the pivot point
            if self[i] < pivot {
                left.push(self[i].clone());
            } else {
                right.push(self[i].clone()); // clone(cuz u cannot move out of index of `Vec<T>`) the element and push it to the right vector
            }
        }
        left.quick_sort(); // recursively call quick_sort on the left  ::<T> is used to specify the type of the vector
        right.quick_sort(); // recursively call quick_sort on the right vectors
        self.clear(); // clear the vector
        self.append(&mut left); // append the left vector to the original vector
        self.push(pivot); // append the pivot point to the original vector
        self.append(&mut right); //  append the right vector to the original vector
    }
}

fn main() {
    let mut numbers = vec![4, 2, 8, 6, 1];
    numbers.bubble_sort();
    println!("Sorted numbers: {:?}", &numbers);

    let mut charss = vec!['d', 'a', 'c', 'b'];
    charss.quick_sort();
    println!("Sorted characters: {:?}", charss);
}

