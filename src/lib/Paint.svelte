<script lang="ts">
    import Konva from 'konva';
  import { Stage, Layer, Rect, Line } from 'svelte-konva';

  export let width: number;
  export let height: number;
  export let border: number = 5;
  export let tension: number = 0.5;
  export let stroke: string = '#363636';
  export let background: string = '#f5f5f5';
  export let strokeWidth: number = 25;
  export let cornerRadius: number = 25;
  export let locked: boolean = false;
  
  var isPainting = false;
  let lines = [];
  let drawing: Konva.Layer;

  function handlePress(e) {
    if (locked) {
      return;
    }
    
    const konvaEvent = e.detail;
    const stage = konvaEvent.target.getStage();
    const position = stage.getPointerPosition();

    isPainting = true;
    lines.push({
                points: [position.x, position.y],
                tension: tension,
                stroke: stroke,
                lineCap: 'round',
                lineJoin: 'round',
                strokeWidth: strokeWidth,
    });

    lines = lines;
  }

  function handleRelease(_) {
    isPainting = false;
  }

  function handleMove(e) {
    if (!isPainting || locked) {
      return;
    }

    const konvaEvent = e.detail;
    const stage = konvaEvent.target.getStage();
    const position = stage.getPointerPosition();

    let currentLine = lines.at(-1);
    currentLine.points.push(position.x, position.y);

    lines = lines;
  }

  function clear() {
    if (!locked) {
      lines = [];
    }
  }

  export function getImageData(): ImageData {
    return drawing.getContext().getImageData(border, border, width, height);
  }
</script>

<Stage class="block" config={{ width: width + border * 2, height: height + border * 2}} >
  <Layer>
    <Rect config={{ x: border, y: border, width: width, height: height, cornerRadius: cornerRadius, fill: background, stroke: stroke}} />
  </Layer>
  <Layer
    on:mousedown={handlePress}
    on:touchstart={handlePress}
    on:mouseup={handleRelease}
    on:mouseleave={handleRelease}
    on:touchend={handleRelease}
    on:mousemove={handleMove}
    on:touchmove={handleMove}
    config={{width: width, height: height}}
    bind:handle={drawing}
  >
    <Rect config={{ width: height, height: height}} />
    {#each lines as config}
      <Line {config} />
    {/each}
  </Layer>
</Stage>

{#if !locked}
  <button class="p-2 shadow-md rounded-md" on:click={clear}>Clear</button>
{/if}
