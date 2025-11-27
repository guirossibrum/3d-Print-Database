// src/handlers/selection.rs
use crate::app::App;

/// Build a selection Vec<bool> for `items` where entries matching `selected_items` are true.
pub fn build_selection_state(items: &[String], selected_items: &[String]) -> Vec<bool> {
    items
        .iter()
        .map(|it| selected_items.iter().any(|s| s == it))
        .collect()
}

/// Restore a selection after sorting/dedup: given previously selected names and new item list,
/// produce `Vec<bool>` of same length as `new_items`.
pub fn restore_selection_by_name(previously_selected: &[String], new_items: &[String]) -> Vec<bool> {
    new_items
        .iter()
        .map(|item| previously_selected.iter().any(|s| s == item))
        .collect()
}

/// Replace selection in app for tags/materials generically.
pub fn apply_selection_to_app(app: &mut App, selection: Vec<bool>, kind: SelectionKind) {
    match kind {
        SelectionKind::Tags => app.tag_selection = selection,
        SelectionKind::Materials => app.tag_selection = selection,
        SelectionKind::Categories => {} // categories handled differently (single-select)
    }
}

pub enum SelectionKind {
    Tags,
    Materials,
    Categories,
}

/// Attempt to keep previously selected items when the items list is reloaded/sorted.
/// old_items: previous full list
/// old_selected: previous Vec<bool>
/// new_items: refreshed list
pub fn remap_selection_from_old(old_items: &[String], old_selected: &[bool], new_items: &[String]) -> Vec<bool> {
    // Build list of previously selected names
    let mut prev_names = Vec::new();
    for (i, &sel) in old_selected.iter().enumerate() {
        if sel {
            if let Some(name) = old_items.get(i) {
                prev_names.push(name.clone());
            }
        }
    }
    restore_selection_by_name(&prev_names, new_items)
}
