pub fn paginate<T: Clone>(data: Vec<T>, limit: usize, offset: usize) -> Vec<T> {
    if limit == 0 {
        return data;
    }

    let mut new_data = Vec::<T>::new();

    let start = limit * offset;
    let end = start + limit;
    let length = data.len();

    for n in start..end {
        if n >= length {
            break;
        }

        new_data.push(data[n].clone());
    }

    new_data
}
