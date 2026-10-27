//! Core application discovery for Neon Desktop.
use std::{collections::HashSet, env, fs, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
}

/// Rank installed applications for a keyboard-driven launcher.
/// Exact names precede prefixes, word prefixes, and substring matches.
pub fn search_applications<'a>(apps: &'a [Application], query: &str) -> Vec<&'a Application> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return apps.iter().collect();
    }
    let mut matches: Vec<_> = apps
        .iter()
        .filter_map(|app| {
            let name = app.name.to_lowercase();
            let id = app.id.to_lowercase();
            let score = if name == query {
                0
            } else if name.starts_with(&query) {
                1
            } else if name.split_whitespace().any(|word| word.starts_with(&query)) {
                2
            } else if name.contains(&query) {
                3
            } else if id.contains(&query) {
                4
            } else {
                return None;
            };
            Some((score, app))
        })
        .collect();
    matches.sort_by(|(a_score, a), (b_score, b)| {
        a_score.cmp(b_score).then_with(|| a.name.cmp(&b.name))
    });
    matches.into_iter().map(|(_, app)| app).collect()
}

/// Discover visible applications using XDG data directories. User entries override system entries.
pub fn applications() -> Vec<Application> {
    let mut dirs = vec![
        env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(env::var_os("HOME").unwrap_or_default()).join(".local/share")
            }),
    ];
    dirs.extend(env::split_paths(
        &env::var_os("XDG_DATA_DIRS").unwrap_or_else(|| "/usr/local/share:/usr/share".into()),
    ));
    let mut seen = HashSet::new();
    let mut found = Vec::new();
    for dir in dirs {
        let app_dir = dir.join("applications");
        let Ok(entries) = fs::read_dir(app_dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "desktop") {
                continue;
            }
            let Some(id) = path.file_name().and_then(|v| v.to_str()).map(str::to_owned) else {
                continue;
            };
            if !seen.insert(id.clone()) {
                continue;
            }
            if let Ok(content) = fs::read_to_string(path)
                && let Some(mut app) = parse_desktop_entry(&content)
            {
                app.id = id;
                found.push(app);
            }
        }
    }
    found.sort_by_key(|a| a.name.to_lowercase());
    found
}

/// Parse the Desktop Entry main group. Hidden entries are excluded.
pub fn parse_desktop_entry(text: &str) -> Option<Application> {
    let mut in_group = false;
    let (mut name, mut exec, mut icon) = (None, None, None);
    let (mut hidden, mut terminal) = (false, false);
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_group = line == "[Desktop Entry]";
            continue;
        }
        if !in_group || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            match key {
                "Name" => name = Some(value.to_owned()),
                "Exec" => exec = Some(value.to_owned()),
                "Icon" => icon = Some(value.to_owned()),
                "Hidden" | "NoDisplay" if value == "true" => hidden = true,
                "Terminal" if value == "true" => terminal = true,
                _ => {}
            }
        }
    }
    if hidden || terminal {
        return None;
    }
    Some(Application {
        id: String::new(),
        name: name?,
        exec: exec?,
        icon,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_visible_entry() {
        let app = parse_desktop_entry(
            "[Desktop Entry]\nName=Files\nExec=nautilus %U\nIcon=org.gnome.Nautilus\n",
        )
        .unwrap();
        assert_eq!(app.name, "Files");
        assert_eq!(app.icon.as_deref(), Some("org.gnome.Nautilus"));
    }
    #[test]
    fn rejects_hidden_and_terminal() {
        assert!(parse_desktop_entry("[Desktop Entry]\nName=A\nExec=a\nHidden=true").is_none());
        assert!(parse_desktop_entry("[Desktop Entry]\nName=A\nExec=a\nTerminal=true").is_none());
    }
    #[test]
    fn search_ranks_exact_and_word_prefix() {
        let apps = vec![
            Application {
                id: "org.example.Photo.desktop".into(),
                name: "Photo Editor".into(),
                exec: "photo".into(),
                icon: None,
            },
            Application {
                id: "org.example.Editor.desktop".into(),
                name: "Editor".into(),
                exec: "editor".into(),
                icon: None,
            },
            Application {
                id: "org.example.Notes.desktop".into(),
                name: "Notes Editor".into(),
                exec: "notes".into(),
                icon: None,
            },
        ];
        let result = search_applications(&apps, "editor");
        assert_eq!(
            result.iter().map(|a| a.name.as_str()).collect::<Vec<_>>(),
            vec!["Editor", "Notes Editor", "Photo Editor"]
        );
    }
}
