<template>
  <div class="game-container">
    <q-card class="q-pa-md">
      <q-card-section>
        <div class="text-h4 text-center">🪨📄✂️ Rock Paper Scissors</div>
        <div class="text-subtitle2 text-center text-grey-7">
          Blockchain PvP Game on Linera
        </div>
      </q-card-section>

      <!-- Connection Status -->
      <q-card-section>
        <q-banner v-if="!connected" class="bg-warning text-white">
          <template v-slot:avatar>
            <q-icon name="warning" />
          </template>
          Please connect your wallet to play
          <template v-slot:action>
            <q-btn flat label="Connect" @click="connectWallet" />
          </template>
        </q-banner>

        <q-banner v-else class="bg-positive text-white">
          <template v-slot:avatar>
            <q-icon name="check_circle" />
          </template>
          Connected: {{ currentChain?.substring(0, 20) }}...
        </q-banner>
      </q-card-section>

      <!-- Game Lobby -->
      <q-card-section v-if="connected && gameStatus === 'Idle'">
        <div class="text-h6 text-center q-mb-md">Game Lobby</div>
        <div class="row justify-center q-gutter-md">
          <q-btn
            color="primary"
            size="lg"
            label="Create New Game"
            icon="add"
            @click="createGame"
            :loading="loading"
          />
          <q-btn
            color="secondary"
            size="lg"
            label="Join Game"
            icon="login"
            @click="showJoinDialog = true"
            :loading="loading"
          />
        </div>
      </q-card-section>

      <!-- Waiting for Opponent -->
      <q-card-section v-if="gameStatus === 'WaitingForOpponent'">
        <div class="text-center">
          <q-spinner-dots size="50px" color="primary" />
          <div class="text-h6 q-mt-md">Waiting for opponent...</div>
          <div class="text-subtitle2 text-grey-7 chain-info q-mt-md">
            Share your chain ID: <strong>{{ currentChain }}</strong>
          </div>
          <q-btn
            flat
            color="negative"
            label="Cancel"
            class="q-mt-md"
            @click="leaveGame"
          />
        </div>
      </q-card-section>

      <!-- Active Game -->
      <q-card-section v-if="gameStatus !== 'Idle' && gameStatus !== 'WaitingForOpponent'">
        <!-- Score Board -->
        <div class="row justify-center q-gutter-md q-mb-lg">
          <q-card class="score-card bg-blue-1">
            <q-card-section>
              <div class="text-h6">Player 1</div>
              <div class="text-caption chain-info">{{ player1?.chain?.substring(0, 20) }}...</div>
              <div class="text-h3 text-primary">{{ player1?.rounds_won || 0 }}</div>
            </q-card-section>
          </q-card>

          <q-card class="score-card bg-orange-1">
            <q-card-section>
              <div class="text-h6">Player 2</div>
              <div class="text-caption chain-info">{{ player2?.chain?.substring(0, 20) }}...</div>
              <div class="text-h3 text-orange">{{ player2?.rounds_won || 0 }}</div>
            </q-card-section>
          </q-card>
        </div>

        <!-- Game Status -->
        <div class="game-status text-center">
          {{ getStatusMessage() }}
        </div>

        <!-- Move Selection (when waiting for moves) -->
        <div v-if="gameStatus === 'WaitingForMoves' && !myMoveCommitted" class="text-center q-mt-lg">
          <div class="text-h6 q-mb-md">Choose Your Move</div>
          <div class="row justify-center">
            <q-btn
              class="move-button"
              color="grey-7"
              round
              @click="selectMove('Rock')"
              :loading="loading"
            >
              🪨
            </q-btn>
            <q-btn
              class="move-button"
              color="grey-7"
              round
              @click="selectMove('Paper')"
              :loading="loading"
            >
              📄
            </q-btn>
            <q-btn
              class="move-button"
              color="grey-7"
              round
              @click="selectMove('Scissors')"
              :loading="loading"
            >
              ✂️
            </q-btn>
          </div>
        </div>

        <!-- Waiting for Reveal -->
        <div v-if="gameStatus === 'WaitingForReveals' && !myMoveRevealed" class="text-center q-mt-lg">
          <q-btn
            color="primary"
            size="lg"
            label="Reveal Move"
            @click="revealMove"
            :loading="loading"
          />
        </div>

        <!-- Round Complete -->
        <div v-if="gameStatus === 'RoundComplete'" class="text-center q-mt-lg">
          <div class="text-h5 q-mb-md">Round Result</div>

          <!-- Show moves -->
          <div class="row justify-center q-mb-lg">
            <div class="text-center q-mx-md">
              <div class="text-subtitle2">Player 1</div>
              <div class="move-display">{{ getMoveEmoji(lastRoundP1Move) }}</div>
            </div>
            <div class="text-h3 q-mx-md self-center">VS</div>
            <div class="text-center q-mx-md">
              <div class="text-subtitle2">Player 2</div>
              <div class="move-display">{{ getMoveEmoji(lastRoundP2Move) }}</div>
            </div>
          </div>

          <!-- Winner announcement -->
          <div v-if="lastRoundWinner" class="winner-announcement text-positive q-mb-md">
            🎉 {{ getWinnerName() }} Wins! 🎉
          </div>
          <div v-else class="text-h6 text-grey-7 q-mb-md">
            It's a Tie!
          </div>

          <q-btn
            color="primary"
            size="lg"
            label="Play Next Round"
            @click="playAgain"
            :loading="loading"
          />
          <q-btn
            flat
            color="negative"
            label="Leave Game"
            class="q-ml-md"
            @click="leaveGame"
          />
        </div>
      </q-card-section>

      <!-- Total Rounds -->
      <q-card-section v-if="totalRounds > 0" class="text-center">
        <div class="text-caption text-grey-7">
          Total Rounds Played: {{ totalRounds }}
        </div>
      </q-card-section>
    </q-card>

    <!-- Join Game Dialog -->
    <q-dialog v-model="showJoinDialog">
      <q-card style="min-width: 400px">
        <q-card-section>
          <div class="text-h6">Join Game</div>
        </q-card-section>

        <q-card-section>
          <q-input
            v-model="opponentChainId"
            label="Opponent's Chain ID"
            hint="Enter the chain ID of the player who created the game"
            :rules="[val => !!val || 'Chain ID is required']"
          />
        </q-card-section>

        <q-card-actions align="right">
          <q-btn flat label="Cancel" color="grey-7" v-close-popup />
          <q-btn
            label="Join"
            color="primary"
            @click="joinGame"
            :loading="loading"
          />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useQuasar } from 'quasar';
