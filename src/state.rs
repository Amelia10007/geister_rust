use data_structure::{Table, TableIndex, TableSize};
use minimax_strategy::{actors, Actor, State};
use std::collections::HashMap;

/// ゲームフィールドの大きさ．
pub const FIELD_SIZE: TableSize = TableSize::new(6, 6);

/// ゲーム開始時にフィールドに存在する👻の数．
pub const INITIAL_GEISTER_COUNT: usize = 4;

/// ゲームフィールドに登場する駒(👻)の種類．
/// プレイヤーはこの駒を操作してゲームを進めていく．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Geister {
    /// 善良な👻．
    Holy,
    /// 邪悪な👻．
    Evil,
}

/// 👻とその所有者をセットで表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnedGeister {
    /// 👻の種類．
    pub geister: Geister,
    /// 👻の所有者．
    pub owner: Actor,
}

impl OwnedGeister {
    pub fn new(geister: Geister, owner: Actor) -> Self {
        Self { geister, owner }
    }
}

/// ゲームGeisterの状態．
#[derive(Clone)]
pub struct GeisterState {
    /// フィールドに存在する👻．
    pub lattices: Table<Option<OwnedGeister>>,
    /// フィールドから取り除かれた👻の数．
    killed_geister_counts: HashMap<OwnedGeister, usize>,
    /// フィールドから上がった👻の所有者．
    /// これが`None`でないということは，ゲームが終了したことを表す．
    pub actor_of_cleared_geister: Option<Actor>,
}

impl GeisterState {
    /// 指定した初期配置をもとに，初期状態を生成する．
    pub fn create_initial_state(
        initial_geister_positions: HashMap<OwnedGeister, Vec<TableIndex>>,
    ) -> Self {
        // フィールドに👻を配置
        let mut lattices = Table::from_fill(None, FIELD_SIZE);
        for (owned_geister, positions) in initial_geister_positions {
            // 👻の数が想定と合っているか確かめる
            assert_eq!(INITIAL_GEISTER_COUNT, positions.len());

            for &position in positions.iter() {
                lattices[position] = Some(owned_geister);
            }
        }

        // ゲーム開始時，フィールドから取り除かれた👻は一体もいない
        let mut killed_geister_counts = HashMap::new();
        for &geister in geisters().iter() {
            for &actor in actors().iter() {
                let owned_geister = OwnedGeister::new(geister, actor);
                killed_geister_counts.insert(owned_geister, 0);
            }
        }

        Self {
            lattices,
            killed_geister_counts,
            actor_of_cleared_geister: None,
        }
    }

    pub fn iterate_geister_positions_of(
        &'_ self,
        actor: Actor,
    ) -> impl Iterator<Item = TableIndex> + '_ {
        self.lattices
            .iter_row()
            .enumerate()
            .flat_map(move |(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(move |(_x, lattice)| lattice.map(|l| l.owner) == Some(actor))
                    .map(move |(x, _)| TableIndex::new(x, y))
            })
    }

    pub fn iterate_owned_geister_positions(
        &'_ self,
        owned_geister: OwnedGeister,
    ) -> impl Iterator<Item = TableIndex> + '_ {
        self.lattices
            .iter_row()
            .enumerate()
            .flat_map(move |(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(move |(_x, lattice)| lattice == &&Some(owned_geister))
                    .map(move |(x, _)| TableIndex::new(x, y))
            })
    }

    /// 指定した👻が何体フィールドから取り除かれたか返す．
    pub fn killed_geister_count(&self, owned_geister: OwnedGeister) -> usize {
        self.killed_geister_counts[&owned_geister]
    }

    /// 指定した位置に👻がいれば，その👻をフィールドから取り除く (いなかった場合は何もしない)．
    /// 👻を取り除いた場合，フィールドから取り除かれた👻のカウントを更新する．
    pub fn kill_geister_at(&mut self, position: TableIndex) {
        if let Some(killed_owned_geister) = self.lattices[position] {
            // 元々👻がいたところにはもう何もない
            self.lattices[position] = None;
            // 取り除かれた👻の数を更新
            *self
                .killed_geister_counts
                .get_mut(&killed_owned_geister)
                .expect("key (OwnedGeister) must exist") += 1;
        }
    }
}

impl State for GeisterState {}

/// ゲームに登場するすべての👻を返す．
pub fn geisters() -> [Geister; 2] {
    [Geister::Evil, Geister::Holy]
}
