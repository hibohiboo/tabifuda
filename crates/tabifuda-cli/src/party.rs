//! パーティファイル(`Vec<Character>`のJSON)の読み込み・検証(domain-model.md
//! 「パーティファイル(CLIの決定)」)。coreのdecideはparty内`CharacterId`の
//! 一意性を前提としないため、ここで検証する。

use std::collections::BTreeSet;
use std::io;
use std::path::Path;

use tabifuda_core::Character;

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Parse(serde_json::Error),
    Empty,
    DuplicateCharacterId(String),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(err) => write!(f, "読み込みに失敗しました: {err}"),
            LoadError::Parse(err) => write!(f, "パーティファイルの形式が不正です: {err}"),
            LoadError::Empty => write!(f, "パーティが空です(1人以上必要です)"),
            LoadError::DuplicateCharacterId(id) => {
                write!(f, "CharacterIdが重複しています: {id}")
            }
        }
    }
}

/// パーティファイルを読み込み、検証済みの`Vec<Character>`を返す。
/// 空配列・`CharacterId`重複は拒否する(domain-model.md「パーティファイル
/// (CLIの決定)」)。
pub fn load(path: &Path) -> Result<Vec<Character>, LoadError> {
    let text = std::fs::read_to_string(path).map_err(LoadError::Io)?;
    let party: Vec<Character> = serde_json::from_str(&text).map_err(LoadError::Parse)?;
    validate(&party)?;
    Ok(party)
}

fn validate(party: &[Character]) -> Result<(), LoadError> {
    if party.is_empty() {
        return Err(LoadError::Empty);
    }
    let mut seen = BTreeSet::new();
    for character in party {
        if !seen.insert(&character.id) {
            return Err(LoadError::DuplicateCharacterId(character.id.0.clone()));
        }
    }
    Ok(())
}

// テスト名は日本語で検証内容を表す(docs/tasks/tools/docs-site/task.md D2)
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use tabifuda_core::CharacterId;

    fn character(id: &str) -> Character {
        Character {
            id: CharacterId(id.to_string()),
            name: id.to_string(),
            stats: BTreeMap::new(),
            deck: vec![],
            owned_cards: vec![],
        }
    }

    fn write_temp(name: &str, content: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("tabifuda-party-test-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("party.json");
        std::fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn 妥当なパーティファイルを読み込める() {
        let party = vec![character("alice"), character("bob")];
        let path = write_temp("valid", &serde_json::to_string(&party).unwrap());

        let loaded = load(&path).unwrap();
        std::fs::remove_dir_all(path.parent().unwrap()).ok();

        assert_eq!(loaded, party);
    }

    #[test]
    fn 空配列のパーティファイルは拒否される() {
        let path = write_temp("empty", "[]");

        let result = load(&path);
        std::fs::remove_dir_all(path.parent().unwrap()).ok();

        assert!(matches!(result, Err(LoadError::Empty)));
    }

    #[test]
    fn CharacterIdが重複するパーティファイルは拒否される() {
        let party = vec![character("alice"), character("alice")];
        let path = write_temp("dup", &serde_json::to_string(&party).unwrap());

        let result = load(&path);
        std::fs::remove_dir_all(path.parent().unwrap()).ok();

        assert!(matches!(result, Err(LoadError::DuplicateCharacterId(id)) if id == "alice"));
    }
}