import { graphql } from '../wasm/rock_paper_scissors_wasm';

const $q = useQuasar();

// State
const connected = ref(false);
const currentChain = ref<string | null>(null);
const loading = ref(false);
const gameStatus = ref('Idle');
const player1 = ref<any>(null);
const player2 = ref<any>(null);
const totalRounds = ref(0);
const lastRoundWinner = ref<string | null>(null);
const lastRoundP1Move = ref<string | null>(null);
const lastRoundP2Move = ref<string | null>(null);
const myMoveCommitted = ref(false);
const myMoveRevealed = ref(false);
const selectedMove = ref<string | null>(null);
const moveSecret = ref<string | null>(null);
const showJoinDialog = ref(false);
const opponentChainId = ref('');

// Subscription
let subscriptionInterval: any = null;

// Connect wallet
async function connectWallet() {
  try {
    if (typeof window.linera === 'undefined') {
      $q.notify({
        type: 'negative',
        message: 'Linera wallet not found. Please install CheCko or MetaMask with Linera support.',
      });
      return;
    }

    const accounts = await window.linera.request({ method: 'linera_requestAccounts' });
    if (accounts && accounts.length > 0) {
      currentChain.value = accounts[0];
      connected.value = true;
      startPolling();
      $q.notify({
        type: 'positive',
        message: 'Wallet connected successfully!',
      });
    }
  } catch (error: any) {
    $q.notify({
      type: 'negative',
      message: 'Failed to connect wallet: ' + error.message,
    });
  }
}

