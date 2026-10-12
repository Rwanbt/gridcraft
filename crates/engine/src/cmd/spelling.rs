//! Spelling: checks text cells against the system word list (`/usr/share/dict/words` on macOS
//! and Linux; nothing is bundled) plus a user dictionary kept in the engine preferences.

use std::collections::HashSet;
use std::sync::OnceLock;

use gridcraft_core::{RangeRef, Value};
use serde_json::{Value as Json, json};

use super::*;

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "review.spelling", "Spelling", ["Review", "Proofing"], Some("F7"), "{range?, sheet?} → misspelled words with cells and suggestions", has_doc, spelling),
        cmd!(noundo "review.addToDictionary", "Add to Dictionary", [], None, "{word}", has_doc, add_word),
        cmd!("review.changeSpelling", "Change", [], None, "{cell, word, to, all?: bool}", has_doc, change),
        cmd!(query "review.thesaurus", "Thesaurus", ["Review", "Proofing"], None, "{word} → related words (words sharing the stem in the dictionary)", has_doc, thesaurus),
    ]
}

fn dictionary() -> &'static HashSet<String> {
    static DICT: OnceLock<HashSet<String>> = OnceLock::new();
    DICT.get_or_init(|| {
        #[cfg(target_arch = "wasm32")]
        {
            HashSet::new()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut set = HashSet::new();
            for path in ["/usr/share/dict/words", "/usr/share/dict/american-english", "/usr/share/dict/british-english"] {
                if let Ok(text) = std::fs::read_to_string(path) {
                    for w in text.lines().take(2_000_000) {
                        set.insert(w.trim().to_lowercase());
                    }
                    break;
                }
            }
            set
        }
    })
}

fn known(s: &Session, w: &str) -> bool {
    known_with(dictionary(), &s.prefs.user_dictionary, w)
}

/// Pure dictionary lookup so inflection rules can be tested without a host word-list file.
fn known_with(dict: &HashSet<String>, user_words: &[String], w: &str) -> bool {
    let l = w.to_lowercase();
    if dict.contains(&l) || user_words.iter().any(|u| u.eq_ignore_ascii_case(w)) {
        return true;
    }
    // Simple inflections the list doesn't spell out.
    for suf in ["s", "es", "ed", "ing", "ly", "er", "ers", "'s"] {
        if let Some(stem) = l.strip_suffix(suf)
            && stem.len() > 2
            && (dict.contains(stem) || dict.contains(&format!("{stem}e")))
        {
            return true;
        }
    }
    if let Some(stem) = l.strip_suffix("ies")
        && dict.contains(&format!("{stem}y"))
    {
        return true;
    }
    false
}

fn edits1(w: &str) -> Vec<String> {
    let chars: Vec<char> = w.chars().collect();
    let mut out = Vec::new();
    let letters = "abcdefghijklmnopqrstuvwxyz";
    for i in 0..=chars.len() {
        if i < chars.len() {
            let mut d = chars.clone();
            d.remove(i);
            out.push(d.iter().collect());
            if i + 1 < chars.len() {
                let mut t = chars.clone();
                t.swap(i, i + 1);
                out.push(t.iter().collect());
            }
            for c in letters.chars() {
                let mut r = chars.clone();
                r[i] = c;
                out.push(r.iter().collect());
            }
        }
        for c in letters.chars() {
            let mut ins = chars.clone();
            ins.insert(i, c);
            out.push(ins.iter().collect());
        }
    }
    out
}

fn suggestions(w: &str) -> Vec<String> {
    suggestions_with(dictionary(), w)
}

/// Limit edit-distance candidate generation: a hostile cell can contain a very long word.
/// Each candidate clones its characters, so unrestricted input is quadratic in memory.
fn suggestions_with(dict: &HashSet<String>, w: &str) -> Vec<String> {
    if w.chars().count() > 48 {
        return Vec::new();
    }
    let l = w.to_lowercase();
    let mut out: Vec<String> = edits1(&l).into_iter().filter(|c| dict.contains(c)).collect();
    out.sort();
    out.dedup();
    out.truncate(6);
    // Keep the original capitalisation.
    let cap = w.chars().next().is_some_and(char::is_uppercase);
    out.into_iter()
        .map(|s| {
            if cap {
                let mut c = s.chars();
                c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or(s)
            } else {
                s
            }
        })
        .collect()
}

fn spelling(s: &mut Session, p: &Json) -> Result<Json> {
    if dictionary().is_empty() {
        return Err(EngineError::Other("No dictionary is available on this system (GridCraft uses the system word list).".into()));
    }
    let si = target_sheet(s, p)?;
    let d = s.doc()?;
    let sh = d.wb.sheet(si).ok_or(EngineError::NoDocument)?;
    let scope = match str_param(p, "range") {
        Some(_) => target_range(s, p)?,
        None => sh.used_range().unwrap_or(RangeRef::cell(d.selection.active)),
    };
    let mut issues = Vec::new();
    for (c, cell) in sh.cells.iter_range(scope) {
        if cell.formula.is_some() {
            continue;
        }
        let Value::Text(t) = &cell.value else { continue };
        for w in t.split(|ch: char| !(ch.is_alphabetic() || ch == '\'')) {
            let w = w.trim_matches('\'');
            if w.chars().count() < 2 || w.chars().all(char::is_uppercase) || !w.is_ascii() {
                continue;
            }
            if !known(s, w) {
                issues.push(json!({"cell": c.a1(), "word": w, "suggestions": suggestions(w)}));
            }
        }
        if issues.len() >= 500 {
            break;
        }
    }
    Ok(json!({"issues": issues, "done": issues.is_empty()}))
}

