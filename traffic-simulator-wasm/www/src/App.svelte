<script lang="ts">
    import { onMount } from "svelte";
    import { Simulator } from "./lib/simulator";

    let simulator: Simulator;
    let canvas;

    let stats_vehicle_count = 0;
    let stats_flux = 0;
    let stats_speed = 0;

    let spawnProbability = 0;

    onMount(() => {
        simulator = new Simulator(canvas!);
    });

    setInterval(() => {
        if (!simulator) return;

        stats_vehicle_count = simulator.sim.stats.completed_vehicle_count;
        stats_flux = simulator.sim.stats.avg_flux;
        stats_speed = simulator.sim.stats.avg_vel;
    }, 100);
</script>

<main class="w-screen h-screen bg-neutral-800 p-2 overflow-scroll font-sans">
    <div class="flex flex-row justify-start items-stretch">
        <div
            class="my-2 flex flex-col justify-around items-start text-white text-left p-4 border-2 rounded-sm border-fuchsia-200 w-fit border-opacity-30"
        >
            <h1 class="text-xl font-black">Stats</h1>
            <hr />

            <p class="text-md">
                Vehicle count: <span class="font-semibold text-fuchsia-300"
                    >{stats_vehicle_count}</span
                >
            </p>
            <p class="text-md">
                Flux: <span class="font-semibold text-fuchsia-300"
                    >{stats_flux.toFixed(5)}</span
                >
            </p>
            <p class="text-md">
                Speed: <span class="font-semibold text-fuchsia-300"
                    >{stats_speed.toFixed(5)}</span
                >
            </p>
        </div>
        <div
            class="m-2 flex flex-col justify-around items-start text-white text-left p-4 border-2 rounded-sm border-fuchsia-200 w-fit border-opacity-30"
        >
            <h1 class="text-xl font-black">Settings</h1>
            <hr />
            <label for="spawn-prob" class="font-bold">Spawn probability</label>
            <input
                type="range"
                name="spawn-prob"
                id="spawn-prob"
                min="0"
                max="1"
                step="0.001"
                bind:value={spawnProbability}
                on:input={() => {
                    if (simulator)
                        simulator.spawnProbability = spawnProbability;
                }}
            />
            <input
                class="  m-1 bg-transparent border-2 border-fuchsia-200 border-opacity-20 p-1 rounded-sm font-semibold text-fuchsia-300 outline-none"
                type="number"
                name="spawn-prob"
                id="spawn-prob"
                min="0"
                max="1"
                step="0.001"
                bind:value={spawnProbability}
                on:input={() => {
                    if (simulator)
                        simulator.spawnProbability = spawnProbability;
                }}
            />
        </div>
    </div>

    <canvas
        bind:this={canvas}
        width="640"
        height="480"
        class="border-black border-2 rounded-sm"
    />
</main>

<style lang="postcss">
    input {
        @apply accent-purple-400;
    }

    hr {
        @apply w-full my-2 border-fuchsia-200 border-opacity-30;
    }
</style>
