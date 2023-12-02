advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let line: Vec<&str> = line.split(':').collect();
        let game_id: &str = line[0].split(' ').collect::<Vec<_>>()[1];
        let handfuls: Vec<&str> = line[1].split(';').collect();
        let mut possible = true;

        for handful in handfuls {
            handful.split(',').into_iter().for_each(|color| {
                match color.trim().split_once(' ').unwrap() {
                    (num, "blue") => {
                        if num.parse::<u32>().unwrap() > 14 {
                            possible = false;
                        }
                    }
                    (num, "green") => {
                        if num.parse::<u32>().unwrap() > 13 {
                            possible = false;
                        }
                    }
                    (num, "red") => {
                        if num.parse::<u32>().unwrap() > 12 {
                            possible = false;
                        }
                    }
                    _ => panic!(),
                }
            })
        }
        if possible {
            sum += game_id.parse::<u32>().unwrap();
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let line: Vec<&str> = line.split(':').collect();
        let handfuls: Vec<&str> = line[1].split(';').collect();

        let mut green_min = u32::MIN;
        let mut red_min = u32::MIN;
        let mut blue_min = u32::MIN;

        for handful in handfuls {
            handful.split(',').into_iter().for_each(|color| {
                match color.trim().split_once(' ').unwrap() {
                    (num, "blue") => {
                        if num.parse::<u32>().unwrap() > blue_min {
                            blue_min = num.parse::<u32>().unwrap();
                        }
                    }
                    (num, "green") => {
                        if num.parse::<u32>().unwrap() > green_min {
                            green_min = num.parse::<u32>().unwrap();
                        }
                    }
                    (num, "red") => {
                        if num.parse::<u32>().unwrap() > red_min {
                            red_min = num.parse::<u32>().unwrap();
                        }
                    }
                    _ => panic!(),
                }
            })
        }
        sum += green_min * red_min * blue_min;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
