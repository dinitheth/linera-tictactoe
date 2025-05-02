use crate::{ContractState, Error, Game, Message, Operation, Result};
use async_trait::async_trait;
use linera_sdk::{
    linera_base_types::{ApplicationId, Owner, Timestamp},
    Contract, ExecutionResult, MessageContext, SimpleStateStorage,
};

pub struct TicTacToeContract;

#[async_trait]
impl Contract for TicTacToeContract {
    type Parameters = ();
    type StateStorage = SimpleStateStorage<ContractState>;
    type Message = Message;
    type Operation = Operation;

    async fn initialize(
        &self,
        context: &mut linera_sdk::ApplicationCallbackContext<Self>,
    ) -> Result<ExecutionResult<Self::Message>> {
        let state = ContractState {
            games: Default::default(),
        };
        Ok(ExecutionResult {
            state,
            messages: vec![],
        })
    }

    async fn execute_operation(
        &self,
        context: &mut linera_sdk::ApplicationCallbackContext<Self>,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>> {
        let sender = context.authenticated_signer().unwrap_or_default();
        let owner = Owner::from(sender);
        let mut state = context.state().await?;
        let mut messages = vec![];

        match operation {
            Operation::CreateGame => {
                let game_id = context.message_id();
                
                // Create new game with the sender as creator
                let game = Game::new(owner);
                state.games.insert(game_id, game);
                
                messages.push((
                    context.chain_id(),
                    Message::GameCreated { game_id },
                ));
            }
            Operation::JoinGame { game_id } => {
                let game = state.games.get_mut(&game_id).ok_or_else(|| 
                    Error::GameError("Game not found".to_string()))?;
                
                game.join(owner)?;
                
                messages.push((
                    context.chain_id(),
                    Message::PlayerJoined { game_id, player: owner },
                ));
            }
            Operation::MakeMove { game_id, cell } => {
                let game = state.games.get_mut(&game_id).ok_or_else(|| 
                    Error::GameError("Game not found".to_string()))?;
                
                game.make_move(owner, cell.clone())?;
                
                messages.push((
                    context.chain_id(),
                    Message::MoveMade { game_id, player: owner, cell },
                ));
            }
        }

        Ok(ExecutionResult { state, messages })
    }

    async fn execute_message(
        &self,
        context: &mut linera_sdk::ApplicationCallbackContext<Self>,
        message: MessageContext<Self::Message>,
    ) -> Result<ExecutionResult<Self::Message>> {
        let state = context.state().await?;
        
        // We don't need to do anything with messages in this simple game
        // They are just for notifying about game events
        
        Ok(ExecutionResult { state, messages: vec![] })
    }
}