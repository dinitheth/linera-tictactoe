use crate::{ApplicationState, ContractState, GameView};
use async_trait::async_trait;
use linera_sdk::{Service, SimpleStateStorage};

pub struct TicTacToeService;

#[async_trait]
impl Service for TicTacToeService {
    type StateStorage = SimpleStateStorage<ContractState>;
    type View = ApplicationState;

    async fn query(
        &self,
        context: &linera_sdk::ApplicationCallbackContext<Self>,
        _query_bytes: Vec<u8>,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let state = context.state().await?;
        
        // Convert contract state to application view
        let games = state.games
            .iter()
            .map(|(id, game)| GameView {
                id: *id,
                creator: game.creator,
                players: game.players.iter().map(|(o, p)| (*o, p.clone())).collect(),
                board: game.board.clone(),
                state: game.state.clone(),
            })
            .collect();
        
        let view = ApplicationState { games };
        
        // Serialize and return the view
        Ok(serde_json::to_vec(&view)?)
    }
}