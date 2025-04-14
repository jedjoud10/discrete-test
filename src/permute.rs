fn next<const SIZE: usize>(arr: &[u32; SIZE]) -> [u32; SIZE] {
    let mut copy = arr.clone();

    // tried implementing with iterators, but the fact that "windows" is not an iterator in of itself kinda sux

    // find last increasing seq
    let mut index = 0;
    for i in (0..(SIZE-1)).rev()  {
        if copy[i] < copy[i + 1] {
            index = i;
            break;
        }
    }

    // find next largest elem
    // cant use short circuiting since we need the smallest value
    let (swap_index, swap_value) = copy[(index+1)..].iter().enumerate().filter(|(_, x)| **x > copy[index]).min_by(|(_, a), (_, b)| u32::cmp(a, b)).unwrap();

    // swap
    let temp = copy[index];
    copy[index] = *swap_value;
    copy[swap_index] = temp;

    // sorting
    copy[(index+1)..].sort();
    copy
}

pub fn instantiate<const SIZE: usize>(count: usize) -> Vec<[u32; SIZE]> {
    let mut permutations: Vec<[u32; SIZE]> = vec![];

    let iter = (0..SIZE).into_iter().map(|x| x as u32).collect::<Vec<_>>();
    println!("{:?}", iter.as_slice());
    permutations.push(iter.try_into().unwrap());

    for _ in 0..count {
        let new = next(permutations.last().unwrap());
        permutations.push(new);
        println!("{:?}", new);
    }

    permutations
}