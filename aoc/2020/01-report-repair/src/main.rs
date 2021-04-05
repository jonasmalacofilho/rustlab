const INPUT: &str = include_str!("input.txt");

fn main() {
    let nums: Vec<u16> = INPUT
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Part One");
    println!("--------");
    'part1: for x in &nums {
        for y in &nums {
            if x + y == 2020 {
                dbg!(x, y);
                println!("{}", *x as u32 * *y as u32);
                break 'part1;
            }
        }
    }

    print!("\n\n");

    println!("Part Two");
    println!("--------");
    'part2: for x in &nums {
        for y in &nums {
            for z in &nums {
                if x + y + z == 2020 {
                    dbg!(x, y, z);
                    println!("{}", (*x as u32) * (*y as u32) * (*z as u32));
                    break 'part2;
                }
            }
        }
    }
}
