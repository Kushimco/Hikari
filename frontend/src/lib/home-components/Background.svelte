<script lang="ts">
  // No logic required
</script>

<div class="ambient-container">
  <div class="blob light-warm"></div>
  <div class="blob deep-peach"></div>
  <div class="blob soft-pink"></div>
</div>

<style>
  /* #region --- CONTAINER & SHARED --- */
  .ambient-container { 
    position: fixed; inset: 0; z-index: 0; pointer-events: none; 
    overflow: hidden; background: transparent; 
  }

  .blob { 
    position: absolute; border-radius: 50%; filter: blur(120px); 
    opacity: 0.4; will-change: transform; 
  }
  /* #endregion */

  /* #region --- BLOB DEFINITIONS --- */
  .light-warm { 
    top: -20%; left: -10%; width: 80vw; height: 80vw; 
    max-width: 800px; max-height: 800px; background: #fff5eb; 
    animation: floatBreathing 10s infinite ease-in-out alternate; 
  }

  .deep-peach { 
    bottom: -20%; right: -10%; width: 70vw; height: 70vw; 
    max-width: 700px; max-height: 700px; background: rgba(254, 214, 169, 0.5); 
    animation: floatDrift 10s infinite linear; 
  }

  .soft-pink { 
    top: 40%; left: -10%; width: 50vw; height: 50vw; 
    max-width: 500px; max-height: 500px; background: rgba(255, 189, 245, 0.3); 
    opacity: 0.3; animation: floatWander 10s infinite ease-in-out; 
  }
  /* #endregion */

  /* #region --- ANIMATIONS --- */
  @keyframes floatBreathing {
    0%   { transform: translate(0, 0) scale(1); }
    50%  { transform: translate(30px, 40px) scale(1.1); }
    100% { transform: translate(-20px, 10px) scale(0.95); }
  }

  @keyframes floatDrift {
    0%   { transform: translate(0, 0) rotate(0deg); }
    33%  { transform: translate(-40px, -50px) rotate(10deg); }
    66%  { transform: translate(20px, -30px) rotate(-5deg); }
    100% { transform: translate(0, 0) rotate(0deg); }
  }

  @keyframes floatWander {
    0%   { transform: translate(0, 0) scale(1); }
    25%  { transform: translate(50px, -40px) scale(1.1, 0.9); }
    50%  { transform: translate(20px, 60px) scale(0.95, 1.05); }
    75%  { transform: translate(-30px, 20px) scale(1.05, 0.95); }
    100% { transform: translate(0, 0) scale(1); }
  }
  /* #endregion */
</style>
