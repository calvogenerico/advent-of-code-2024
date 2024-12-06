use std::collections::HashMap;

struct SafetyManual {
    rules: HashMap<usize, Vec<usize>>,
    chapters: Vec<Vec<usize>>,
}

impl SafetyManual {
    pub fn from_str(input: &str) -> SafetyManual {
        let mut rules = HashMap::new();
        let mut chapters: Vec<Vec<usize>> = vec![];

        for line in input.lines() {
            if line.contains('|') {
                let mut parts = line.split("|");
                let key = parts.next().unwrap().parse::<usize>().unwrap();
                let value = parts.next().unwrap().parse::<usize>().unwrap();

                if rules.contains_key(&key) {
                    let vec: &mut Vec<usize> = rules.get_mut(&key).unwrap();
                    vec.push(value);
                } else {
                    rules.insert(key, vec![value]);
                }
            }
            if line.contains(',') {
                let chapter = line
                    .split(",")
                    .map(|chunk| chunk.trim())
                    .map(|chunk| chunk.parse().unwrap())
                    .collect();

                chapters.push(chapter)
            }
        }

        SafetyManual { rules, chapters }
    }

    pub fn count_ordered_pages(&self) -> usize {
        self.chapters
            .iter()
            .filter(|c| self.is_ordered(c))
            .map(|c| self.middle_page(c))
            .sum()
    }

    pub fn count_fixes(&self) -> usize {
        self.chapters
            .iter()
            .filter(|c| !self.is_ordered(c))
            .map(|c| self.fix_page(c))
            .sum()
    }

    fn middle_page(&self, chapter: &[usize]) -> usize {
        chapter[chapter.len() / 2]
    }

    fn is_ordered(&self, chapter: &[usize]) -> bool {
        chapter
            .iter()
            .enumerate()
            .all(|(index, page)| self.item_fulfills_restrictions(*page, &chapter[index + 1..]))
    }

    fn item_fulfills_restrictions(&self, item: usize, rest: &[usize]) -> bool {
        self.first_restriction_not_fulfilled(item, rest).is_none()
    }

    fn first_restriction_not_fulfilled(&self, item: usize, rest: &[usize]) -> Option<usize> {
        rest.iter()
            .enumerate()
            .find(|(_, other)| {
                self.rules
                    .get(other)
                    .map(|restrictions| restrictions.contains(&item))
                    .unwrap_or(false)
            })
            .map(|(index, _)| index)
    }

    fn fix_page(&self, chapter: &[usize]) -> usize {
        let mut current = chapter.to_vec();
        println!("{:?}", &current);
        while !self.is_ordered(&current) {
            let (first_bad_index, n) = current
                .iter()
                .enumerate()
                .find(|(index, page)| !self.item_fulfills_restrictions(**page, &current[*index..]))
                .unwrap();

            let position = self.first_restriction_not_fulfilled(*n, &current[first_bad_index..]).unwrap();

            let position_to_move = first_bad_index + position;

            let mut new_vec = vec![];

            new_vec.extend_from_slice(&current[..first_bad_index]);
            new_vec.push(current[position_to_move]);
            new_vec.extend_from_slice(&current[first_bad_index..position_to_move]);
            new_vec.extend_from_slice(&current[position_to_move + 1..]);
            current = new_vec;
        }
        println!("{:?}", &current);
        self.middle_page(&current)
    }
}

fn step1(input: &str) -> usize {
    let manual = SafetyManual::from_str(input);
    manual.count_ordered_pages()
}

fn step2(input: &str) -> usize {
    let manual = SafetyManual::from_str(input);
    manual.count_fixes()
}

fn main() {
    let input = include_str!("../input.txt");
    let res1 = step1(input);
    let res2 = step2(input);
    println!("step 1: {}", res1);
    println!("step 2: {}", res2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number_with_one_rule_returns_that_number() {
        let input = concat!("47|53\n", "\n", "21,21,21");
        assert_eq!(step1(input), 21);
    }

    #[test]
    fn when_multiple_chapters_match_adds_all_the_middle_numbers() {
        let input = concat!("47|53\n", "\n", "21,21,21\n", "7,7,7,7,7\n",);
        assert_eq!(step1(input), 28);
    }

    #[test]
    fn when_page_contradicts_a_rule_chapter_is_not_considered() {
        let input = concat!("12|1\n", "\n", "1,10,12\n", "11,20,23\n",);
        assert_eq!(step1(input), 20);
    }

    #[test]
    fn step2_a_not_ordered_page_is_fixed_and_then_counted() {
        let input = concat!(
            "12|1\n",
            "\n",
            "1,10,12\n",
            "11,20,23\n",
        );
        assert_eq!(step2(input), 1);
    }
}
