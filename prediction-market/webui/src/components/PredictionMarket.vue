<template>
  <div class="page-container">
    <!-- Hero Section -->
    <header class="hero-section fade-in">
      <h1 class="text-hero">Prediction Markets</h1>
      <p class="text-body" style="max-width: 600px;">
        Decentralized prediction markets on Linera blockchain. Bet on future outcomes and win rewards.
      </p>
    </header>

    <!-- Stats Section -->
    <div class="grid grid-3" style="margin: 3rem 0;">
      <div class="stat-card">
        <div class="stat-value">{{ totalMarkets }}</div>
        <div class="stat-label">Total Markets</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{{ userBalance }}</div>
        <div class="stat-label">Your Balance</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{{ activeMarkets.length }}</div>
        <div class="stat-label">Active Markets</div>
      </div>
    </div>

    <!-- Connection Status -->
    <div v-if="!connected" class="card-glass" style="margin-bottom: 2rem;">
      <div style="display: flex; justify-content: space-between; align-items: center;">
        <div>
          <h3 class="text-h3" style="margin: 0;">Connect Wallet</h3>
          <p class="text-small" style="margin: 0.5rem 0 0 0;">Connect your Linera wallet to start trading</p>
        </div>
        <button class="btn btn-primary" @click="connectWallet">
          <span class="material-icons">account_balance_wallet</span>
          Connect
        </button>
      </div>
    </div>

    <!-- Connected Status -->
    <div v-else class="card-glass" style="margin-bottom: 2rem;">
      <div style="display: flex; justify-content: space-between; align-items: center;">
        <div>
          <h3 class="text-h3" style="margin: 0;">Connected</h3>
          <p class="text-small" style="margin: 0.5rem 0 0 0; word-break: break-all;">
            {{ currentChain?.substring(0, 40) }}...
          </p>
        </div>
        <button class="btn btn-primary" @click="showCreateMarketDialog = true">
          <span class="material-icons">add</span>
          Create Market
        </button>
      </div>
    </div>

    <!-- Market Filters -->
    <div style="margin-bottom: 2rem;">
      <div style="display: flex; gap: 1rem;">
        <button
          :class="['btn', filter === 'all' ? 'btn-primary' : 'btn-ghost']"
          @click="filter = 'all'"
        >
          All Markets
        </button>
        <button
          :class="['btn', filter === 'active' ? 'btn-primary' : 'btn-ghost']"
          @click="filter = 'active'"
        >
          Active
        </button>
        <button
          :class="['btn', filter === 'resolved' ? 'btn-primary' : 'btn-ghost']"
          @click="filter = 'resolved'"
        >
          Resolved
        </button>
      </div>
    </div>

    <!-- Markets List -->
    <div v-if="loading" class="grid grid-2">
      <div v-for="i in 4" :key="i" class="skeleton" style="height: 300px; border-radius: 1rem;"></div>
    </div>

    <div v-else-if="filteredMarkets.length === 0" class="card" style="text-align: center; padding: 3rem;">
      <span class="material-icons" style="font-size: 4rem; color: var(--color-text-tertiary);">
        search_off
      </span>
      <h3 class="text-h3">No Markets Found</h3>
      <p class="text-body">Be the first to create a prediction market!</p>
      <button class="btn btn-primary" @click="showCreateMarketDialog = true" style="margin-top: 1rem;">
        Create Market
      </button>
    </div>

    <div v-else class="grid grid-2">
      <div
        v-for="market in filteredMarkets"
        :key="market.id"
        class="market-card"
        @click="selectedMarket = market"
      >
        <div class="market-card-header">
          <div style="display: flex; justify-content: space-between; align-items: start; margin-bottom: 0.5rem;">
            <span :class="['badge', market.status === 'Active' ? 'badge-active' : 'badge-resolved']">
              {{ market.status === 'Active' ? 'Active' : 'Resolved' }}
            </span>
            <span class="text-small">Pool: {{ market.total_pool }}</span>
          </div>
          <h3 class="market-question">{{ market.question }}</h3>
          <div class="market-meta">
            <span>
              <span class="material-icons" style="font-size: 1rem; vertical-align: middle;">schedule</span>
              {{ formatDate(market.resolution_time) }}
            </span>
            <span>
              <span class="material-icons" style="font-size: 1rem; vertical-align: middle;">people</span>
              {{ market.outcomes.length }} outcomes
            </span>
          </div>
        </div>

        <div class="market-outcomes">
          <div
            v-for="(outcome, index) in market.outcomes"
            :key="index"
            class="outcome-button"
          >
            <span class="outcome-name">{{ outcome }}</span>
            <span class="outcome-pool">{{ market.outcome_pools[index] || 0 }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Market Dialog -->
    <q-dialog v-model="showCreateMarketDialog">
      <q-card style="min-width: 500px; background: var(--color-bg-secondary); color: white;">
        <q-card-section>
          <div class="text-h2">Create New Market</div>
        </q-card-section>

        <q-card-section>
          <div style="display: flex; flex-direction: column; gap: 1rem;">
            <div>
              <label class="text-body" style="display: block; margin-bottom: 0.5rem;">Question</label>
              <input
                v-model="newMarket.question"
                class="input"
                placeholder="Will Bitcoin reach $100k in 2025?"
              />
            </div>

            <div>
              <label class="text-body" style="display: block; margin-bottom: 0.5rem;">
                Outcomes (comma separated)
              </label>
              <input
                v-model="newMarket.outcomesStr"
                class="input"
                placeholder="Yes, No"
              />
            </div>

            <div>
              <label class="text-body" style="display: block; margin-bottom: 0.5rem;">
                Resolution Date
              </label>
              <input
                v-model="newMarket.resolutionDate"
                type="datetime-local"
                class="input"
              />
            </div>

            <div>
              <label class="text-body" style="display: block; margin-bottom: 0.5rem;">
                Oracle URL (optional)
              </label>
              <input
                v-model="newMarket.oracleUrl"
                class="input"
                placeholder="https://api.example.com/result"
              />
            </div>
          </div>
        </q-card-section>

        <q-card-actions align="right">
          <button class="btn btn-ghost" @click="showCreateMarketDialog = false">Cancel</button>
          <button class="btn btn-primary" @click="createMarket" :disabled="creating">
            {{ creating ? 'Creating...' : 'Create Market' }}
          </button>
        </q-card-actions>
      </q-card>
    </q-dialog>

    <!-- Market Detail Dialog -->
    <q-dialog v-model="showMarketDetail" v-if="selectedMarket">
      <q-card style="min-width: 600px; background: var(--color-bg-secondary); color: white;">
        <q-card-section>
          <div style="display: flex; justify-content: between; align-items: start;">
            <div style="flex: 1;">
              <span :class="['badge', selectedMarket.status === 'Active' ? 'badge-active' : 'badge-resolved']">
                {{ selectedMarket.status === 'Active' ? 'Active' : 'Resolved' }}
              </span>
              <h2 class="text-h2" style="margin-top: 0.5rem;">{{ selectedMarket.question }}</h2>
            </div>
          </div>

          <div class="market-meta" style="margin-top: 1rem;">
            <span>Total Pool: {{ selectedMarket.total_pool }}</span>
            <span>Resolves: {{ formatDate(selectedMarket.resolution_time) }}</span>
          </div>
        </q-card-section>

        <q-card-section>
          <h3 class="text-h3">Place Your Bet</h3>
          <div class="market-outcomes">
            <div
              v-for="(outcome, index) in selectedMarket.outcomes"
              :key="index"
              :class="['outcome-button', selectedOutcome === index ? 'selected' : '']"
              @click="selectedOutcome = index"
            >
              <div>
                <div class="outcome-name">{{ outcome }}</div>
                <div class="outcome-pool">Pool: {{ selectedMarket.outcome_pools[index] || 0 }}</div>
              </div>
              <span v-if="selectedOutcome === index" class="material-icons" style="color: var(--color-accent);">
                check_circle
              </span>
            </div>
          </div>

          <div style="margin-top: 1.5rem;">
            <label class="text-body" style="display: block; margin-bottom: 0.5rem;">Bet Amount</label>
            <input
              v-model.number="betAmount"
              type="number"
              class="input"
              placeholder="Enter amount"
              min="1"
            />
          </div>
        </q-card-section>

        <q-card-actions align="right">
          <button class="btn btn-ghost" @click="closeMarketDetail">Close</button>
          <button
            v-if="selectedMarket.status === 'Active'"
            class="btn btn-primary"
            @click="placeBet"
            :disabled="betting || selectedOutcome === null || !betAmount"
          >
            {{ betting ? 'Placing Bet...' : 'Place Bet' }}
          </button>
          <button
            v-else
            class="btn btn-primary"
            @click="claimWinnings"
            :disabled="claiming"
          >
            {{ claiming ? 'Claiming...' : 'Claim Winnings' }}
          </button>
        </q-card-actions>
      </q-card>
    </q-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useQuasar } from 'quasar';
