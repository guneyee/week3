struct FilterCondition<T> {
    value: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.value == item
    }
}

fn custom_filter<T>(collection: Vec<T>, condition: &FilterCondition<T>) -> Vec<T>
where
    T: Clone,
{
    let mut result = Vec::new();

    for item in collection {
        if condition.is_match(&item) {
            result.push(item.clone());
        }
    }

    result
}

fn main() {
    // Create a collection of integers
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Create a FilterCondition that matches the value 5
    let condition = FilterCondition { value: 5 };

    // Filter the collection based on the condition
    let filtered_numbers = custom_filter(numbers.clone(), &condition);

    // Print the filtered result
    println!("Filtered numbers: {:?}", filtered_numbers);
}
