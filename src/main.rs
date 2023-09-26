fn main() {
    let mut arr = [0; 100_000];
    for i in 0..100_000 {
        arr[i] = rand::random::<i32>();
    }

    let start = std::time::Instant::now();

    merge_sort::<100_000>(&mut arr);

    let end = std::time::Instant::now();
    let time_taken = end.duration_since(start);

    println!("Time taken to sort {} random numbers: {} ms", 100_000, time_taken.as_millis());
}

fn merge_sort<const N: usize>(arr: &mut [i32; N]) {
    let mut buffer = [0; N];
    let mut width = 1;

    while width < N {
        for i in (0..N).step_by(2 * width) {
            let left = i;
            let right = std::cmp::min(i + width, N);
            let end = std::cmp::min(i + 2 * width, N);

            merge(&arr[left..right], &arr[right..end], &mut buffer[left..end]);
        }

        arr.copy_from_slice(&buffer);
        width *= 2;
    }
}

fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        result[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        result[k] = right[j];
        j += 1;
        k += 1;
    }
}