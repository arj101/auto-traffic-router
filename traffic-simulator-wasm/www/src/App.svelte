<script lang="ts">
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
    import { Simulator } from "./lib/simulator";

    let simulator: Simulator;
    let canvas;

    let stats_vehicle_count = 0;
    let stats_flux = 0;
    let stats_speed = 0;

    let spawnProbability = 0;
    let simSpeed = 1;
    let densityCoeff = 0;
    let velCoeff = 0;

    let showFileOpener = true;
    let files;
    let mapData;
    let mapDataReadResult = "";
    let hadReadError = false;

    onMount(() => {
        simulator = new Simulator(canvas!);
        simulator.start();
    });

    setInterval(() => {
        if (!simulator) return;

        stats_vehicle_count = simulator.sim.stats.completed_vehicle_count;
        stats_flux = simulator.sim.stats.avg_flux;
        stats_speed = simulator.sim.stats.avg_vel;
    }, 100);
</script>

<div class="flex flex-col justify-stretch align-stretch">
    {#if showFileOpener}
        <div
            transition:fly={{ duration: 300, y: -100 }}
            class="color-white text-white accent-fuchsia-300 p-4 bg-fuchsia-300 bg-opacity-10 flex flex-row justify-start items-center"
        >
            <label for="map-data" class="m-2 font-bold">Upload map file</label>
            <!-- svelte-ignore missing-declaration -->
            <div class="flex flex-row justify-stretch items-stretch">
                <input
                    class="cursor-pointer p-2 block placeholder-fuchsia-300 mr-0 bg-transparent text-white border-2 border-fuchsia-200 border-opacity-30 rounded-l-md hover:bg-fuchsia-300 hover:bg-opacity-30"
                    type="file"
                    name="map-data"
                    id="map-data"
                    bind:files
                    on:change={() => {
                        const reader = new FileReader();
                        reader.onload = (e) => {
                            if (typeof e.target.result == "string") {
                                let readSuccess = true;
                                mapData = (() => {
                                    try {
                                        return JSON.parse(e.target.result);
                                    } catch (e) {
                                        hadReadError = true;
                                        mapDataReadResult =
                                            "Parsing JSON failed (wrong format maybe?)";
                                        readSuccess = false;
                                    }
                                })();
                                if (readSuccess) {
                                    simulator.reinstantiateWithMap(mapData);
                                    hadReadError = false;
                                    mapDataReadResult =
                                        "Successfully read map data";
                                }
                            } else {
                                hadReadError = true;
                                mapDataReadResult =
                                    "Unexpected type for file content";
                            }
                        };
                        reader.onerror = (e) => {
                            hadReadError = true;
                            mapDataReadResult = `Error reading file ${e}`;
                        };
                        reader.readAsText(files[0]);
                    }}
                />
                {#if hadReadError && mapDataReadResult.length > 0}
                    <p
                        class="p-2 bg-red-500 rounded-r-md grid place-items-center"
                    >
                        {mapDataReadResult}
                    </p>
                {:else if mapDataReadResult.length > 0}
                    <p
                        class="p-2 bg-green-500 rounded-r-md grid place-items-center"
                    >
                        {mapDataReadResult}
                    </p>
                {/if}
            </div>
        </div>
    {/if}

    <div class="relative">
        <button
            class="transition-all absolute left-1/2 top-2 -translate-x-1/2 rounded-xl bg-transparent border-2 border-fuchsia-300 font-black shadow-lg grid place-items-center hover:opacity-100 {showFileOpener
                ? 'opacity-80 p-2'
                : 'opacity-30 p-1'}"
            on:click={() => {
                showFileOpener = !showFileOpener;
            }}
        >
            <span
                class="material-symbols-rounded grid place-items-center text-fuchsia-300 text-3xl {showFileOpener
                    ? 'rotate-180'
                    : ''}"
            >
                expand_more
            </span>
        </button>
    </div>
</div>

<main
    class="transition-all min-w-screen min-h-screen bg-neutral-800 p-2 font-sans flex flex-col lg:flex-row justify-around items-center"
