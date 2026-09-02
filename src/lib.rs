// Data relative to the mini-game

use itertools::join; // for converting Vec<FullMove> to JSON

// #[derive(Clone)]
pub enum UCIstr
{
    Move4char([char; 4]),
    Move5char([char; 5]),
}

impl UCIstr {
    fn new(input_str: &str) -> UCIstr {
        match input_str.len() {
            4 => UCIstr::Move4char(input_str.chars().collect::<Vec<char>>().try_into().unwrap()), //incompatible with 'const'
            5 => UCIstr::Move5char(input_str.chars().collect::<Vec<char>>().try_into().unwrap()),
            _ => panic!("Invalid UCI string length"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            UCIstr::Move4char(chars) => chars.iter().collect(),
            UCIstr::Move5char(chars) => chars.iter().collect(),
        }
    }
}


// #[derive(Clone)]
pub struct FullMove
{
    pub player: UCIstr,
    pub opponent: UCIstr,
}


impl FullMove {
    fn new(str_player: &str, str_opponent: &str) -> FullMove {
        FullMove { player : UCIstr::new(str_player)
            , opponent : UCIstr::new(str_opponent) }
    }

    /// Converts a FullMove to a JSON string for saving to DB
    fn to_json(&self) -> String {
        format!("{{\"player\": \"{}\", \"opponent\": \"{}\"}}", self.player.to_string(), self.opponent.to_string())
    }
}

/// Converts an array of FullMoves to a JSON string for saving to DB
pub fn to_json(arr_full_moves : &[FullMove]) -> String
{
    let comma_sep_str = join(arr_full_moves
                                 .iter()
                                 .map(|full_move: &FullMove| full_move.to_json()),
                             ",");

    format!("[{}]", comma_sep_str)
}

/// Defines the correct solution for level 1.
pub fn solution_lvl1() -> [FullMove; 3]
{
    [
        FullMove::new("c6c7", "d5d6"),
        FullMove::new("b6b5", "d6d5"),
        FullMove::new("b5b4", "d5d4"),
    ]
}

/// Attempts to decode a hex-encoded session hash from a string.
pub fn decode_session_hash(hex_str: &str) -> Option<[u8; 32]> {
    match hex::decode(hex_str) {
        Ok(bytes) => {
            if bytes.len() == 32 {
                Some(bytes.try_into().unwrap())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}


/// For gathering connection information (fields of fixed size in bytes).
/// Fn 'use_persistent', called in main.rs, requires:
/// Serialize + DeserializeOwned + Clone + Send + Sync + PartialEq + 'static
#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ConnectionInfo
{
    pub connection_id: i32,
    pub user_id: i32,
    pub session_hash: [u8; 32],
}

impl ConnectionInfo {
    
    /// Copies the field session_hash from the ConnectionInfo struct
    /// for passing the copy from frontend to middleware.
    pub fn copy_session_hash(&self) -> [u8; 32] {
        self.session_hash
    }
    
}

#[test]
fn test_fullmove_to_json() {
    let full_move = FullMove::new("c6c7", "d5d6");
    let json_str = full_move.to_json();
    println!("JSON string: {}", json_str); // {"player": "c6c7", "opponent": "d5d6"}
    assert_eq!(json_str, r#"{"player": "c6c7", "opponent": "d5d6"}"#);
}

#[test]
fn test_arr_to_json() {
    let arr_full_moves = &solution_lvl1()[..2] ; // example: 2 correct moves
    let json_str = to_json(arr_full_moves);
    println!("JSON string for array: {}", json_str);
    // [{"player": "c6c7", "opponent": "d5d6"},{"player": "b6b5", "opponent": "d6d5"}]

    assert_eq!(json_str, r#"[{"player": "c6c7", "opponent": "d5d6"},{"player": "b6b5", "opponent": "d6d5"}]"#);
}