// Polling for game state updates
async function startPolling() {
  // Poll every 2 seconds
  subscriptionInterval = setInterval(async () => {
    await fetchGameState();
  }, 2000);
}

async function fetchGameState() {
  try {
    const query = `
      query {
        gameStatus
        player1 { owner chain rounds_won }
        player2 { owner chain rounds_won }
        totalRounds
        lastRoundWinner
        lastRoundP1Move
        lastRoundP2Move
        bothCommitted
        bothRevealed
      }
    `;

    const response = await executeGraphQLQuery(query);
    if (response.data) {
      gameStatus.value = response.data.gameStatus || 'Idle';
      player1.value = response.data.player1;
      player2.value = response.data.player2;
      totalRounds.value = response.data.totalRounds || 0;
      lastRoundWinner.value = response.data.lastRoundWinner;
      lastRoundP1Move.value = response.data.lastRoundP1Move;
      lastRoundP2Move.value = response.data.lastRoundP2Move;

      // Update move status
      if (response.data.bothCommitted) {
        myMoveCommitted.value = true;
      }
      if (response.data.bothRevealed) {
        myMoveRevealed.value = true;
      }
    }
  } catch (error) {
    console.error('Failed to fetch game state:', error);
  }
}

async function executeGraphQLQuery(query: string) {
  const applicationId = localStorage.getItem('applicationId');
  if (!applicationId) {
    throw new Error('Application ID not found');
  }

  const result = await window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId,
      query: { query },
    },
  });

  return result;
}

async function executeMutation(mutationName: string, variables: any = {}) {
  // Use WASM to serialize the operation
  const wasmQuery = buildWasmMutation(mutationName, variables);
  const wasmResult = await graphql(wasmQuery);
  const parsed = JSON.parse(wasmResult);

  if (parsed.errors) {
    throw new Error(parsed.errors[0].message);
  }

  const operationBytes = parsed.data[mutationName];

  // Execute via Linera provider
  const applicationId = localStorage.getItem('applicationId');
  const result = await window.linera.request({
    method: 'linera_executeOperation',
    params: {
      applicationId,
      operation: operationBytes,
    },
  });

  return result;
}

function buildWasmMutation(name: string, vars: any): string {
  const args = Object.entries(vars)
    .map(([key, value]) => `${key}: "${value}"`)
    .join(', ');

  return `mutation { ${name}${args ? `(${args})` : ''} }`;
}

async function createGame() {
  loading.value = true;
  try {
    await executeMutation('create_game');
    myMoveCommitted.value = false;
    myMoveRevealed.value = false;
    $q.notify({
      type: 'positive',
      message: 'Game created! Waiting for opponent...',
    });
    await fetchGameState();
  } catch (error: any) {
    $q.notify({
      type: 'negative',
      message: 'Failed to create game: ' + error.message,
    });
  } finally {
    loading.value = false;
  }
}

async function joinGame() {
  if (!opponentChainId.value) {
    $q.notify({ type: 'warning', message: 'Please enter opponent chain ID' });
    return;
  }

  loading.value = true;
  try {
    await executeMutation('join_game', { opponent_chain: opponentChainId.value });
    showJoinDialog.value = false;
    myMoveCommitted.value = false;
    myMoveRevealed.value = false;
    $q.notify({
      type: 'positive',
      message: 'Joined game successfully!',
    });
    await fetchGameState();
  } catch (error: any) {
    $q.notify({
      type: 'negative',
      message: 'Failed to join game: ' + error.message,
    });
  } finally {
    loading.value = false;
  }
}