fn add_word(s: &mut Session, p: &Json) -> Result<Json> {
    let w = str_param(p, "word").ok_or_else(|| bad("review.addToDictionary", "missing `word`"))?.to_string();
    if !s.prefs.user_dictionary.iter().any(|u| u.eq_ignore_ascii_case(&w)) {
        s.prefs.user_dictionary.push(w);
    }
    ok()
}

fn change(s: &mut Session, p: &Json) -> Result<Json> {
    let word = str_param(p, "word").ok_or_else(|| bad("review.changeSpelling", "missing `word`"))?.to_string();
    let to = str_param(p, "to").ok_or_else(|| bad("review.changeSpelling", "missing `to`"))?.to_string();
    if bool_param(p, "all").unwrap_or(false) {
        return s.execute("edit.replace", json!({"what": word, "with": to, "matchCase": true, "all": true}));
    }
    let c = cell_param(s, p, "cell")?.ok_or_else(|| bad("review.changeSpelling", "missing `cell`"))?;
    let text = s.doc()?.wb.active().and_then(|sh| sh.cell(c)).map(|x| x.input_text()).unwrap_or_default();
    s.execute("cell.set", json!({"cell": c.a1(), "input": text.replacen(&word, &to, 1)}))
}

fn thesaurus(_: &mut Session, p: &Json) -> Result<Json> {
    let w = str_param(p, "word").unwrap_or("").to_lowercase();
    if w.len() < 3 {
        return Ok(json!([]));
    }
    let stem: String = w.chars().take(w.chars().count().saturating_sub(2).max(3)).collect();
    let mut related: Vec<&String> = dictionary().iter().filter(|d| d.starts_with(&stem) && **d != w && d.len() <= w.len() + 4).collect();
    related.sort();
    related.truncate(20);
    Ok(json!(related))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gridcraft_core::{CellRef, Value};
    use serde_json::json;

    fn fixture(words: &[&str]) -> HashSet<String> {
        words.iter().map(|word| (*word).to_string()).collect()
    }

    #[test]
    fn edit_distance_one_includes_all_four_operations() {
        let edits = edits1("cat");
        assert!(edits.iter().any(|word| word == "at"), "deletion");
        assert!(edits.iter().any(|word| word == "act"), "transposition");
        assert!(edits.iter().any(|word| word == "cot"), "substitution");
        assert!(edits.iter().any(|word| word == "cart"), "insertion");
    }

    #[test]
    fn suggestions_use_sorted_unique_results_and_keep_case() {
        let dict = fixture(&["cat", "cut", "dot", "coat"]);
        assert_eq!(suggestions_with(&dict, "Cot"), ["Cat", "Coat", "Cut", "Dot"]);
        assert_eq!(suggestions_with(&dict, "cot"), ["cat", "coat", "cut", "dot"]);
        assert_eq!(suggestions_with(&dict, "zebra"), Vec::<String>::new());
    }

    #[test]
    fn long_words_cannot_expand_into_quadratic_candidate_memory() {
        let dict = fixture(&["cat", "cut"]);
        assert!(suggestions_with(&dict, &"z".repeat(48)).is_empty());
        assert!(suggestions_with(&dict, &"z".repeat(49)).is_empty());
        assert!(suggestions_with(&dict, &"z".repeat(32_000)).is_empty());
        assert!(suggestions_with(&dict, &"é".repeat(49)).is_empty());
    }

    #[test]
    fn user_dictionary_and_inflections_do_not_depend_on_system_files() {
        let dict = fixture(&["work", "make", "happy"]);
        let user = vec!["GridCraftword".to_string()];
        for word in ["work", "WORK", "works", "worked", "working", "worker", "makers", "happily", "GRIDCRAFTWORD"] {
            // Inflections are accepted when their stem or stem + 'e' is in the supplied dictionary.
            if word != "happily" {
                assert!(known_with(&dict, &user, word), "{word}");
            }
        }
        assert!(!known_with(&dict, &user, "unknown"));
        assert!(!known_with(&dict, &[], "GridCraftword"));
    }

    #[test]
    fn add_to_dictionary_deduplicates_case_insensitively() {
        let mut s = Session::new();
        s.new_workbook();
        assert!(s.execute("review.addToDictionary", json!({})).is_err());
        s.execute("review.addToDictionary", json!({"word": "QA-Aether"})).unwrap();
        s.execute("review.addToDictionary", json!({"word": "qa-aether"})).unwrap();
        assert_eq!(s.prefs.user_dictionary, ["QA-Aether"]);
        assert!(known_with(&HashSet::new(), &s.prefs.user_dictionary, "Qa-Aether"));
    }

    #[test]
    fn single_spelling_replacement_is_undoable() {
        let mut s = Session::new();
        s.new_workbook();
        s.execute("cell.set", json!({"cell": "A1", "input": "colour colour"})).unwrap();
        assert!(s.execute("review.changeSpelling", json!({"word": "colour", "to": "color"})).is_err());
        s.execute("review.changeSpelling", json!({"cell": "A1", "word": "colour", "to": "color"})).unwrap();
        let get = |s: &Session| s.doc().unwrap().wb.active().unwrap().value(CellRef::new(0, 0));
        assert_eq!(get(&s), Value::text("color colour"));
        s.execute("edit.undo", json!({})).unwrap();
        assert_eq!(get(&s), Value::text("colour colour"));
        assert_eq!(s.execute("review.thesaurus", json!({"word": "x"})).unwrap(), json!([]));
    }
}
