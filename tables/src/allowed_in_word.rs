//! Construct a lookup table to find whether a particular character is allowed in words.

use std::{
    any::type_name,
    collections::hash_map,
    error::Error,
    io::{self, Read},
    mem::size_of,
};

use bitvec::prelude::*;
use rustc_hash::FxHashMap;

use crate::unicode_data::{set_by_general_category, set_by_prop, CodepointBitArr, DataFiles};

/// Change this to u128 for wider leaves
type LeafElement = u64;

const ENTRIES_PER_LEAF: usize = LeafElement::BITS as usize;

/// `true` for all punctuation other than `Other_Punctuation`
/// (`[\p{Punctuation}-\p{Other_Punctuation}]`)
fn punctuation_non_other(data: &DataFiles) -> CodepointBitArr {
    let mut arr = BitArray::ZERO;
    set_by_general_category(&mut arr, data, "Pc|Pd|Ps|Pe|Pi|Pf", true);
    arr
}

/// `true` for all unassigned and private use characters
fn unassigned_private_use(data: &DataFiles) -> CodepointBitArr {
    let mut arr = BitArray::ZERO;
    set_by_general_category(&mut arr, data, "[A-Za-z]+", true);
    set_by_general_category(&mut arr, data, "Cn|Co", false);
    !arr
}

/// `true` for all codepoints that can be part of a word:
/// `[\p{ID_Continue}\p{ID_Compat_Math_Continue}\p{Cn}\p{Co}\p{Alphabetic}\p{N}-[\p{P}-\p{Po}]]`,
/// plus the extra characters listed below.
pub fn allowed_in_word(data: &DataFiles) -> CodepointBitArr {
    let mut word_component = unassigned_private_use(data);

    set_by_prop(
        &mut word_component,
        &data.derived_core_properties,
        "ID_Continue|Alphabetic",
        true,
    );

    set_by_general_category(&mut word_component, data, "Nd|Nl|No", true);

    set_by_prop(
        &mut word_component,
        &data.prop_list,
        "ID_Compat_Math_Continue",
        true,
    );

    // Choose from characters in https://www.unicode.org/reports/tr31/#Specific_Character_Adjustments
    // that are not Punctuation other than Other_Punctuation
    // (U+00B7 is already in ID_Continue).
    for cp in [
        0x05F3, // HEBREW PUNCTUATION GERESH https://en.wikipedia.org/wiki/Geresh
        0x05F4, // HEBREW PUNCTUATION GERSHAYIM https://en.wikipedia.org/wiki/Gershayim
        0x0F0B, // TIBETAN MARK INTERSYLLABIC TSHEG https://w3c.github.io/tlreq/#language_overview
    ] {
        word_component.set(cp, true);
    }

    word_component &= !punctuation_non_other(data);

    word_component
}

fn build_tree(allowed_in_word: &BitSlice) -> (Vec<u8>, Vec<LeafElement>) {
    let mut chunk_to_leaf_idx_map: FxHashMap<LeafElement, u8> = FxHashMap::from_iter([(!0, 0)]);
    let mut root = Vec::with_capacity(allowed_in_word.len().div_ceil(ENTRIES_PER_LEAF));
    let mut leaves = vec![!0];
    let chunks_iter = allowed_in_word.chunks_exact(ENTRIES_PER_LEAF);
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

fn list_of_ranges(cps: impl Iterator<Item = u32>) -> Vec<(u32, u32)> {
    let mut vec = Vec::new();
    for cp in cps {
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

pub fn write_table(
    out: &mut impl io::Write,
    allowed_in_word: &CodepointBitArr,
) -> Result<(), Box<dyn Error>> {
    let bits_to_shift = ENTRIES_PER_LEAF.ilog2();

    let first_cp_not_in_tree =
        (allowed_in_word[..0x40000].last_zero().unwrap() + 1).next_multiple_of(ENTRIES_PER_LEAF);

    let first_cp_not_in_tree_shifted = first_cp_not_in_tree >> bits_to_shift;

    writeln!(
        out,
        "/// Whether this character can be part of a word.
pub fn allowed_in_word(c: char) -> bool {{
    const BOTTOM_BITS_MASK: u32 = !((!0_u32) << {bits_to_shift});
    let cp: u32 = c.into();
    let top_bits = cp >> {bits_to_shift};
    if top_bits < 0x{first_cp_not_in_tree_shifted:X} {{
        let leaf_idx: u8 = ALLOWED_IN_WORD_ROOT[usize::try_from(top_bits).unwrap()];
        let leaf = ALLOWED_IN_WORD_LEAVES[usize::from(leaf_idx)];
        (leaf >> (cp & BOTTOM_BITS_MASK)) & 1 == 1
    }} else {{"
    )?;

    let mut late_zeros = list_of_ranges(
        allowed_in_word[first_cp_not_in_tree..]
            .iter_zeros()
            .map(|n| u32::try_from(n + first_cp_not_in_tree).unwrap()),
    )
    .into_iter();
    if let Some(first_late_zero) = late_zeros.next() {
        write!(out, "        !matches!(cp, 0x{:06X}", first_late_zero.0)?;
        if first_late_zero.0 != first_late_zero.1 {
            write!(out, "..=0x{:06X}", first_late_zero.1)?;
        }
        for late_zero in late_zeros {
            write!(out, " | 0x{:06X}", late_zero.0)?;
            if late_zero.0 != late_zero.1 {
                write!(out, "..=0x{:06X}", late_zero.1)?;
            }
        }
        writeln!(out, ")")?;
    } else {
        writeln!(out, "true")?;
    }

    writeln!(
        out,
        "    }}
}}",
    )?;

    let (root, leaves) = build_tree(&allowed_in_word[..first_cp_not_in_tree]);
    eprintln!(
        "allowed_in_words: {} bytes of static data",
        root.len() + leaves.len() * size_of::<LeafElement>()
    );

    write!(
        out,
        "\nstatic ALLOWED_IN_WORD_ROOT: [u8; {}] = [",
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

static ALLOWED_IN_WORD_LEAVES: [{}; {}] = [",
        type_name::<LeafElement>(),
        leaves.len()
    )?;

    for leaf in leaves {
        writeln!(out, "    0x{leaf:016X},")?;
    }
    writeln!(
        out,
        "];

#[cfg(test)]
#[test]
fn test_allowed_in_words_casing_closure() {{
    for c in '\\0'..=char::MAX {{
        if allowed_in_word(c) {{
            assert!(c.to_uppercase().all(allowed_in_word));
            assert!(c.to_lowercase().all(allowed_in_word));
        }}
    }}
}}"
    )?;

    Ok(())
}