import axios from 'axios';

const $q = useQuasar();

// State
const connected = ref(false);
const currentChain = ref<string | null>(null);
const loading = ref(false);
const markets = ref<any[]>([]);
const activeMarkets = ref<any[]>([]);
const totalMarkets = ref(0);
const userBalance = ref(0);
const filter = ref('all');

// Dialogs
const showCreateMarketDialog = ref(false);
const showMarketDetail = computed({
  get: () => selectedMarket.value !== null,
  set: (val) => { if (!val) selectedMarket.value = null; }
});
const selectedMarket = ref<any>(null);
const selectedOutcome = ref<number | null>(null);
const betAmount = ref<number>(10);

// Loading states
const creating = ref(false);
const betting = ref(false);
const claiming = ref(false);

// New market form
const newMarket = ref({
  question: '',
  outcomesStr: 'Yes, No',
  resolutionDate: '',
  oracleUrl: ''
});

// Computed
const filteredMarkets = computed(() => {
  if (filter.value === 'all') return markets.value;
  if (filter.value === 'active') return markets.value.filter(m => m.status === 'Active');
  if (filter.value === 'resolved') return markets.value.filter(m => m.status !== 'Active');
  return markets.value;
});

let pollInterval: any = null;

// Methods
async function connectWallet() {
  try {
    if (typeof window.linera === 'undefined') {
      $q.notify({
        type: 'negative',
        message: 'Linera wallet not found. Please install a Linera-compatible wallet.',
      });
      return;
    }

    const accounts = await window.linera.request({ method: 'linera_requestAccounts' });
    if (accounts && accounts.length > 0) {
      currentChain.value = accounts[0];
      connected.value = true;
      startPolling();
      await fetchData();
      $q.notify({
        type: 'positive',
        message: 'Wallet connected!',
      });
    }
  } catch (error: any) {
    $q.notify({
      type: 'negative',
      message: 'Failed to connect wallet: ' + error.message,
    });
  }
}

