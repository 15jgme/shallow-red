<script lang="ts">
	import { onMount } from 'svelte';
	
	let board: any;
	let el: any;
	
	function onDrop (source, target, piece, newPos, oldPos, orientation) {
  	console.log('Source: ' + source)
  	console.log('Target: ' + target)
  	console.log('Piece: ' + piece)
  	console.log('New position: ' + Chessboard.objToFen(newPos))
  	console.log('Old position: ' + Chessboard.objToFen(oldPos))
  	console.log('Orientation: ' + orientation)
  	console.log('~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~')
	}
	
	function initBoard() {
		board = Chessboard(el, {
			draggable: true,
  		position: 'start',
  		onDrop: onDrop,
  		sparePieces: true});
	}
	
	onMount(() => {
    console.log('Mounted');
  });
	
	//How to implement the onDrop event?
	
</script>

<svelte:head>
	<script src="https://cdnjs.cloudflare.com/ajax/libs/jquery/3.6.0/jquery.min.js" ></script>
	<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/chessboard-js/1.0.0/chessboard-1.0.0.min.css"/>
	<script src="https://cdnjs.cloudflare.com/ajax/libs/chessboard-js/1.0.0/chessboard-1.0.0.js" on:load={initBoard}> </script>
</svelte:head>
	
{#if board===undefined}
	<div style="width: 400px, height:400px background-color: gray"></div>
{/if}

<div bind:this={el} style="width: 400px"></div>

<button id="startBtn" on:click={() => initBoard()}>Start Position</button>
<button id="clearBtn">Clear Board</button>