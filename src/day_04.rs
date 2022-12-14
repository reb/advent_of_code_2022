/// --- Day 4: Camp Cleanup ---
///
/// Space needs to be cleared before the last supplies can be unloaded from the ships, and so
/// several Elves have been assigned the job of cleaning up sections of the camp. Every section has
/// a unique ID number, and each Elf is assigned a range of section IDs.
///
/// However, as some of the Elves compare their section assignments with each other, they've noticed
/// that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated
/// effort, the Elves pair up and make a big list of the section assignments for each pair (your
/// puzzle input).
///
/// For example, consider the following list of section assignment pairs:
///
/// 2-4,6-8
/// 2-3,4-5
/// 5-7,7-9
/// 2-8,3-7
/// 6-6,4-6
/// 2-6,4-8
///
/// For the first few pairs, this list means:
///
///     Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and
///     4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
///     The Elves in the second pair were each assigned two sections.
///     The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7,
///     while the other also got 7, plus 8 and 9.
///
/// This example list uses single-digit section IDs to make it easier to draw; your actual list
/// might contain larger numbers. Visually, these pairs of section assignments look like this:
///
/// .234.....  2-4
/// .....678.  6-8
///
/// .23......  2-3
/// ...45....  4-5
///
/// ....567..  5-7
/// ......789  7-9
///
/// .2345678.  2-8
/// ..34567..  3-7
///
/// .....6...  6-6
/// ...456...  4-6
///
/// .23456...  2-6
/// ...45678.  4-8
///
/// Some of the pairs have noticed that one of their assignments fully contains the other. For
/// example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one
/// assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections
/// their partner will already be cleaning, so these seem like the most in need of reconsideration.
/// In this example, there are 2 such pairs.
///
/// In how many assignment pairs does one range fully contain the other?
///
/// --- Part Two ---
///
/// It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would
/// like to know the number of pairs that overlap at all.
///
/// In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the
/// remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:
///
///     5-7,7-9 overlaps in a single section, 7.
///     2-8,3-7 overlaps all of the sections 3 through 7.
///     6-6,4-6 overlaps in a single section, 6.
///     2-6,4-8 overlaps in sections 4, 5, and 6.
///
/// So, in this example, the number of overlapping assignment pairs is 4.
///
/// In how many assignment pairs do the ranges overlap?
use itertools::Itertools;

const INPUT: &str = include_str!("../input/day_04");

pub fn run() {
    let assignments = load_assignments(INPUT);

    let fully_contained_pairs = assignments.iter().filter(fully_overlaps).count();
    println!(
        "The amount of assignment pairs that fully contain the other is: {}",
        fully_contained_pairs
    );

    let partially_contained_pairs = assignments.iter().filter(partially_overlaps).count();
    println!(
        "The amount of assignment pairs that fully contain the other is: {}",
        partially_contained_pairs
    );
}

#[derive(Debug, PartialEq)]
struct Assignment {
    begin: u32,
    end: u32,
}

impl Assignment {
    fn new((begin, end): (u32, u32)) -> Assignment {
        Assignment { begin, end }
    }
    fn covers(&self, t: u32) -> bool {
        self.begin <= t && self.end >= t
    }
    fn contains(&self, other: &Assignment) -> bool {
        other.begin >= self.begin && other.end <= self.end
    }
}

fn load_assignments(input: &str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .take(2)
                .map(convert_to_assignment)
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn convert_to_assignment(assignment: &str) -> Assignment {
    Assignment::new(
        assignment
            .split('-')
            .take(2)
            .map(str::parse)
            .filter_map(Result::ok)
            .collect_tuple()
            .unwrap(),
    )
}

fn fully_overlaps((a, b): &&(Assignment, Assignment)) -> bool {
    a.contains(&b) || b.contains(&a)
}

fn partially_overlaps((a, b): &&(Assignment, Assignment)) -> bool {
    a.covers(b.begin) || a.covers(b.end) || b.covers(a.begin) || b.covers(a.end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_assignments() {
        let input = "2-4,6-8";
        let expected = vec![(Assignment::new((2, 4)), Assignment::new((6, 8)))];

        assert_eq!(load_assignments(input), expected);
    }

    #[test]
    fn test_convert_to_assignment() {
        let input = "2-4";

        assert_eq!(convert_to_assignment(input), Assignment::new((2, 4)));
    }

    #[test]
    fn test_fully_overlaps_1() {
        // 2-4,6-8
        let input = (Assignment::new((2, 4)), Assignment::new((6, 8)));

        assert!(!fully_overlaps(&&input))
    }

    #[test]
    fn test_fully_overlaps_2() {
        // 2-3,4-5
        let input = (Assignment::new((2, 3)), Assignment::new((4, 5)));

        assert!(!fully_overlaps(&&input))
    }

    #[test]
    fn test_fully_overlaps_3() {
        // 5-7,7-9
        let input = (Assignment::new((5, 7)), Assignment::new((7, 9)));

        assert!(!fully_overlaps(&&input))
    }

    #[test]
    fn test_fully_overlaps_4() {
        // 2-8,3-7
        let input = (Assignment::new((2, 8)), Assignment::new((3, 7)));

        assert!(fully_overlaps(&&input))
    }

    #[test]
    fn test_fully_overlaps_5() {
        // 6-6,4-6
        let input = (Assignment::new((6, 6)), Assignment::new((4, 6)));

        assert!(fully_overlaps(&&input))
    }

    #[test]
    fn test_fully_overlaps_6() {
        // 2-6,4-8
        let input = (Assignment::new((2, 6)), Assignment::new((4, 8)));

        assert!(!fully_overlaps(&&input))
    }

    #[test]
    fn test_partially_overlaps_1() {
        // 2-4,6-8
        let input = (Assignment::new((2, 4)), Assignment::new((6, 8)));

        assert!(!partially_overlaps(&&input))
    }

    #[test]
    fn test_partially_overlaps_2() {
        // 2-3,4-5
        let input = (Assignment::new((2, 3)), Assignment::new((4, 5)));

        assert!(!partially_overlaps(&&input))
    }

    #[test]
    fn test_partially_overlaps_3() {
        // 5-7,7-9
        let input = (Assignment::new((5, 7)), Assignment::new((7, 9)));

        assert!(partially_overlaps(&&input))
    }

    #[test]
    fn test_partially_overlaps_4() {
        // 2-8,3-7
        let input = (Assignment::new((2, 8)), Assignment::new((3, 7)));

        assert!(partially_overlaps(&&input))
    }

    #[test]
    fn test_partially_overlaps_5() {
        // 6-6,4-6
        let input = (Assignment::new((6, 6)), Assignment::new((4, 6)));

        assert!(partially_overlaps(&&input))
    }

    #[test]
    fn test_partially_overlaps_6() {
        // 2-6,4-8
        let input = (Assignment::new((2, 6)), Assignment::new((4, 8)));

        assert!(partially_overlaps(&&input))
    }
}
