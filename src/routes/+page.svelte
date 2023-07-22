<script lang="ts">
  import { INPUT_EVENT_TYPE, COLOR, Chessboard, BORDER_TYPE } from "cm-chessboard/src/Chessboard";
  import { MARKER_TYPE, Markers } from "cm-chessboard/src/extensions/markers/Markers.js"
  import { PromotionDialog } from "cm-chessboard/src/extensions/promotion-dialog/PromotionDialog.js"
  import { Accessibility } from "cm-chessboard/src/extensions/accessibility/Accessibility.js"
  import { Chess } from "chess.js";
  import "$lib/assets/styles/cm-chessboard.css";
  import { onMount } from "svelte";

  import {invoke} from '@tauri-apps/api/tauri'

  // ChessJS
  let chess = new Chess('r2qkbnr/ppp2p1p/2n3p1/3ppb2/1P2P3/2NP1N2/P1P2PPP/R1BQKB1R w KQkq - 0 1');

  let board_el: any;
  let board: any;
  let fen: string = "";
  let engine_move: string = "";

  // Board config
  let board_config = {
      position: chess.fen(),
      extensions: [
        { class: Markers, props: { autoMarkers: MARKER_TYPE.square } },
        { class: PromotionDialog },
        { class: Accessibility, props: { visuallyHidden: true } },
      ],
    }

  async function makeEngineMove(chessboard: any) {
    let board_fen = chess.fen()
    let move_str: string = await invoke('run_engine', {currentFen: board_fen})
    let move_from = move_str.substring(0, 2)
    let move_to   = move_str.substring(2)
    
    let move_obj = {from: move_from, to: move_to}
    console.log(move_obj)
    chess.move(move_obj)
    chessboard.setPosition(chess.fen(), true)
    chessboard.enableMoveInput(inputHandler, COLOR.white)
  }


  function makeRandomMove(chessboard: any) {
        const possibleMoves = chess.moves({verbose: true})
        if (possibleMoves.length > 0) {
            const randomIndex = Math.floor(Math.random() * possibleMoves.length)
            const randomMove = possibleMoves[randomIndex]
            setTimeout(() => { // smoother with 500ms delay
                chess.move({from: randomMove.from, to: randomMove.to})
                chessboard.setPosition(chess.fen(), true)
                chessboard.enableMoveInput(inputHandler, COLOR.white)
            }, 500)
        }
    }

  function inputHandler(event: any) {
      console.log("inputHandler", event)
      if(event.type !== INPUT_EVENT_TYPE.moveInputFinished) {
          event.chessboard.removeMarkers(MARKER_TYPE.dot)
          event.chessboard.removeMarkers(MARKER_TYPE.bevel)
      }
      if (event.type === INPUT_EVENT_TYPE.moveInputStarted) {
          const moves = chess.moves({square: event.squareFrom, verbose: true})
          for (const move of moves) { // draw dots on possible squares
              if (move.promotion && move.promotion !== "q") {
                  continue
              }
              if (event.chessboard.getPiece(move.to)) {
                  event.chessboard.addMarker(MARKER_TYPE.bevel, move.to)
              } else {
                  event.chessboard.addMarker(MARKER_TYPE.dot, move.to)
              }
          }
          return moves.length > 0
      } else if (event.type === INPUT_EVENT_TYPE.validateMoveInput) {
          const move = {from: event.squareFrom, to: event.squareTo, promotion: event.promotion}
          const result = chess.move(move)
          console.log(result)
          console.log(board.state)
          console.log(board.state.moveInputProcess)
          if (result) {
              board.state.moveInputProcess.then(() => { // wait for the move input process has finished
                  board.setPosition(chess.fen(), true).then(() => { // update position, maybe castled and wait for animation has finished
                      makeEngineMove(event.chessboard)
                  })
              })
          } else {
              // promotion?
              let possibleMoves = chess.moves({square: event.squareFrom, verbose: true})
              for (const possibleMove of possibleMoves) {
                  if (possibleMove.promotion && possibleMove.to === event.squareTo) {
                      event.chessboard.showPromotionDialog(event.squareTo, COLOR.white, (result) => {
                          console.log("promotion result", result)
                          if (result) {
                              chess.move({from: event.squareFrom, to: event.squareTo, promotion: result.piece.charAt(1)})
                              event.chessboard.setPosition(chess.fen(), true)
                              makeEngineMove(event.chessboard).then(() => {})
                          } else {
                              // promotion canceled
                              event.chessboard.enableMoveInput(inputHandler, COLOR.white)
                              event.chessboard.setPosition(chess.fen(), true)
                          }
                      })
                      return true
                  }
              }
          }
          return result
      } else if (event.type === INPUT_EVENT_TYPE.moveInputFinished) {
          if(event.legalMove) {
              event.chessboard.disableMoveInput()
          }
      }
  }

  async function run_engine(){
    console.log("starting engine")
    let current_fen = chess.fen()
    engine_move = await invoke('run_engine', {currentFen: current_fen})
    console.log("engine finished")
  }

  // Mount board when ready 
  onMount(() => {
    board = new Chessboard(board_el, board_config);
    board.enableMoveInput(inputHandler, COLOR.white)
    console.log(board);
  });

  function updateFen() {
    fen = chess.fen();
  }
</script>

<div bind:this={board_el} style="width: auto;" />

<!-- <p>{fen}</p>
<button on:click={updateFen}> print </button> -->

<p>Best engine move: {engine_move}</p>
<button on:click={run_engine}>
  run engine
</button>
