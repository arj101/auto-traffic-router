<script lang="ts">
    import { onMount } from "svelte";
    import { Simulator } from "./lib/simulator";

    let simulator;
    let canvas;

    onMount(() => {
        simulator = new Simulator(canvas!);
    });
</script>

<main class="w-screen h-screen bg-neutral-800 p-2 overflow-scroll">
    <canvas
        bind:this={canvas}
        width="640"
        height="480"
        class="border-black border-2 rounded-sm"
    />
</main>

<style lang="postcss">
</style>