async function selectMove(move: string) {
  loading.value = true;
  try {
    selectedMove.value = move;
    // Generate random secret
    const secret = Math.random().toString(36).substring(2, 15);
    moveSecret.value = secret;

    // Commit the move
    const wasmQuery = `mutation { commit_move(game_move: "${move}", secret: "${secret}") { operation_bytes } }`;
    const wasmResult = await graphql(wasmQuery);
    const parsed = JSON.parse(wasmResult);

    if (parsed.errors) {
      throw new Error(parsed.errors[0].message);
    }

    const operationBytes = parsed.data.commit_move.operation_bytes;

    const applicationId = localStorage.getItem('applicationId');
    await window.linera.request({
      method: 'linera_executeOperation',
      params: {
        applicationId,
        operation: operationBytes,
      },
    });

    myMoveCommitted.value = true;
    $q.notify({
      type: 'positive',
      message: `Move committed: ${move}`,
    });
    await fetchGameState();
  } catch (error: any) {
    $q.notify({
      type: 'negative',
      message: 'Failed to commit move: ' + error.message,
    });
  } finally {
    loading.value = false;
  }
}

async function revealMove() {
  if (!selectedMove.value || !moveSecret.value) {
    $q.notify({ type: 'warning', message: 'No move to reveal' });
    return;
  }

  loading.value = true;
  try {
    await executeMutation('reveal_move', {
      game_move: selectedMove.value,
      secret: moveSecret.value,
    });

    myMoveRevealed.value = true;
    $q.notify({
      type: 'positive',
      message: 'Move revealed!',
    });
    await fetchGameState();
  } catch (error: any) {
    $q.notify({
      type: 'negative',
      message: 'Failed to reveal move: ' + error.message,
    });
  } finally {
    loading.value = false;
  }
}

async function playAgain() {
  loading.value = true;
  try {
    await executeMutation('play_again');
    myMoveCommitted.value = false;
    myMoveRevealed.value = false;
    selectedMove.value = null;
    moveSecret.value = null;
    $q.notify({
      type: 'positive',
      message: 'New round started!',
    });
    await fetchGameState();
  } catch (error: any) {
    $q.notify({
      type: 'negative',
      message: 'Failed to start new round: ' + error.message,
    });
  } finally {
    loading.value = false;
  }
}

async function leaveGame() {
  loading.value = true;
  try {
    await executeMutation('leave_game');
    myMoveCommitted.value = false;
    myMoveRevealed.value = false;
    selectedMove.value = null;
    moveSecret.value = null;
    $q.notify({
      type: 'info',
      message: 'Left the game',
    });
    await fetchGameState();
  } catch (error: any) {
    $q.notify({
      type: 'negative',
      message: 'Failed to leave game: ' + error.message,
    });
  } finally {
    loading.value = false;
  }
}

function getStatusMessage(): string {
  switch (gameStatus.value) {
    case 'WaitingForMoves':
      return myMoveCommitted.value ? 'Waiting for opponent to commit...' : 'Make your move!';
    case 'WaitingForReveals':
      return myMoveRevealed.value ? 'Waiting for opponent to reveal...' : 'Reveal your move!';
    case 'RoundComplete':
      return 'Round Complete!';
    default:
      return '';
  }
}

function getMoveEmoji(move: string | null): string {
  if (!move) return '?';
  switch (move) {
    case 'Rock': return '🪨';
    case 'Paper': return '📄';
    case 'Scissors': return '✂️';
    default: return '?';
  }
}

function getWinnerName(): string {
  if (!lastRoundWinner.value) return '';
  if (player1.value?.owner === lastRoundWinner.value) return 'Player 1';
  if (player2.value?.owner === lastRoundWinner.value) return 'Player 2';
  return 'Unknown';
}

onMounted(() => {
  // Try to auto-connect if already connected
  if (typeof window.linera !== 'undefined') {
    connectWallet();
  }
});

onUnmounted(() => {
  if (subscriptionInterval) {
    clearInterval(subscriptionInterval);
  }
});
</script>

<style scoped>
/* Component-specific styles are in app.css */
</style>
