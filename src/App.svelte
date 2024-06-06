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
  const targetSize = 8;

  let getPaintData: () => ImageData;

  var evaluating = false;
  var evaluated = false;
  var error = "";

  function resizeImageWithAverageAlpha(
    imageData: ImageData,
    newWidth: number,
    newHeight: number
  ): number[] {
    const { data, width, height } = imageData;
    const alphaValues: number[] = [];
    const ratioX = width / newWidth;
    const ratioY = height / newHeight;

    for (let y = 0; y < newHeight; y++) {
      for (let x = 0; x < newWidth; x++) {
        let totalAlpha = 0;
        let count = 0;

        for (let j = 0; j < ratioY; j++) {
          for (let i = 0; i < ratioX; i++) {
            const pixelIndex = ((y * ratioY + j) * width + (x * ratioX + i)) * 4 + 3;
            totalAlpha += data[pixelIndex];
            count++;
          }
        }

        const averageAlpha = totalAlpha / count;
        alphaValues.push(averageAlpha);
      }
    }

    return alphaValues;
  }


  async function evaluate() {
    evaluating = true;
    evaluated = false;

    // Create an array of pixels
    const image = getPaintData();
    let pixels = resizeImageWithAverageAlpha(image, targetSize, targetSize);
    pixels = pixels.map((v) => v / 16);
      
    try {
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
        width={targetSize * pixelSize}
        height={targetSize * pixelSize}
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

