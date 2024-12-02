use std::{fs::File, io::Read as _};

fn part_one(list_one: Vec<i32>, list_two: Vec<i32>) {
    let mut final_list: Vec<i32> = Vec::new();

    let mut i = 0;

    for num1 in &list_one {
        if num1 > &list_two[i] {
            final_list.push(num1 - list_two[i])
        } else {
            final_list.push(list_two[i] - num1)
        }
        i += 1;
    }

    let mut total = 0;

    for num in final_list {
        total += num;
    }

    println!("Distance: {}", total);
}

fn part_two(list_one: Vec<i32>, list_two: Vec<i32>) {
    let mut score = 0;

    for num in list_one {
        let hits: Vec<i32> = list_two
            .iter()
            .filter_map(|x| if *x == num { Some(*x) } else { None })
            .collect();

        let contribution = num as usize * hits.len();
        score += contribution;
    }

    println!("Similarity Score: {}", score);
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();

    f.read_to_string(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .lines()
        .filter_map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            if nums.is_empty() {
                None
            } else {
                Some(nums)
            }
        })
        .flatten()
        .collect();

    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    let mut i = 1;
    for item in numbers {
        if i % 2 == 0 {
            list_two.push(item);
        } else {
            list_one.push(item);
        }
        i += 1;
    }

    list_one.sort();
    list_two.sort();

    part_one(list_one.clone(), list_two.clone());
    part_two(list_one, list_two);
}
