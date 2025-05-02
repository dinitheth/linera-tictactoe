use linera_sdk::linera_base_types::{ApplicationId, ChainId, MessageId, Owner};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Game error: {0}")]
    GameError(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Player {
    X,
    O,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cell {
    row: usize,
    col: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameState {
    WaitingForPlayers,
    InProgress { current_turn: Player },
    Finished { winner: Option<Player> },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Game {
    pub creator: Owner,
    pub players: HashMap<Owner, Player>,
    pub board: [[Option<Player>; 3]; 3],
    pub state: GameState,
}

impl Game {
    pub fn new(creator: Owner) -> Self {
        let mut game = Game {
            creator,
            players: HashMap::new(),
            board: [[None; 3]; 3],
            state: GameState::WaitingForPlayers,
        };
        
        // Assign creator as player X
        game.players.insert(creator, Player::X);
        
        game
    }

    pub fn join(&mut self, player: Owner) -> Result<()> {
        match self.state {
            GameState::WaitingForPlayers => {
                if self.players.contains_key(&player) {
                    return Err(Error::GameError("Player already joined".to_string()));
                }
                
                // Assign as player O
                self.players.insert(player, Player::O);
                
                // Start the game with player X's turn
                self.state = GameState::InProgress { current_turn: Player::X };
                Ok(())
            }
            _ => Err(Error::GameError("Game is not waiting for players".to_string())),
        }
    }

    pub fn make_move(&mut self, player: Owner, cell: Cell) -> Result<()> {
        let player_type = self.players.get(&player).ok_or_else(|| 
            Error::GameError("Player not in game".to_string()))?;
        
        match &self.state {
            GameState::InProgress { current_turn } => {
                if current_turn != player_type {
                    return Err(Error::GameError("Not your turn".to_string()));
                }
                
                if cell.row >= 3 || cell.col >= 3 {
                    return Err(Error::GameError("Invalid cell".to_string()));
                }
                
                if self.board[cell.row][cell.col].is_some() {
                    return Err(Error::GameError("Cell already occupied".to_string()));
                }
                
                // Make the move
                self.board[cell.row][cell.col] = Some(player_type.clone());
                
                // Check for win or draw
                if self.check_winner(player_type.clone()) {
                    self.state = GameState::Finished { winner: Some(player_type.clone()) };
                } else if self.is_board_full() {
                    self.state = GameState::Finished { winner: None };
                } else {
                    // Switch turns
                    let next_turn = match player_type {
                        Player::X => Player::O,
                        Player::O => Player::X,
                    };
                    self.state = GameState::InProgress { current_turn: next_turn };
                }
                
                Ok(())
            }
            _ => Err(Error::GameError("Game is not in progress".to_string())),
        }
    }

    fn check_winner(&self, player: Player) -> bool {
        // Check rows
        for row in 0..3 {
            if (0..3).all(|col| self.board[row][col] == Some(player.clone())) {
                return true;
            }
        }
        
        // Check columns
        for col in 0..3 {
            if (0..3).all(|row| self.board[row][col] == Some(player.clone())) {
                return true;
            }
        }
        
        // Check diagonals
        if (0..3).all(|i| self.board[i][i] == Some(player.clone())) {
            return true;
        }
        
        if (0..3).all(|i| self.board[i][2-i] == Some(player.clone())) {
            return true;
        }
        
        false
    }

    fn is_board_full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContractState {
    pub games: HashMap<MessageId, Game>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Operation {
    CreateGame,
    JoinGame { game_id: MessageId },
    MakeMove { game_id: MessageId, cell: Cell },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Message {
    GameCreated { game_id: MessageId },
    PlayerJoined { game_id: MessageId, player: Owner },
    MoveMade { game_id: MessageId, player: Owner, cell: Cell },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameView {
    pub id: MessageId,
    pub creator: Owner,
    pub players: Vec<(Owner, Player)>,
    pub board: [[Option<Player>; 3]; 3],
    pub state: GameState,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationState {
    pub games: Vec<GameView>,
}

mod contract;
mod service;

pub use contract::TicTacToeContract;
pub use service::TicTacToeService;