pub fn part_one(input: &str) -> Option<u32> {
    let elf_meals = input.split("\n\n").filter(|lines| !lines.is_empty());
    let mut calory_sum = Vec::<u32>::new();

    for elf_meal in elf_meals {
        let calory_list = elf_meal.split("\n").filter(|s| !s.is_empty());
        let sum = calory_list
            .map(|calory| calory.parse::<u32>().unwrap())
            .sum();
        calory_sum.push(sum);
    }

    Some(calory_sum.iter().max().unwrap().clone())
}

pub fn part_two(input: &str) -> Option<u32> {
    let elf_meals = input.split("\n\n").filter(|lines| !lines.is_empty());
    let mut calory_sum = Vec::<u32>::new();

    for elf_meal in elf_meals {
        let calory_list = elf_meal.split("\n").filter(|s| !s.is_empty());
        let sum = calory_list
            .map(|calory| calory.parse::<u32>().unwrap())
            .sum();
        calory_sum.push(sum);
    }

    calory_sum.sort();
    calory_sum.reverse();
    Some(calory_sum.iter().take(3).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