async function fetchData() {
  try {
    loading.value = true;

    // Fetch markets
    const marketsQuery = `
      query {
        markets {
          id
          creator
          question
          outcomes
          resolution_time
          oracle_url
          status
          total_pool
          outcome_pools
        }
        active_markets {
          id
        }
        total_markets
      }
    `;

    const result = await executeGraphQLQuery(marketsQuery);
    if (result.data) {
      markets.value = result.data.markets || [];
      activeMarkets.value = result.data.active_markets || [];
      totalMarkets.value = result.data.total_markets || 0;
    }

    // Fetch user balance if connected
    if (connected.value && currentChain.value) {
      const balanceQuery = `
        query {
          balance(owner: "${currentChain.value}")
        }
      `;
      const balanceResult = await executeGraphQLQuery(balanceQuery);
      if (balanceResult.data) {
        userBalance.value = balanceResult.data.balance || 0;
      }
    }
  } catch (error) {
    console.error('Failed to fetch data:', error);
  } finally {
    loading.value = false;
  }
}

async function executeGraphQLQuery(query: string) {
  const applicationId = localStorage.getItem('applicationId') || 'YOUR_APP_ID';

  // For development, use direct HTTP to Linera service
  const chainId = currentChain.value || 'default';
  const url = `http://localhost:8080/chains/${chainId}/applications/${applicationId}`;

  const response = await axios.post(url, { query });
  return response.data;
}

