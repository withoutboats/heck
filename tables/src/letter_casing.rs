use std::{any::type_name, collections::hash_map, error::Error, io, mem::size_of};

use bitvec::prelude::*;
use rustc_hash::FxHashMap;

use crate::unicode_data::{set_by_general_category, set_by_prop, CodepointBitArr, DataFiles};

/// Change this to u64 for smaller leaves
type LeafElement = u128;

const ENTRIES_PER_LEAF: usize = LeafElement::BITS as usize / 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CasedLetterKind {
    Lowercase = 1,
    Uppercase = 2,
    Titlecase = 3,
}

pub fn letter_casing(
    data: &DataFiles,
    word_components: &BitSlice,
) -> (Vec<Option<CasedLetterKind>>, Vec<u32>) {
    let mut lowercase = CodepointBitArr::ZERO;
    set_by_general_category(&mut lowercase, data, "Ll", true);
    lowercase &= word_components;
    let mut uppercase = CodepointBitArr::ZERO;
    set_by_general_category(&mut uppercase, data, "Lu", true);
    uppercase &= word_components;
    let mut titlecase = CodepointBitArr::ZERO;
    set_by_general_category(&mut titlecase, data, "Lt", true);
    titlecase &= word_components;

    let last = [
        lowercase.last_one(),
        uppercase.last_one(),
        titlecase.last_one(),
    ]
    .into_iter()
    .max()
    .flatten()
    .unwrap();

    let mut casing_vec = vec![None; last + 1];
    for cp in lowercase.iter_ones() {
        casing_vec[cp] = Some(CasedLetterKind::Lowercase);
    }
    for cp in uppercase.iter_ones() {
        casing_vec[cp] = Some(CasedLetterKind::Uppercase);
    }
    for cp in titlecase.iter_ones() {
        casing_vec[cp] = Some(CasedLetterKind::Titlecase);
    }

    set_by_prop(&mut titlecase, &data.scripts, "Greek", false);

    (
        casing_vec,
        titlecase
            .iter_ones()
            .map(|cp| u32::try_from(cp).unwrap())
            .collect(),
    )
}

fn build_casing_tree(casings_list: &[Option<CasedLetterKind>]) -> (Vec<u8>, Vec<LeafElement>) {
    let mut chunk_to_leaf_idx_map: FxHashMap<LeafElement, u8> = FxHashMap::from_iter([(!0, 0)]);
    let mut root = Vec::with_capacity(casings_list.len().div_ceil(ENTRIES_PER_LEAF));
    let mut leaves = Vec::new();
    let chunks_iter = casings_list.chunks_exact(ENTRIES_PER_LEAF);
    assert!(chunks_iter.remainder().is_empty());
    let mut chunks_iter = chunks_iter.map(|c| {
        let mut chunk_uint: LeafElement = 0;
        for (index, elem) in c.iter().copied().enumerate() {
            let bits = elem.map_or(0, |k| k as u8);
            chunk_uint |= LeafElement::from(bits) << (index * 2);
        }
        chunk_uint
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

pub fn write_table(
    out: &mut impl io::Write,
    data: &DataFiles,
    allowed_in_word: &CodepointBitArr,
) -> Result<(), Box<dyn Error>> {
    let (mut casing_vec, non_greek) = letter_casing(data, allowed_in_word);

    let bits_to_shift = ENTRIES_PER_LEAF.ilog2();

    let first_cp_not_in_tree = casing_vec.len().next_multiple_of(ENTRIES_PER_LEAF);
    for _ in casing_vec.len()..first_cp_not_in_tree {
        casing_vec.push(None);
    }

    let first_cp_not_in_tree_shifted = first_cp_not_in_tree >> bits_to_shift;

    let mut non_greek = non_greek.into_iter();

    write!(
        out,
        "
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CasedLetterKind {{
    Lowercase = 1,
    Uppercase = 2,
    Titlecase = 3,
}}

/// The case of this letter, or `None` if it is not a cased letter.
pub fn letter_casing(c: char) -> Option<CasedLetterKind> {{
    const BOTTOM_BITS_MASK: u32 = !((!0_u32) << {bits_to_shift});
    let cp: u32 = c.into();
    let top_bits = cp >> {bits_to_shift};
    if top_bits < 0x{first_cp_not_in_tree_shifted:X} {{
        let leaf_idx: u8 = LETTER_CASING_ROOT[usize::try_from(top_bits).unwrap()];
        let leaf = LETTER_CASING_LEAVES[usize::from(leaf_idx)];
        match (leaf >> ((cp & BOTTOM_BITS_MASK) * 2)) & 3 {{
            0 => None,
            1 => Some(CasedLetterKind::Lowercase),
            2 => Some(CasedLetterKind::Uppercase),
            3 => Some(CasedLetterKind::Titlecase),
            _ => unreachable!(),
        }}
    }} else {{
        None
    }}
}}

/// Whether the character is a non-Greek titlecase letter.
pub fn is_non_greek_titlecase(c: char) -> bool {{
    matches!(c, '\\u{{{:04X}}}'",
        non_greek.next().unwrap()
    )?;

    for cp in non_greek {
        write!(out, " | '\\u{{{cp:04X}}}'")?;
    }

    writeln!(
        out,
        ")
}}
"
    )?;

    let (root, leaves) = build_casing_tree(&casing_vec);
    eprintln!(
        "letter_casing: {} bytes of static data",
        root.len() + leaves.len() * size_of::<LeafElement>()
    );

    write!(out, "static LETTER_CASING_ROOT: [u8; {}] = [", root.len())?;

    for line in root.chunks(16) {
        write!(out, "\n   ")?;
        for byte in line {
            write!(out, " 0x{byte:02X},")?;
        }
    }

    writeln!(
        out,
        "\n];

static LETTER_CASING_LEAVES: [{}; {}] = [",
        type_name::<LeafElement>(),
        leaves.len()
    )?;

    for leaf in leaves {
        writeln!(out, "    0x{leaf:032X},")?;
    }
    writeln!(out, "];")?;
    Ok(())
}
