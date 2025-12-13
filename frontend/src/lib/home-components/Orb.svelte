<script lang="ts">
  export let orbElement: HTMLDivElement | null = null;
  export let activeTab: "home" | "menu" | "settings";
  export let isReturning: boolean;
  export let returnStage: "idle" | "fading" | "bouncing_down" | "bouncing_up";
  export let isGlowing: boolean;
  export let shouldScale: boolean;
  export let isPulsing: boolean;
  export let isAdding: boolean;

  // When the orb is tiny (your “small-orb” state), boost glow slightly
  $: isTiny = isReturning && returnStage === "bouncing_down";
  $: glowTiny = isGlowing && isTiny;
</script>

<div
  class="orb-floater"
  class:expanded-floater={activeTab === "menu" || (isReturning && returnStage === "fading")}
>
  <div
    class="orb"
    bind:this={orbElement}
    class:expanded={activeTab === "menu" || (isReturning && returnStage === "fading")}
    class:small-orb={isTiny}
    class:glowing={isGlowing}
    class:small-glow={glowTiny}
    class:typing-scale={shouldScale}
    class:pulsing={isPulsing && activeTab === "home"}
    class:add-success={isAdding}
  >
    <slot />
  </div>
</div>

<style>
  /* Match this to Home.svelte's overlay fade time */
  :root {
    --glow-fade: 900ms;
    --glow-ease: cubic-bezier(0.33, 0, 0.67, 1);
  }

  .orb-floater {
    width: 780px;
    height: 780px;
    margin-left: -105px;
    display: flex;
    justify-content: center;
    align-items: center;
    will-change: transform;
    transition:
      transform 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      margin 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      width 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      height 1.5s cubic-bezier(0.25, 1, 0.5, 1);
  }

  .orb-floater.expanded-floater {
    width: 96%;
    height: 95%;
    margin-left: 0;
    padding: 20px;
    padding-right: 75px;
    box-sizing: border-box;
    transform: translateY(0);
  }

  @keyframes float {
    0% { transform: translateY(0px); }
    25% { transform: translateY(-12px); }
    75% { transform: translateY(12px); }
    100% { transform: translateY(0px); }
  }

  .orb {
    position: relative;
    overflow: visible;
    width: 100%;
    height: 100%;
    border-radius: 50%;
    background: linear-gradient(
      180deg,
      rgb(254, 214, 169) 0%,
      rgba(244, 202, 167, 0.9) 60%,
      rgba(255, 189, 245, 0.7) 90%
    );

    box-shadow:
      inset 2px 4px 20px rgba(255, 255, 255, 0.6),
      inset -2px -4px 30px rgba(0, 0, 0, 0.05),
      0 25px 60px rgba(219, 168, 172, 0.35);

    display: flex;
    justify-content: center;
    align-items: center;
    max-width: 780px;
    max-height: 780px;

    transition:
      /* Make glow-related shadow changes finish on the same schedule */
      box-shadow var(--glow-fade) var(--glow-ease),
      width 0.7s cubic-bezier(0.25, 1, 0.5, 1),
      height 0.7s cubic-bezier(0.25, 1, 0.5, 1),
      border-radius 0.7s cubic-bezier(0.25, 1, 0.5, 1),
      background 1.5s ease,
      transform 0.6s cubic-bezier(0.25, 0.8, 0.25, 1),
      filter var(--glow-fade) var(--glow-ease);

    animation: float 8s ease-in-out infinite;
  }

  .orb::before {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: inherit;

    box-shadow:
      inset 0 0 30px rgba(255, 255, 255, 0.9),
      0 0 120px rgba(255, 220, 180, 0.8),
      0 0 200px rgba(255, 200, 150, 0.4);

    opacity: 0;
    pointer-events: none;

    transition:
      opacity var(--glow-fade) var(--glow-ease),
      box-shadow var(--glow-fade) var(--glow-ease);

    z-index: -1;
  }

  .orb.glowing::before { opacity: 1; }

  .orb.glowing {
    box-shadow:
      inset 0 0 30px rgba(255, 255, 255, 0.9),
      0 0 120px rgba(255, 220, 180, 0.8),
      0 0 200px rgba(255, 200, 150, 0.4);
    filter: brightness(1.05);
  }

  /* Stronger/tighter glow when tiny */
  .orb.small-glow::before {
    opacity: 1;
    box-shadow:
      inset 0 0 18px rgba(255, 255, 255, 0.95),
      0 0 70px rgba(255, 220, 180, 0.95),
      0 0 110px rgba(255, 200, 150, 0.55);
  }

  .orb.small-glow {
    box-shadow:
      inset 0 0 22px rgba(255, 255, 255, 0.95),
      0 0 70px rgba(255, 220, 180, 0.95),
      0 0 110px rgba(255, 200, 150, 0.55);
    filter: brightness(1.1);
  }

  .orb.small-orb {
    width: 60px !important;
    height: 60px !important;
    border-radius: 50% !important;
  }

  .orb.expanded {
    border-radius: 40px;
    width: 100%;
    height: 100%;
    max-width: 100%;
    max-height: 100%;
    display: block;
    padding: 40px;
    box-sizing: border-box;
    overflow-y: auto;
    cursor: default;
    scrollbar-width: none;
    transform: scale(1) !important;

    box-shadow:
      inset 0 0 30px rgba(255, 255, 255, 0.9),
      0 0 120px rgba(255, 220, 180, 0.8),
      0 0 200px rgba(255, 200, 150, 0.4);

    filter: brightness(1.05);

    transition:
      width 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      height 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      border-radius 1.5s cubic-bezier(0.25, 1, 0.5, 1),
      background 1.5s ease,
      box-shadow var(--glow-fade) var(--glow-ease),
      filter var(--glow-fade) var(--glow-ease);
  }

  .orb.expanded::-webkit-scrollbar { display: none; }

  .orb.add-success { animation: float 8s ease-in-out infinite, addPulse 0.6s ease-out; }

  @keyframes addPulse {
    0% {
      box-shadow:
        inset 2px 4px 20px rgba(255, 255, 255, 0.6),
        0 25px 60px rgba(219, 168, 172, 0.35);
      filter: brightness(1.05);
    }
    40% {
      box-shadow:
        inset 0 0 26px rgba(255, 255, 255, 0.9),
        0 0 120px rgba(255, 220, 180, 0.9),
        0 40px 90px rgba(219, 168, 172, 0.6);
      filter: brightness(1.12);
    }
    100% {
      box-shadow:
        inset 2px 4px 20px rgba(255, 255, 255, 0.6),
        0 25px 60px rgba(219, 168, 172, 0.35);
      filter: brightness(1.05);
    }
  }

  .orb.typing-scale {
    transform: scale(1.02);
    transition: transform 0.4s cubic-bezier(0.2, 0.8, 0.2, 1);
  }

  .orb.pulsing {
    transform: translateY(0) scale(1.05) !important;
    transition: transform 0.05s cubic-bezier(0.2, 0.8, 0.2, 1);
  }
</style>