async function createMarket() {
  if (!newMarket.value.question || !newMarket.value.outcomesStr || !newMarket.value.resolutionDate) {
    $q.notify({
      type: 'warning',
      message: 'Please fill in all required fields',
    });
    return;
  }

  creating.value = true;
  try {
    const outcomes = newMarket.value.outcomesStr.split(',').map(s => s.trim());
    const resolutionTime = new Date(newMarket.value.resolutionDate).getTime() * 1000; // Convert to microseconds

    const mutation = `
      mutation {
        createMarket(
          question: "${newMarket.value.question}",
          outcomes: ${JSON.stringify(outcomes)},
          resolutionTime: ${resolutionTime},
          oracleUrl: ${newMarket.value.oracleUrl ? `"${newMarket.value.oracleUrl}"` : 'null'}
        )
      }
    `;

    await executeGraphQLQuery(mutation);

    $q.notify({
      type: 'positive',
      message: 'Market created successfully!',
    });

    showCreateMarketDialog.value = false;
    newMarket.value = {
      question: '',
      outcomesStr: 'Yes, No',
      resolutionDate: '',
      oracleUrl: ''
    };

    await fetchData();
  } catch (error: any) {
    $q.notify({
      type: 'negative',
      message: 'Failed to create market: ' + error.message,
    });
  } finally {
    creating.value = false;
  }
}

async function placeBet() {
  if (selectedOutcome.value === null || !betAmount.value) return;

  betting.value = true;
  try {
    const mutation = `
      mutation {
        placeBet(
          marketId: ${selectedMarket.value.id},
          outcomeIndex: ${selectedOutcome.value},
          amount: ${betAmount.value}
        )
      }
    `;

    await executeGraphQLQuery(mutation);

    $q.notify({
      type: 'positive',
      message: 'Bet placed successfully!',
    });

    closeMarketDetail();
    await fetchData();
  } catch (error: any) {
    $q.notify({
      type: 'negative',
      message: 'Failed to place bet: ' + error.message,
    });
  } finally {
    betting.value = false;
  }
}

async function claimWinnings() {
  claiming.value = true;
  try {
    const mutation = `
      mutation {
        claimWinnings(marketId: ${selectedMarket.value.id})
      }
    `;

    await executeGraphQLQuery(mutation);

    $q.notify({
      type: 'positive',
      message: 'Winnings claimed!',
    });

    closeMarketDetail();
    await fetchData();
  } catch (error: any) {
    $q.notify({
      type: 'negative',
      message: 'Failed to claim winnings: ' + error.message,
    });
  } finally {
    claiming.value = false;
  }
}

function closeMarketDetail() {
  selectedMarket.value = null;
  selectedOutcome.value = null;
  betAmount.value = 10;
}

function formatDate(timestamp: number) {
  const date = new Date(timestamp / 1000); // Convert from microseconds
  return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

function startPolling() {
  pollInterval = setInterval(async () => {
    await fetchData();
  }, 5000);
}

onMounted(() => {
  fetchData();
});

onUnmounted(() => {
  if (pollInterval) {
    clearInterval(pollInterval);
  }
});

// Extend window interface
declare global {
  interface Window {
    linera: any;
  }
}
</script>

<style scoped>
.hero-section {
  margin-bottom: 3rem;
}
</style>
