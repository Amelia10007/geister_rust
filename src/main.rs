mod action;
mod evaluation;
mod rule;
mod state;

use action::*;
use cui_gaming::*;
use data_structure::{Pair, TableIndex};
use minimax_strategy::{actors, Actor, Rule, Strategy};
use rand::seq::SliceRandom;
use rule::*;
use state::*;
use std::collections::HashMap;

/// ゲームユーザーの意思決定を管理する．
struct PlayerStrategy {
    /// ユーザーのからのキー入力を監視する．
    keyboard_input: KeyboardInput,
}

impl Strategy<GeisterState, GeisterAction> for PlayerStrategy {
    fn select_action(&self, state: &GeisterState, actor: Actor) -> Option<GeisterAction> {
        // まずは実行可能な行動を列挙．このゲームでは必ずひとつ以上の行動がとれるはず．
        let available_actions = GeisterRule::iterate_available_actions(&state, actor);
        assert!(available_actions.len() > 0);

        // 自分が所有している👻の位置を求め，右上のものから順に配列に格納していく．
        let own_geister_positions = {
            let mut positions = vec![];
            for (y, row) in state.lattices.iter_row().enumerate() {
                for (x, &lattice) in row.iter().enumerate() {
                    if let Some(owned_geister) = lattice {
                        if owned_geister.owner == actor {
                            positions.push(TableIndex::new(x, y));
                        }
                    }
                }
            }
            positions
        };

        loop {
            // 何番目の👻を動かすか
            let i: usize = ui::input_parsable("Input an index of which you want to move");

            let action = match own_geister_positions.get(i) {
                // 指定した番号の👻が存在すれば，移動方法の入力へ移る
                Some(&target_geister_position) => {
                    println!("Type a direction to move, or Enter to clear");
                    let geister_movement = match self.keyboard_input.read_key().ok()? {
                        Key::ArrowUp => GeisterMovement::Direction(Pair::new(0, -1)),
                        Key::ArrowDown => GeisterMovement::Direction(Pair::new(0, 1)),
                        Key::ArrowLeft => GeisterMovement::Direction(Pair::new(-1, 0)),
                        Key::ArrowRight => GeisterMovement::Direction(Pair::new(1, 0)),
                        Key::Enter => GeisterMovement::Clear,
                        _ => {
                            println!("Invalid keyboard input");
                            continue;
                        }
                    };
                    GeisterAction::new(target_geister_position, geister_movement, actor)
                }
                None => {
                    println!("Invalid index");
                    continue;
                }
            };

            // 入力された行動が実行可能なものであれば，それを返す．
            // そうでない場合はもう一度入力をやり直させる．
            if let Some(_) = available_actions.iter().filter(|&&a| a == action).next() {
                return Some(action);
            } else {
                println!("Unavailable action");
                continue;
            }
        }
    }
}

/// 👻の初期位置をランダムに決定する．
fn select_initial_positions() -> HashMap<OwnedGeister, Vec<TableIndex>> {
    let mut hash_map = HashMap::new();

    for &actor in actors().iter() {
        // 配置可能な位置を決定する．
        // 各プレイヤー，最も自分側よりの2行で，かつ端以外の列にあるマスに👻を配置できる．
        let mut available_positions = (1..FIELD_SIZE.x - 1)
            .flat_map(|x| {
                let y_range = match actor {
                    Actor::First => FIELD_SIZE.y - 2..FIELD_SIZE.y,
                    Actor::Second => 0..2,
                };
                y_range.map(move |y| TableIndex::new(x, y))
            })
            .collect::<Vec<_>>();
        assert!(available_positions.len() >= INITIAL_GEISTER_COUNT * 2);
        // 列挙した配置可能位置をランダムな順序にする．
        available_positions.shuffle(&mut rand::thread_rng());

        // 配置可能位置の配列から👻の初期個数ずつ要素を取り出して，各👻の初期位置とする．
        for (&geister, positions) in geisters()
            .iter()
            .zip(available_positions.chunks_exact(INITIAL_GEISTER_COUNT))
        {
            let owned_geister = OwnedGeister::new(geister, actor);
            let positions = positions.iter().map(|&p| p).collect();
            hash_map.insert(owned_geister, positions);
        }
    }

    hash_map
}

