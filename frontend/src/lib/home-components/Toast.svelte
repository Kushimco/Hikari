<script lang="ts">
  import { fly } from 'svelte/transition';

  // Props to customize the message
  export let title = "Notification";
  export let message = "";
</script>

<div 
  class="toast-overlay" 
  transition:fly={{ y: 30, duration: 400, opacity: 0 }}
>
  <div class="toast-card">
    <div class="toast-icon">!</div>
    <div class="toast-content">
      <h4>{title}</h4>
      <p>{message}</p>
    </div>
  </div>
</div>

<style>
  /* Position fixed at bottom center */
  .toast-overlay {
    position: fixed;
    bottom: 30px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 1000;
    pointer-events: none; /* Allows clicks to pass through around the toast */
  }

  /* The actual toast card */
  .toast-card {
    background: rgba(255, 255, 255, 0.85);
    border-radius: 16px;
    padding: 14px 20px;
    display: flex;
    align-items: center;
    gap: 14px;
    
    /* Soft shadows and glass effect */
    box-shadow: 
      0 10px 30px rgba(200, 100, 100, 0.15),
      inset 0 0 20px rgba(255, 255, 255, 0.8);
    border: 1px solid rgba(255, 200, 200, 0.5);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    
    min-width: 300px;
    pointer-events: auto; /* The card itself is interactive if needed */
  }

  /* Icon Circle */
  .toast-icon {
    width: 32px;
    height: 32px;
    background: linear-gradient(135deg, #ff9a9e, #fecfef);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #fff;
    font-weight: 800;
    font-size: 1.1rem;
    box-shadow: 0 4px 10px rgba(255, 154, 158, 0.4);
    flex-shrink: 0;
  }

  /* Text Content */
  .toast-content {
    display: flex;
    flex-direction: column;
  }

  .toast-content h4 {
    margin: 0;
    font-size: 0.95rem;
    font-weight: 700;
    color: #8e4a4a;
  }

  .toast-content p {
    margin: 2px 0 0;
    font-size: 0.85rem;
    color: #8e4a4a;
    opacity: 0.8;
  }
</style>
