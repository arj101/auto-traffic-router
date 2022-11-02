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
  let vehiclesPerSpawn = 3;

  let showFileOpener = false;
  let files;
  let mapData;
  let mapDataReadResult = "";
  let hadReadError = false;
  let vehicleAlpha = 0.2;

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

<div
  class="absolute top-0 left-0 w-full flex flex-col justify-stretch align-stretch mt-0 text-sm lg:mb-2 p-0"
>
  {#if showFileOpener}
    <div
      class="fixed top-0 left-0 h-screen w-screen bg-neutral-800 bg-opacity-30 z-1 z-10 backdrop-blur-sm"
      on:click={() => (showFileOpener = false)}
    />
    <div
      transition:fly={{ duration: 300, y: -100 }}
      class="z-20 transition-all color-white text-white accent-fuchsia-300 p-4 bg-fuchsia-300 bg-opacity-10 flex flex-row justify-start items-center {!showFileOpener
        ? '-top-30 '
        : ''}"
    >
      <label for="map-data" class="m-2 font-bold">Upload map file</label>
      <!-- svelte-ignore missing-declaration -->
      <div class="flex flex-row justify-stretch items-stretch ">
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
                  mapDataReadResult = "Successfully read map data";
                  showFileOpener = false;
                }
              } else {
                hadReadError = true;
                mapDataReadResult = "Unexpected type for file content";
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
          <p class="p-2 bg-red-500 rounded-r-md grid place-items-center">
            {mapDataReadResult}
          </p>
        {:else if mapDataReadResult.length > 0}
          <p class="p-2 bg-green-500 rounded-r-md grid place-items-center">
            {mapDataReadResult}
          </p>
        {/if}
      </div>
    </div>
    <div class="relative m-0 p-0 z-20 w-full h-fit top-0">
      <button
        transition:fly={{ duration: 300, y: -100 }}
        title="Closes menu to upload custom map file."
        class="transition-all absolute left-1/2 
             -translate-x-1/2 rounded-full bg-transparent border-0 font-black grid place-items-center hover:opacity-100 opacity-80 p-2 top-2"
        on:click={() => {
          showFileOpener = false;
        }}
      >
        <span
          title="Closes menu to upload custom map file."
          class=" -translate-y-0.5 material-symbols-rounded grid place-items-center text-fuchsia-300 text-3xl rotate-180"
        >
          expand_more
        </span>
      </button>
    </div>
  {:else}
    <div class="absolute top-0 left-0 m-0 p-0 z-20 w-full h-fit top-0">
      <button
        title="Opens menu to upload custom map file."
        class="absolute left-1/2 
                 -translate-x-1/2 rounded-full bg-transparent border-0 font-black grid place-items-center hover:opacity-100 opacity-80 p-2 top-2 opacity-50 px-8 py-0.5 m-0 border-opacity-0 top-0"
        on:click={() => {
          showFileOpener = true;
        }}
      >
        <span
          title="Opens menu to upload custom map file."
          class="-translate-y-0.5 material-symbols-rounded grid place-items-center text-fuchsia-300 text-3xl -translate-y-2"
        >
          expand_more
        </span>
      </button>
    </div>
  {/if}
</div>

<main
  class="z-0 transition-all min-w-screen min-h-screen bg-neutral-800 p-2 pt-6 font-sans flex flex-col-reverse lg:flex-row justify-stretch items-stretch md:justify-around md:items-center"
>
  <div class="flex flex-col justify-around items-stretch m-4">
    <div
      class="flex flex-row m-2 justify-stretch items-stretch p-0 w-full md:w-auto"
    >
      <div
        class="m-0 flex flex-col justify-around items-stretch w-full text-white text-left p-4 border-2 rounded-sm border-fuchsia-200 border-opacity-30"
      >
        <h1 class="text-xl font-black">Stats</h1>
        <hr />

        <p
          class="text-md"
          title="Number of vehicles that has reached their destination."
        >
          Vehicle count: <span
            class="font-semibold text-fuchsia-300"
            title="Number of vehicles that has reached their destination."
            >{stats_vehicle_count}</span
          >
        </p>
        <p
          class="text-md"
          title="Number of vehicles reaching destination per unit time."
        >
          Flux: <span
            class="font-semibold text-fuchsia-300"
            title="Number of vehicles reaching destination per unit time."
            >{stats_flux.toFixed(5)}</span
          >
        </p>
        <p
          class="text-md"
          title="Average speed of all vehicles that has reached their destination"
        >
          Speed: <span
            class="font-semibold text-fuchsia-300"
            title="Average speed of all vehicles that has reached their destination"
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
        <label
          for="spawn-prob"
          class="font-bold"
          title="Probability of spawning new vehicles each frame."
          >Spawn probability</label
        >
        <div class="flex flex-row justify-stretch w-full">
          <input
            title="Probability of spawning new vehicles each frame."
            type="range"
            name="spawn-prob"
            id="spawn-prob"
            min="0"
            max="1"
            step="0.001"
            bind:value={spawnProbability}
            on:input={() => {
              if (simulator) simulator.spawnProbability = spawnProbability;
            }}
          />

          <input
            title="Probability of spawning new vehicles each frame."
            class="m-1 w-20 bg-transparent border-2 border-fuchsia-200 border-opacity-20 p-1 rounded-sm font-semibold text-fuchsia-300 outline-none"
            type="number"
            name="spawn-prob"
            id="spawn-prob-val"
            min="0"
            max="1"
            step="0.001"
            bind:value={spawnProbability}
            on:input={() => {
              if (simulator) simulator.spawnProbability = spawnProbability;
            }}
          />
        </div>
        <hr />
        <label
          for="time-scaling"
          class="font-bold"
          title="Scales the timestep by this number."
          >Simulation speed [{simSpeed.toFixed(2)}x]</label
        >
        <input
          title="Scales the timestep by this number."
          type="range"
          name="time-scaling"
          id="time-scaling"
          min="0"
          max="5"
          step="0.01"
          bind:value={simSpeed}
          on:input={() => {
            if (simulator) simulator.timeScale = simSpeed;
          }}
        />

        <hr />
        <label for="vehicles-per-spawn" class="font-bold"
          >Max. vehicles per spawn [{vehiclesPerSpawn}]</label
        >
        <input
          title="Number of vehicles that are each time."
          type="range"
          name="vehicles-per-spawn"
          id="vehicles-per-spawn"
          min="0"
          max="10"
          step="1"
          bind:value={vehiclesPerSpawn}
          on:input={() => {
            if (simulator) simulator.vehiclesPerSpawn = vehiclesPerSpawn;
          }}
        />
        <hr />
        <label
          for="vehicle-alpha"
          class="font-bold"
          title="Transparency value of the vehicles (circles)"
          >Vehicle alpha [{vehicleAlpha.toFixed(2)}]</label
        >
        <input
          title="Transparency value of the vehicles (circles)"
          type="range"
          name="vehicle-alpha"
          id="vehicle-alpha"
          min="0"
          max="0.5"
          step="0.01"
          bind:value={vehicleAlpha}
          on:input={() => {
            simulator.rebuildVehicleTexture(vehicleAlpha);
          }}
        />
        <hr />

        <button
          title="Clears all vehicles and creates a new instance of the underlying (WASM) traffic simulator"
          class=" mx-0 text-sm border-red-500 text-red-500 font-black hover:bg-red-500 active:bg-opacity-100 active:text-white hover:bg-opacity-10 bg-transparent rounded-md border-2 px-5 py-2"
          on:click={() => {
            simulator.reinstantiateWithMap(mapData);
          }}
        >
          Reset
        </button>
      </div>
      <div
        class="m-2 flex flex-col justify-around items-start text-white text-left p-5 border-2 rounded-sm border-fuchsia-200 w-full border-opacity-30"
      >
        <h1 class="text-xl font-black">Rerouting algorithm settings</h1>
        <hr />
        <p class="text-fuchsia-100 text-lg">sensitivity settings</p>
        <hr />
        <label for="density-coeff" class="font-bold">Traffic density</label>
        <div class="flex flex-row justify-stretch w-full">
          <input
            type="range"
            name="density-coeff"
            id="density-coeff"
            min="0"
            max="300"
            step="1"
            bind:value={densityCoeff}
            on:input={() => {
              if (simulator) simulator.densityCoeff = densityCoeff;
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
              if (simulator) simulator.densityCoeff = densityCoeff;
            }}
          />
        </div>

        <hr />
        <label for="vel-coeff" class="font-bold">Average speed</label>
        <div class="flex flex-row justify-stretch w-full">
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
    class="w-full h-auto lg:w-max border-black border-2 rounded-sm m-1"
  />
</main>

<style lang="postcss">
  input {
    @apply accent-fuchsia-300;
  }

  h1 {
    @apply text-base;
  }

  label,
  p {
    @apply text-sm;
  }

  input[type="number"] {
    @apply ml-2  text-sm;
  }

  input[type="range"] {
    @apply w-full;
  }

  hr {
    @apply w-full my-2 border-fuchsia-200 border-opacity-30;
  }
</style>
