<script lang="ts">
  // No logic needed, but required for TS type inference
</script>

<div class="ambient-light one"></div>
<div class="ambient-light two"></div>

<style>
  /* ... your existing styles ... */
  .ambient-light {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    opacity: 0.4;
    z-index: 0;
    animation: floatBlob 20s infinite ease-in-out alternate;
  }

  .one {
    width: 600px;
    height: 600px;
    background: #ffffff;
    top: -200px;
    left: -100px;
  }

  .two {
    width: 500px;
    height: 500px;
    background: #ffd1bc;
    bottom: -150px;
    right: -100px;
  }

  @keyframes floatBlob {
    0% {
      transform: translate(0, 0);
    }
    100% {
      transform: translate(40px, 60px);
    }
  }
</style>
