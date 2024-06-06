<script lang="ts">
  import "./styles.css";
  
	import { Jumper } from 'svelte-loading-spinners';
  import { VisXYContainer, VisGroupedBar, VisLine, VisAxis } from '@unovis/svelte'

  type DataRecord = { x: number, y: number }
  export let result: DataRecord[] = [
  ]

  import Paint from './lib/Paint.svelte'
  import { invoke } from "@tauri-apps/api/tauri";

  const pixelSize = 45;
  const targetResolution = 8;

  let getPaintData: () => ImageData;

  var evaluating = false;
  var evaluated = false;
  var error = "";
  var result;

  async function evaluate() {
    evaluating = true;
    evaluated = false;

    // Create an array of pixels
    const image = getPaintData();
    let pixels = [];
    image.data.forEach((value: number, index: number, _: Uint8ClampedArray) => {
      
      // Process alpha values only
      if (index % 4 != 3) {
        return;
      }

      // Process only the pixels in the centre of each cell
      const pixelIndex = Math.floor(index / 4);
      if (Math.floor(pixelIndex / image.width) % pixelSize != Math.floor(pixelSize / 2)) {
        return;
      }

      if (pixelIndex % pixelSize != Math.floor(pixelSize / 2)) {
        return;
      }

      pixels.push(value / 255);
    });

    try {
          result = await invoke("evaluate", { pixels: pixels });
          console.log(result);
       let raw_result: number[] = await invoke("evaluate", { pixels: pixels });
      
      // data = [
      //   { x: 0, y: 2 },
      //   { x: 1, y: 0 },
      //   { x: 2, y: 5 },
      //   { x: 3, y: 6 },
      //   { x: 4, y: 6 },
      // ]
    // console.log(result);
      result = []
      raw_result.forEach((v, i) => result.push({ x: i, y: v }));
      console.log(result);
      result = result;
    } catch (e) {
      error = e;
    }

    evaluating = false;
    evaluated = true;
  }
</script>

<header class="p-10 text-center">
  <h1 class="text-4xl font-bold">NeuronVeil Digit Detector</h1>
  <h2 class="subtitle">Digit detection using <a class="text-blue-600 visited:text-purple-600" href="https://github.com/AmitDIRTYC0W/neuronveil" target="_blank">NeuronVeil</a></h2>
</header>

<section>
  <main>
    <center>
      <Paint
        width={targetResolution * pixelSize}
        height={targetResolution * pixelSize}
        locked={evaluating}
        bind:getImageData={getPaintData}/>
      {#if !evaluating}
        <button class="p-2 rounded-md text-white shadow-md bg-sky-500" on:click={evaluate}>Evaluate</button>
      {/if}
      {#if evaluating}
        <div class="p-4">
          <Jumper size="5" unit="em" color="#0ea5e9" />
        </div>
      {/if}
      {#if evaluated}
        <div class="p-10">
          <VisXYContainer width="50%">
            <VisGroupedBar data={result} x={d => d.x} y={d => d.y}/>
            <VisAxis type="x" numTicks={10} gridLine={false} tickTextFontSize="20pt"/>
          </VisXYContainer>
        </div>
      {/if}

      <p class="font-bold text-red-500">{error}</p>
     </center>
  </main>
</section>

