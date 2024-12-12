use std::fmt::{Debug, Display, Formatter};

use crate::TaskCompleter;

pub struct Task9;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum DiskSpace {
    Number(i64),
    Free,
}

#[derive(Clone, Copy)]
struct DiskPartition {
    filled_with: DiskSpace,
    length: usize,
}

impl Display for DiskPartition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.char().repeat(self.length))
    }
}

impl Debug for DiskPartition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.char().repeat(self.length))
    }
}

impl DiskPartition {
    fn is_free(&self) -> bool {
        self.filled_with == DiskSpace::Free
    }

    fn has_space(&self, space_required: usize) -> bool {
        self.filled_with == DiskSpace::Free && self.length >= space_required
    }

    fn value(&self) -> i64 {
        match self.filled_with {
            DiskSpace::Number(i) => i,
            DiskSpace::Free => 0,
        }
    }

    fn char(&self) -> String {
        match self.filled_with {
            DiskSpace::Number(i) => i.to_string(),
            DiskSpace::Free => ".".to_string(),
        }
    }
}

impl TaskCompleter for Task9 {
    fn do_task_1(&self) -> String {
        let mut disk = vec![];
        let mut use_index = true;
        let mut index = 0;
        for c in include_str!("../input/day_09/input").chars() {
            let v;
            if use_index {
                v = vec![DiskSpace::Number(index); c.to_string().parse::<usize>().unwrap()];
                index += 1;
            } else {
                v = vec![DiskSpace::Free; c.to_string().parse::<usize>().unwrap()];
            }
            disk.push(v);
            use_index = !use_index;
        }
        let mut disk: Vec<DiskSpace> = disk.into_iter().flatten().collect();
        let mut i = 0;
        let mut j = disk.len() - 1;
        while i < j {
            if disk[j] == DiskSpace::Free {
                j -= 1;
            } else if disk[i] == DiskSpace::Free {
                disk[i] = disk[j];
                disk[j] = DiskSpace::Free;
                i += 1;
            } else {
                i += 1;
            }
        }
        disk.iter()
            .enumerate()
            .map(|(i, x)| match x {
                DiskSpace::Number(j) => i as i64 * j,
                DiskSpace::Free => 0,
            })
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let mut disk = vec![];
        let mut use_index = true;
        let mut index = 0;
        for c in include_str!("../input/day_09/input").chars() {
            let filled_with;
            let length = c.to_string().parse::<usize>().unwrap();
            if length > 0 {
                if use_index {
                    filled_with = DiskSpace::Number(index);
                    index += 1;
                } else {
                    filled_with = DiskSpace::Free;
                }
                disk.push(DiskPartition {
                    filled_with,
                    length,
                });
            }
            use_index = !use_index;
        } // Ensure no consecutive free spaces
        let mut i = 1;
        while i < disk.len() {
            if disk[i].is_free() && disk[i - 1].is_free() {
                disk[i - 1].length += disk[i].length;
                disk.remove(i);
            } else {
                i += 1;
            }
        }
        let mut earliest_free_space = 0;
        let mut item_to_be_moved = disk.len() - 1;
        while earliest_free_space < item_to_be_moved {
            if !disk[earliest_free_space].is_free() {
                earliest_free_space += 1;
            } else if item_to_be_moved >= disk.len() || disk[item_to_be_moved].is_free() {
                item_to_be_moved -= 1;
            } else {
                // println!("{:?}", disk);
                // println!("Trying to move: {}", disk[item_to_be_moved]);
                assert!(disk[earliest_free_space].is_free());
                assert!(!disk[item_to_be_moved].is_free());
                let space_required = disk[item_to_be_moved].length;
                let mut i = earliest_free_space;
                while !disk[i].has_space(space_required) && i < item_to_be_moved {
                    i += 1;
                }
                if i >= item_to_be_moved {
                    // Skip this index and don't move anything else, no disk changes occur
                    item_to_be_moved -= 1;
                    continue;
                } else {
                    if disk[i].length == space_required {
                        disk[i].filled_with = disk[item_to_be_moved].filled_with;
                    } else {
                        disk.insert(i, disk[item_to_be_moved]);
                        i += 1;
                        disk[i].length -= space_required;
                        item_to_be_moved += 1;
                    }
                    disk[item_to_be_moved].filled_with = DiskSpace::Free;
                    // Clean up at the top end.
                    if item_to_be_moved + 1 != disk.len() && disk[item_to_be_moved + 1].is_free() {
                        disk[item_to_be_moved].length += disk[item_to_be_moved + 1].length;
                        disk.remove(item_to_be_moved + 1);
                    }
                    if disk[item_to_be_moved - 1].is_free() {
                        disk[item_to_be_moved - 1].length += disk[item_to_be_moved].length;
                        disk.remove(item_to_be_moved);
                        item_to_be_moved -= 1;
                    }
                }
            }
        }
        let mut s = 0;
        let mut i = 0;
        for d in disk {
            if !d.is_free() {
                s += d.value() * (i..(i + d.length)).into_iter().sum::<usize>() as i64;
            }
            i += d.length;
        }
        s.to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("6291146824486".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