>
    <div
        class="flex flex-col-reverse md:flex-col justify-around items-stretch m-4"
    >
        <div class="flex flex-row m-0 justify-stretch items-stretch w-full">
            <div
                class="m-2 flex flex-col justify-around items-start text-white text-left p-4 border-2 rounded-sm border-fuchsia-200 w-full border-opacity-30"
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
        </div>
        <div
            class="flex flex-col md:flex-row m-0 justify-stretch items-stretch w-full"
        >
            <div
                class="m-2 flex flex-col justify-around items-start text-white text-left p-5 border-2 rounded-sm border-fuchsia-200 w-full border-opacity-30"
            >
                <h1 class="text-xl font-black">Settings</h1>
                <hr />
                <label for="spawn-prob" class="font-bold"
                    >Spawn probability</label
                >
                <div class="flex flex-row justify-between">
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
                        class="m-1 w-20 bg-transparent border-2 border-fuchsia-200 border-opacity-20 p-1 rounded-sm font-semibold text-fuchsia-300 outline-none"
                        type="number"
                        name="spawn-prob"
                        id="spawn-prob-val"
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
                <hr />
                <label for="time-scaling" class="font-bold"
                    >Simulation speed [{simSpeed.toFixed(2)}x]</label
                >
                <input
                    type="range"
                    name="time-scaling"
                    id="time-scaling"
                    min="0"
                    max="4"
                    step="0.01"
                    bind:value={simSpeed}
                    on:input={() => {
                        if (simulator) simulator.timeScale = simSpeed;
                    }}
                />
            </div>
            <div
                class="m-2 flex flex-col justify-around items-start text-white text-left p-5 border-2 rounded-sm border-fuchsia-200 w-full border-opacity-30"
            >
                <h1 class="text-xl font-black">Rerouting algorithm settings</h1>
                <hr />
                <p class="text-fuchsia-100 text-lg">sensitivity settings</p>
                <hr />
                <label for="density-coeff" class="font-bold"
                    >Traffic density</label
                >
                <div class="flex flex-row justify-between">
                    <input
                        type="range"
                        name="density-coeff"
                        id="density-coeff"
                        min="0"
                        max="300"
                        step="1"
                        bind:value={densityCoeff}
                        on:input={() => {
                            if (simulator)
                                simulator.densityCoeff = densityCoeff;
                        }}
                    />
                    <input
                        class="w-20  m-1 bg-transparent border-2 border-fuchsia-200 border-opacity-20 p-1 rounded-sm font-semibold text-fuchsia-300 outline-none"
                        type="number"
                        name="density-coeff"
                        id="density-coeff-val"
                        min="0"
                        max="300"
                        step="1"
                        bind:value={densityCoeff}
                        on:input={() => {
                            if (simulator)
                                simulator.densityCoeff = densityCoeff;
                        }}
                    />
                </div>

                <hr />
                <label for="vel-coeff" class="font-bold">Average speed</label>
                <div class="flex flex-row  justify-between">
                    <input
                        type="range"
                        name="vel-coeff"
                        id="vel-coeff"
                        min="0"
                        max="300"
                        step="1"
                        bind:value={velCoeff}
                        on:input={() => {
                            if (simulator) simulator.velocityCoeff = velCoeff;
                        }}
                    />
                    <input
                        class="w-20 m-1 bg-transparent border-2 border-fuchsia-200 border-opacity-20 p-1 rounded-sm font-semibold text-fuchsia-300 outline-none"
                        type="number"
                        name="density-coeff"
                        id="density-coeff-val"
                        min="0"
                        max="300"
                        step="1"
                        bind:value={velCoeff}
                        on:input={() => {
                            if (simulator) simulator.velocityCoeff = velCoeff;
                        }}
                    />
                </div>
            </div>
        </div>
    </div>
    <canvas
        bind:this={canvas}
        width="640"
        height="480"
        class="w-full h-auto lg:w-max border-black border-2 rounded-sm m-2"
    />
</main>

<style lang="postcss">
    input {
        @apply accent-fuchsia-300;
    }

    input[type="number"] {
        @apply ml-2;
    }

    input[type="range"] {
        @apply w-32;
    }

    #time-scaling {
        @apply w-full;
    }

    hr {
        @apply w-full my-2 border-fuchsia-200 border-opacity-30;
    }
</style>
