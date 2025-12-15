<template>
  <q-layout view="hHh lpR fFf">
    <q-header elevated style="background: var(--color-bg-secondary); border-bottom: 1px solid var(--color-border);">
      <q-toolbar>
        <q-toolbar-title style="display: flex; align-items: center; gap: 0.5rem;">
          <span class="material-icons" style="font-size: 2rem; color: var(--color-accent);">
            trending_up
          </span>
          <span style="font-weight: 700;">Linera Prediction Markets</span>
        </q-toolbar-title>

        <q-btn
          flat
          round
          dense
          icon="account_balance_wallet"
          style="color: var(--color-text-primary);"
        />
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>

    <q-footer style="background: var(--color-bg-secondary); border-top: 1px solid var(--color-border); padding: 2rem;">
      <div style="max-width: 1200px; margin: 0 auto; text-align: center;">
        <p class="text-small">
          Built on Linera Blockchain • Decentralized Prediction Markets
        </p>
      </div>
    </q-footer>
  </q-layout>
</template>

<script setup lang="ts">
// Main layout component
</script>
