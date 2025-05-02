// Game variables
let walletUrl = '';
let walletConnected = false;
let gameContract = null;
let currentChainId = null;
let currentOwner = null;
let games = [];
let activeGame = null;

// DOM Elements
const walletUrlInput = document.getElementById('wallet-url');
const connectWalletButton = document.getElementById('connect-wallet');
const connectionPanel = document.getElementById('connection-panel');
const gamePanel = document.getElementById('game-panel');
const createGameButton = document.getElementById('create-game');
const gameListElement = document.getElementById('game-list');
const activeGameElement = document.getElementById('active-game');
const gameIdElement = document.getElementById('game-id');
const gameStatusElement = document.getElementById('game-status');
const gameBoardElement = document.getElementById('game-board');

// Event Listeners
connectWalletButton.addEventListener('click', connectWallet);
createGameButton.addEventListener('click', createGame);

// Initialize the app
init();

async function init() {
    // Check if wallet URL is stored in localStorage
    const storedWalletUrl = localStorage.getItem('walletUrl');
    if (storedWalletUrl) {
        walletUrlInput.value = storedWalletUrl;
        await connectWallet();
    }
}

async function connectWallet() {
    try {
        walletUrl = walletUrlInput.value.trim();
        if (!walletUrl) {
            alert('Please enter a wallet URL');
            return;
        }
        
        // Store wallet URL in localStorage
        localStorage.setItem('walletUrl', walletUrl);
        
        // Simple API to connect to wallet (simplified for tutorial purposes)
        // In a real application, you would use the Linera wallet API
        const response = await fetch(`${walletUrl}/wallet/info`, {
            method: 'GET',
            headers: { 'Content-Type': 'application/json' }
        });
        
        if (!response.ok) {
            throw new Error('Failed to connect to wallet');
        }
        
        const data = await response.json();
        currentChainId = data.default_chain_id;
        currentOwner = data.owner;
        
        walletConnected = true;
        connectionPanel.style.display = 'none';
        gamePanel.style.display = 'block';
        
        // Load games
        await loadGames();
        
        // Set up periodic refresh
        setInterval(loadGames, 5000);
        
    } catch (error) {
        console.error('Error connecting to wallet:', error);
        alert(`Failed to connect to wallet: ${error.message}`);
    }
}

async function loadGames() {
    try {
        // In a real application, you would query the contract application service
        const response = await fetch(`${walletUrl}/query/${currentChainId}/tictactoe`, {
            method: 'GET',
            headers: { 'Content-Type': 'application/json' }
        });
        
        if (!response.ok) {
            throw new Error('Failed to load games');
        }
        
        const data = await response.json();
        games = data.games || [];
        
        // Update game list UI
        renderGameList();
        
        // Update active game if needed
        if (activeGame) {
            const updatedGame = games.find(g => g.id === activeGame.id);
            if (updatedGame) {
                activeGame = updatedGame;
                renderActiveGame();
            }
        }
        
    } catch (error) {
        console.error('Error loading games:', error);
    }
}

function renderGameList() {
    gameListElement.innerHTML = '';
    
    if (games.length === 0) {
        gameListElement.innerHTML = '<div class="game-item">No games available</div>';
        return;
    }
    
    games.forEach(game => {
        const gameItem = document.createElement('div');
        gameItem.className = 'game-item';
        
        // Create game status text
        let statusText = '';
        if (game.state.WaitingForPlayers) {
            statusText = 'Waiting for players';
        } else if (game.state.InProgress) {
            statusText = `In Progress - ${game.state.InProgress.current_turn}'s turn`;
        } else if (game.state.Finished) {
            if (game.state.Finished.winner) {
                statusText = `Finished - ${game.state.Finished.winner} won`;
            } else {
                statusText = 'Finished - Draw';
            }
        }
        
        // Display creator and status
        gameItem.innerHTML = `
            <div>Game #${game.id.substring(0, 8)}...</div>
            <div>Creator: ${game.creator.substring(0, 8)}...</div>
            <div>Status: ${statusText}</div>
            <div>Players: ${game.players.length}/2</div>
        `;
        
        gameItem.addEventListener('click', () => selectGame(game));
        gameListElement.appendChild(gameItem);
    });
}

