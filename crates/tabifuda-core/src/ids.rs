//! 各種ID型(newtype)。生Stringを引き回さないための包装(CLAUDE.md規約)。

macro_rules! id_type {
    ($name:ident) => {
        #[cfg_attr(test, derive(proptest_derive::Arbitrary))]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub struct $name(pub String);
    };
}

id_type!(ScenarioId);
id_type!(SceneId);
id_type!(CardId);
id_type!(CardInstanceId);
id_type!(CharacterId);
id_type!(UserId);
id_type!(ProposalId);
id_type!(StatId);
