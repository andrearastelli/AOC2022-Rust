use std::collections::HashSet;

fn main() {
    let source_strings = std::fs::read_to_string("./day3/day032.txt").unwrap();

    let lowercase_a = 'a' as i32;
    let uppercase_a = 'A' as i32;

    let mut tot_priorities = 0;

    for line in source_strings.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let set1: HashSet<char> = HashSet::from_iter(first.chars());
        let set2: HashSet<char> = HashSet::from_iter(second.chars());
        for item in set1.intersection(&set2) {
            let to_dec = if item.is_lowercase() {
                *item as i32 - lowercase_a + 1
            } else {
                *item as i32 - uppercase_a + 27
            };
            tot_priorities += to_dec;
        }
    }

    println!("Part1: Tot priorities: {tot_priorities}");

    let lines = source_strings.lines().collect::<Vec<_>>();
    let mut tot_priorities = 0;
    for groups in lines.chunks(3) {
        let (first, second) = groups.split_at(1);
        let mut common_item: HashSet<char> = HashSet::from_iter(first[0].chars());
        for &group in second {
            let tmp_set: HashSet<char> = HashSet::from_iter(group.chars());
            common_item.retain(|v| tmp_set.contains(v));
        }

        for item in common_item.iter() {
            let mut to_dec: i32 = *item as i32;
            if item.is_lowercase() {
                to_dec += -lowercase_a + 1;
            } else {
                to_dec += -uppercase_a + 27;
            }
            tot_priorities += to_dec;
        }
    }

    println!("Part2: Tot priorities: {tot_priorities}");
}
/*

--- Day 3: Rucksack Reorganization ---

One Elf has the important job of loading all of the rucksacks with supplies for
the jungle journey. Unfortunately, that Elf didn't quite follow the packing
instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant
to go into exactly one of the two compartments. The Elf that did the packing
failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your
puzzle input), but they need your help finding the errors. Every item type is
identified by a single lowercase or uppercase letter (that is, a and A refer to
different types of items).

The list of items for each rucksack is given as characters all on a single
line. A given rucksack always has the same number of items in each of its two
compartments, so the first half of the characters represent items in the first
compartment, while the second half of the characters represent items in the
second compartment.

For example, suppose you have the following list of contents from six
rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its
first compartment contains the items vJrwpWtwJgWr, while the second compartment
contains the items hcsFMMfFFhFp.
The only item type that appears in both compartments is lowercase p.
The second rucksack's compartments contain jqHRNqRjqzjGDLGL and
rsFMfFZSrLrFZsSL.
The only item type that appears in both compartments is uppercase L.
The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only
common item type is uppercase P.
The fourth rucksack's compartments only share item type v.
The fifth rucksack's compartments only share item type t.
The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a
priority:
Lowercase item types a through z have priorities 1 through 26.
Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in both
compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19
(s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is
the sum of the priorities of those item types?

 */