function selectGame(game) {
    activeGame = game;
    renderActiveGame();
    activeGameElement.style.display = 'block';
}

function renderActiveGame() {
    if (!activeGame) return;
    
    gameIdElement.textContent = activeGame.id.substring(0, 8);
    
    // Update status
    let statusText = '';
    let canPlay = false;
    let currentPlayer = null;
    
    if (activeGame.state.WaitingForPlayers) {
        statusText = 'Waiting for players to join';
        // Check if the current user is not the creator
        if (activeGame.creator !== currentOwner && activeGame.players.length < 2) {
            statusText += ' - <button id="join-game">Join Game</button>';
        }
    } else if (activeGame.state.InProgress) {
        const currentTurn = activeGame.state.InProgress.current_turn;
        statusText = `In Progress - ${currentTurn}'s turn`;
        
        // Check if it's the current user's turn
        const playerEntry = activeGame.players.find(p => p[0] === currentOwner);
        if (playerEntry && playerEntry[1] === currentTurn) {
            canPlay = true;
            currentPlayer = currentTurn;
        }
    } else if (activeGame.state.Finished) {
        if (activeGame.state.Finished.winner) {
            statusText = `Game Over - ${activeGame.state.Finished.winner} won!`;
        } else {
            statusText = 'Game Over - Draw!';
        }
    }
    
    gameStatusElement.innerHTML = statusText;
    
    // Set up join button if present
    const joinButton = document.getElementById('join-game');
    if (joinButton) {
        joinButton.addEventListener('click', joinGame);
    }
    
    // Render board
    renderBoard(activeGame.board, canPlay, currentPlayer);
}

function renderBoard(board, canPlay, currentPlayer) {
    gameBoardElement.innerHTML = '';
    
    for (let row = 0; row < 3; row++) {
        for (let col = 0; col < 3; col++) {
            const cell = document.createElement('div');
            cell.className = 'cell';
            cell.dataset.row = row;
            cell.dataset.col = col;
            
            const cellValue = board[row][col];
            if (cellValue) {
                cell.textContent = cellValue === 'X' ? 'X' : 'O';
                cell.classList.add(cellValue === 'X' ? 'cell-x' : 'cell-o');
            } else if (canPlay) {
                cell.addEventListener('click', () => makeMove(row, col));
            }
            
            gameBoardElement.appendChild(cell);
        }
    }
}

async function createGame() {
    try {
        // In a real application, you would call the contract with the CreateGame operation
        const response = await fetch(`${walletUrl}/operations/${currentChainId}/tictactoe`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                operation: { CreateGame: {} }
            })
        });
        
        if (!response.ok) {
            throw new Error('Failed to create game');
        }
        
        alert('Game created successfully!');
        await loadGames();
        
    } catch (error) {
        console.error('Error creating game:', error);
        alert(`Failed to create game: ${error.message}`);
    }
}

async function joinGame() {
    try {
        // In a real application, you would call the contract with the JoinGame operation
        const response = await fetch(`${walletUrl}/operations/${currentChainId}/tictactoe`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                operation: { 
                    JoinGame: { 
                        game_id: activeGame.id 
                    } 
                }
            })
        });
        
        if (!response.ok) {
            throw new Error('Failed to join game');
        }
        
        alert('Joined game successfully!');
        await loadGames();
        
    } catch (error) {
        console.error('Error joining game:', error);
        alert(`Failed to join game: ${error.message}`);
    }
}

async function makeMove(row, col) {
    try {
        // In a real application, you would call the contract with the MakeMove operation
        const response = await fetch(`${walletUrl}/operations/${currentChainId}/tictactoe`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                operation: { 
                    MakeMove: { 
                        game_id: activeGame.id,
                        cell: { row, col }
                    } 
                }
            })
        });
        
        if (!response.ok) {
            throw new Error('Failed to make move');
        }
        
        await loadGames();
        
    } catch (error) {
        console.error('Error making move:', error);
        alert(`Failed to make move: ${error.message}`);
    }
}