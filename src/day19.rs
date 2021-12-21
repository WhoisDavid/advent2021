use std::{collections::VecDeque, ops::Index};

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet;
use itertools::Itertools;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Beacon(i32, i32, i32);

impl Index<usize> for Beacon {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("dim == 3"),
        }
    }
}

// Simple hash function - speeds up the HashSet by 30-40%
impl std::hash::Hash for Beacon {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.0 * 31 + self.1 * 37 + self.2 * 41);
    }
}

type Scanner = Vec<Beacon>;

#[aoc_generator(day19)]
pub fn input_parser(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(|l| {
                    let mut d = l.splitn(3, ',');
                    Beacon(
                        d.next().unwrap().parse().ok().unwrap(),
                        d.next().unwrap().parse().ok().unwrap(),
                        d.next().unwrap().parse().ok().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

const PERM: [(usize, usize, usize); 6] = [
    (0, 1, 2),
    (0, 2, 1),
    (2, 0, 1),
    (2, 1, 0),
    (1, 0, 2),
    (1, 2, 0),
];

pub fn match_beacons(beacons: &mut HashSet<Beacon>, s2: &Scanner) -> Option<Beacon> {
    // for (dirx, diry, dirz) in DIRS.into_iter().rev() {
    for dirx in [-1, 1] {
        for diry in [-1, 1] {
            for dirz in [-1, 1] {
                for Beacon(ax, ay, az) in beacons.iter() {
                    for beacon_b in s2 {
                        for (x, y, z) in PERM {
                            let scanner_coords = Beacon(
                                dirx * beacon_b[x] - ax,
                                diry * beacon_b[y] - ay,
                                dirz * beacon_b[z] - az,
                            );
                            let oriented_s2 = s2.iter().map(|b| {
                                Beacon(
                                    dirx * b[x] - scanner_coords.0,
                                    diry * b[y] - scanner_coords.1,
                                    dirz * b[z] - scanner_coords.2,
                                )
                            });

                            let matching_beacons =
                                oriented_s2.clone().filter(|b| beacons.contains(&b)).count();

                            if matching_beacons >= 12 {
                                beacons.extend(oriented_s2);
                                return Some(scanner_coords);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

#[aoc(day19, part1)]
pub fn part1(input: &[Scanner]) -> usize {
    let mut input = VecDeque::from(input.to_vec());
    let mut beacons = input
        .pop_front()
        .unwrap()
        .into_iter()
        .collect::<HashSet<_>>();
    while let Some(beacon) = input.pop_front() {
        if match_beacons(&mut beacons, &beacon).is_none() {
            input.push_back(beacon)
        };
    }
    beacons.len()
}

#[aoc(day19, part2)]
pub fn part2(input: &[Scanner]) -> i32 {
    let mut input = VecDeque::from(input.to_vec());
    let mut beacons = input
        .pop_front()
        .unwrap()
        .into_iter()
        .collect::<HashSet<_>>();
    let mut scanners = vec![Beacon(0, 0, 0)];
    while let Some(beacon) = input.pop_front() {
        if let Some(scanner) = match_beacons(&mut beacons, &beacon) {
            scanners.push(scanner)
        } else {
            input.push_back(beacon)
        }
    }

    scanners
        .iter()
        .combinations(2)
        .map(|c| (0..3).fold(0, |s, i| s + (c[0][i] - c[1][i]).abs()))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test_day19 {
    use super::*;

    const TESTCASE: &str = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 79)
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&input_parser(TESTCASE)), 3621)
    // }
}
