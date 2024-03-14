use std::{
    any::type_name,
    collections::hash_map,
    error::Error,
    io::{self, Read},
    mem::size_of,
};

use bitvec::prelude::*;
use rustc_hash::FxHashMap;

use crate::unicode_data::{set_by_general_category, CodepointBitArr, DataFiles};

type LeafElement = u128;
const ENTRIES_PER_LEAF: usize = LeafElement::BITS as usize;

fn nonspacing_marks(data: &DataFiles) -> CodepointBitArr {
    let mut arr = BitArray::ZERO;
    set_by_general_category(&mut arr, data, "Mn|Me", true);

    arr
}

fn build_tree(nonspacing_marks: &BitSlice) -> (Vec<u8>, Vec<LeafElement>) {
    let mut chunk_to_leaf_idx_map: FxHashMap<LeafElement, u8> = FxHashMap::from_iter([(0, 0)]);
    let mut root = Vec::with_capacity(nonspacing_marks.len().div_ceil(ENTRIES_PER_LEAF));
    let mut leaves = vec![0];
    let chunks_iter = nonspacing_marks.chunks_exact(ENTRIES_PER_LEAF);
    assert!(chunks_iter.remainder().is_empty());
    let mut chunks_iter = chunks_iter.map(|l| {
        LeafElement::from_le_bytes(
            l.bytes()
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
                .try_into()
                .unwrap(),
        )
    });
    for chunk in &mut chunks_iter {
        match chunk_to_leaf_idx_map.entry(chunk) {
            hash_map::Entry::Occupied(o) => {
                root.push(*o.get());
            }
            hash_map::Entry::Vacant(v) => {
                let new_index = u8::try_from(leaves.len()).expect("too many leaves");
                v.insert(new_index);
                root.push(new_index);
                leaves.push(chunk);
            }
        }
    }
    (root, leaves)
}

fn list_of_ranges(nonspacing_marks: &BitSlice, add: usize) -> Vec<(u32, u32)> {
    let mut vec = Vec::new();
    for i in nonspacing_marks.iter_ones() {
        let cp = u32::try_from(i + add).unwrap();
        if let Some((_, prev)) = vec.last_mut() {
            if *prev + 1 == cp {
                *prev = cp;
                continue;
            }
        }
        vec.push((cp, cp))
    }
    vec
}

pub fn write_table(out: &mut impl io::Write, data: &DataFiles) -> Result<(), Box<dyn Error>> {
    let marks = nonspacing_marks(data);

    let bits_to_shift = ENTRIES_PER_LEAF.ilog2();

    let first_cp_not_in_tree =
        (marks[..0x40000].last_one().unwrap() + 1).next_multiple_of(ENTRIES_PER_LEAF);

    /*for i in 3..10 {
        let entries_per_leaf: usize = 1 << i;
        let bytes_per_leaf = entries_per_leaf / 8;
        let first_cp_not_in_tree =
            (marks[..0x40000].last_one().unwrap() + 1).next_multiple_of(entries_per_leaf);
        let leaves = marks[..first_cp_not_in_tree]
            .chunks_exact(entries_per_leaf)
            .collect::<FxHashSet<_>>();
        dbg!((
            bytes_per_leaf,
            leaves.len(),
            leaves.len() * bytes_per_leaf + first_cp_not_in_tree / entries_per_leaf
        ));
    }*/

    let first_cp_not_in_tree_shifted = first_cp_not_in_tree >> bits_to_shift;

    writeln!(
        out,
        "
/// Whether this character is a nonspacing or enclosing mark.
pub fn is_nonspacing_mark(c: char) -> bool {{
    const BOTTOM_BITS_MASK: u32 = !((!0_u32) << {bits_to_shift});
    let cp: u32 = c.into();
    let top_bits = cp >> {bits_to_shift};
    if top_bits < 0x{first_cp_not_in_tree_shifted:X} {{
        let leaf_idx: u8 = NONSPACING_MARKS_ROOT[usize::try_from(top_bits).unwrap()];
        let leaf = NONSPACING_MARKS_LEAVES[usize::from(leaf_idx)];
        (leaf >> (cp & BOTTOM_BITS_MASK)) & 1 == 1
    }} else {{"
    )?;

    let mut late_marks =
        list_of_ranges(&marks[first_cp_not_in_tree..], first_cp_not_in_tree).into_iter();

    if let Some(first_late_mark) = late_marks.next() {
        write!(
            out,
            "        matches!(cp, 0x{:06X}..=0x{:06X}",
            first_late_mark.0, first_late_mark.1
        )?;
        for late_mark in late_marks {
            write!(out, " | 0x{:06X}..=0x{:06X}", late_mark.0, late_mark.1)?;
        }
        writeln!(out, ")")?;
    } else {
        writeln!(out, "false")?;
    }

    writeln!(
        out,
        "    }}
}}
",
    )?;

    let (root, leaves) = build_tree(&marks[..first_cp_not_in_tree]);
    eprintln!(
        "nonspacing_marks: {} bytes of static data",
        root.len() + leaves.len() * size_of::<LeafElement>()
    );

    write!(
        out,
        "static NONSPACING_MARKS_ROOT: [u8; {}] = [",
        root.len()
    )?;

    for line in root.chunks(16) {
        write!(out, "\n   ")?;
        for byte in line {
            write!(out, " 0x{byte:02X},")?;
        }
    }

    writeln!(
        out,
        "\n];

static NONSPACING_MARKS_LEAVES: [{}; {}] = [",
        type_name::<LeafElement>(),
        leaves.len()
    )?;

    for leaf in leaves {
        writeln!(out, "    0x{leaf:032X},")?;
    }
    writeln!(out, "];")?;

    Ok(())
}
