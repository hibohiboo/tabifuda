//! セッション保存ファイルの読み書き(domain-model.md「セッションの保存と
//! 再開(CLIの決定)」)。イベント列だけで自己完結する(SessionStartedが
//! シナリオ・パーティの凍結コピーを持つため)。ここは翻訳層のIOであり、
//! Session復元(apply畳み込み)はplay.rs側が担う。

use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tabifuda_core::Event;

/// セーブファイルのフォーマットversion。Event enumは`#[non_exhaustive]`で
/// 追加前提のため、不一致は警告なしで拒否する(P3.5 C1決定)。
pub const FORMAT_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
struct SaveFile {
    format_version: u32,
    events: Vec<Event>,
}

pub enum LoadError {
    Io(io::Error),
    Parse(serde_json::Error),
    VersionMismatch { found: u32 },
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(err) => write!(f, "読み込みに失敗しました: {err}"),
            LoadError::Parse(err) => write!(f, "保存ファイルの形式が不正です: {err}"),
            LoadError::VersionMismatch { found } => write!(
                f,
                "保存ファイルの形式が古い/新しいため読み込めません(format_version: {found}, 対応: {FORMAT_VERSION})"
            ),
        }
    }
}

/// イベント列を保存する。書き込み失敗はIOエラーとしてそのまま返す。
pub fn write(events: &[Event], path: &Path) -> io::Result<()> {
    let save = SaveFile {
        format_version: FORMAT_VERSION,
        events: events.to_vec(),
    };
    let json = serde_json::to_string_pretty(&save).expect("SaveFileはシリアライズ可能");
    std::fs::write(path, json + "\n")
}

/// 保存ファイルを読み込み、イベント列を返す。`format_version`が現在の実装と
/// 一致しない場合は拒否する(警告付き読込はしない)。
pub fn load(path: &Path) -> Result<Vec<Event>, LoadError> {
    let text = std::fs::read_to_string(path).map_err(LoadError::Io)?;
    let save: SaveFile = serde_json::from_str(&text).map_err(LoadError::Parse)?;
    if save.format_version != FORMAT_VERSION {
        return Err(LoadError::VersionMismatch {
            found: save.format_version,
        });
    }
    Ok(save.events)
}

/// 元ファイルの隣に`{語幹}-save.json`(既存ファイルと衝突したら
/// `{語幹}-save-2.json`から連番)で出力パスを決める(fork.rsの
/// fork_output_pathと同じ方針)。
pub fn save_output_path(scenario_path: &Path) -> PathBuf {
    let stem = scenario_path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "session".to_string());
    let dir = scenario_path.parent().unwrap_or_else(|| Path::new("."));
    let first = dir.join(format!("{stem}-save.json"));
    if !first.exists() {
        return first;
    }
    (2..)
        .map(|n| dir.join(format!("{stem}-save-{n}.json")))
        .find(|p| !p.exists())
        .expect("連番はいつか空きに当たる")
}

// テスト名は日本語で検証内容を表す(docs/tasks/tools/docs-site/task.md D2)
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use tabifuda_core::Outcome;

    fn sample_events() -> Vec<Event> {
        vec![Event::SessionEnded {
            outcome: Outcome::Victory,
        }]
    }

    #[test]
    fn 保存したファイルを読み込むと同じイベント列が復元される() {
        let dir = std::env::temp_dir().join(format!(
            "tabifuda-save-test-roundtrip-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("session.json");

        write(&sample_events(), &path).unwrap();
        let loaded = load(&path);
        std::fs::remove_dir_all(&dir).ok();

        match loaded {
            Ok(events) => assert_eq!(events, sample_events()),
            Err(_) => panic!("読み込みに失敗した"),
        }
    }

    #[test]
    fn format_versionが不一致の保存ファイルは拒否される() {
        let dir =
            std::env::temp_dir().join(format!("tabifuda-save-test-version-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("session.json");
        std::fs::write(&path, r#"{"format_version":999,"events":[]}"#).unwrap();

        let result = load(&path);
        std::fs::remove_dir_all(&dir).ok();

        assert!(matches!(
            result,
            Err(LoadError::VersionMismatch { found: 999 })
        ));
    }
}
