fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut smallest_vec;
    let mut largest_vec;
    if nums1.len().max(nums2.len()) == nums1.len() {
        largest_vec = nums1;
        smallest_vec = nums2;
    } else {
        largest_vec = nums2;
        smallest_vec = nums1;
    }

    if largest_vec.is_empty() && smallest_vec.is_empty() {
        return 0.0;
    }

    let starting_length = largest_vec.len();

    let mut latest_inserted;

    let mut pending_push: Option<i32> = None;

    for num in &smallest_vec {
        if pending_push.is_some() {
            largest_vec.push(pending_push.unwrap());
            pending_push = None;
        }

        for (ii, num_lst) in largest_vec.iter().enumerate() {
            latest_inserted = *largest_vec.last().unwrap();

            if *num <= *num_lst {
                if ii == 0 {
                    largest_vec.insert(0, *num);
                } else {
                    largest_vec.insert(ii, *num);
                }
                break;
            } else if latest_inserted < *num {
                pending_push = Some(*num);
            }
        }
    }

    if (largest_vec.last().is_some() && smallest_vec.last().is_some())
        && (largest_vec.last().unwrap() < smallest_vec.last().unwrap())
    {
        largest_vec.push(*smallest_vec.last().unwrap());
    }

    if largest_vec.len() == starting_length {
        largest_vec.append(&mut smallest_vec);
    }

    println!("{:?}", largest_vec);

    let half: i32 = largest_vec.len() as i32 / 2;
    if largest_vec.len() % 2 != 0 {
        largest_vec[half as usize] as f64
    } else {
        let m_pos = largest_vec.len() / 2;
        (largest_vec[m_pos] + largest_vec[m_pos - 1]) as f64 / 2.0
    }
}

fn main() {
    let nums1 = vec![2, 2, 4, 4];
    let nums2 = vec![2, 2, 2, 4, 4];

    let median = find_median_sorted_arrays(nums1, nums2);

    println!("median: {}", median);
}