/// 指定した👻を表示する際の文字を返す．
/// # Params
/// 1. `owned_geister` 表示対象の👻．
/// 1. `geister_index` 👻を番号付きで表示したい場合に`Some(i)`として指定する．
/// この番号付けは，ユーザーが行動を選択するときに👻を区別するために使用する．
fn drawable_unit_of(owned_geister: OwnedGeister, geister_index: Option<usize>) -> DrawableUnit {
    let left_char = match owned_geister.geister {
        Geister::Holy => 'H',
        Geister::Evil => 'E',
    };
    let right_char = match geister_index {
        Some(i) => i.to_string().chars().nth(0).unwrap(),
        None => ' ',
    };
    let color = match owned_geister.owner {
        Actor::First => match owned_geister.geister {
            Geister::Holy => UnitColor::Blue,
            Geister::Evil => UnitColor::Red,
        },
        Actor::Second => match owned_geister.geister {
            Geister::Holy => UnitColor::Cyan,
            Geister::Evil => UnitColor::Magenta,
        },
    };
    DrawableUnit::from_double_half_char(left_char, right_char, color)
}

/// 指定したプレイヤー視点から見た場合のゲーム状態を返す．
/// # Params
/// 1. `state` ゲーム状態
/// 1. `viewpoint_actor` 視点プレイヤーを`Some(p)`として指定する．
/// 神視点から見た状態を返したい場合は`None`とする．
fn write_state_for(
    state: &GeisterState,
    viewpoint_actor: Option<Actor>,
) -> Result<String, DrawError> {
    let mut s = String::new();
    // 各プレイヤーの取り除かれた👻の数を表示
    for &actor in actors().iter() {
        for c in DrawableUnit::create_units_from(
            &format!("Killed Geisters of {:?}: ", actor),
            UnitColor::White,
        )
        .into_iter()
        {
            c.write_to(&mut s)?;
        }

        for &geister in geisters().iter() {
            let owned_geister = OwnedGeister::new(geister, actor);
            let killed_geister_count = state.killed_geister_count(owned_geister);
            drawable_unit_of(owned_geister, None).write_to(&mut s)?;
            for c in DrawableUnit::create_units_from(
                &format!(": {}  ", killed_geister_count),
                UnitColor::White,
            )
            .into_iter()
            {
                c.write_to(&mut s)?;
            }
        }
        s += "\n";
    }

    // フィールドを表示
    let mut index = 0;
    for row in state.lattices.iter_row() {
        for &lattice in row.iter() {
            // 各マスに何を表示するか決定する．
            let unit = match lattice {
                Some(owned_geister) => match viewpoint_actor {
                    Some(viewpoint_actor) => {
                        if owned_geister.owner == viewpoint_actor {
                            let unit = drawable_unit_of(owned_geister, Some(index));
                            index += 1;
                            unit
                        } else {
                            DrawableUnit::from_double_half_char('?', ' ', UnitColor::White)
                        }
                    }
                    None => drawable_unit_of(owned_geister, None),
                },
                None => DrawableUnit::from_double_half_char('-', '-', UnitColor::White),
            };
            // 表示
            unit.write_to(&mut s)?;
        }
        // 一行表示し終わったら改行
        s += "\n";
    }

    Ok(s)
}

fn main() {
    let strategy = PlayerStrategy {
        keyboard_input: KeyboardInput::new(),
    };
    let mut current_state = GeisterState::create_initial_state(select_initial_positions());
    let mut current_actor = Actor::First;

    while !GeisterRule::is_game_over(&current_state) {
        // 相手プレイヤーの情報が見えないように，端末の表示内容をクリア
        print!("\x1B[2J");
        // 現在の状態を表示
        println!("{:?}'s turn", current_actor);
        match write_state_for(&current_state, Some(current_actor)) {
            Ok(s) => println!("{}", s),
            Err(e) => {
                println!("An error was occurred during writing field: {}", e);
                break;
            }
        }

        // 行動選択
        let action = strategy
            .select_action(&current_state, current_actor)
            .expect("At least 1 action must be available");
        // 状態遷移とターンプレイヤー交代
        current_state = GeisterRule::translate_state(&current_state, &action);
        current_actor = current_actor.opponent();

        println!();
    }

    // ゲーム勝者を表示
    println!(
        "The winner is {:?}",
        GeisterRule::winner_of(&current_state).expect("The winner must be determined")
    );
    println!("{}", write_state_for(&current_state, None).unwrap());
}
