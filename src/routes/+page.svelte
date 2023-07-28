<script lang="ts">
  import {
    INPUT_EVENT_TYPE,
    COLOR,
    Chessboard,
    BORDER_TYPE,
  } from "cm-chessboard/src/Chessboard";
  import {
    MARKER_TYPE,
    Markers,
  } from "cm-chessboard/src/extensions/markers/Markers.js";
  import { PromotionDialog } from "cm-chessboard/src/extensions/promotion-dialog/PromotionDialog.js";
  import { Accessibility } from "cm-chessboard/src/extensions/accessibility/Accessibility.js";
  import { Chess, DEFAULT_POSITION, validateFen } from "chess.js";
  import "$lib/assets/styles/cm-chessboard.css";
  import "$lib/assets/extensions/promotion-dialog/promotion-dialog.css";
  import "$lib/assets/extensions/markers/markers.css"
  import { onMount } from "svelte";

  import { invoke } from "@tauri-apps/api/tauri";

  // UI
  enum engine_state_emoji {
    Pondering = "🤔",
    Waiting = "😁",
  }
  let curr_engine_state_emoj: engine_state_emoji = engine_state_emoji.Waiting;
  let fen_error_msg: String = "";

  let board_el: any;
  let board: any;
  let fen: string = "8/1pn5/5k2/8/6P1/3K1N2/8/8 w - - 0 1";
  fen = DEFAULT_POSITION;
  let engine_move: string = "";

  // ChessJS
  let chess = new Chess(fen);

  // Board config
  let board_config = {
    position: chess.fen(),
    extensions: [
      { class: Markers, props: { autoMarkers: MARKER_TYPE.square } },
      { class: PromotionDialog },
      { class: Accessibility, props: { visuallyHidden: true } },
    ],
  };

  function reload_chess() {
    // Validate fen first
    let { ok, error } = validateFen(fen);

    if (ok) { 
      fen_error_msg = "" // Erase error message

      // Reset engine emoji
      curr_engine_state_emoj = engine_state_emoji.Waiting;

      // Reset ChessJS
      chess = new Chess(fen);

      // Reset UI
      let board_config = {
        position: chess.fen(),
        extensions: [
          { class: Markers, props: { autoMarkers: MARKER_TYPE.square } },
          { class: PromotionDialog },
          { class: Accessibility, props: { visuallyHidden: true } },
        ],
      };
      // board = new Chessboard(board_el, board_config);
      board.setPosition(chess.fen(), true);
      board.enableMoveInput(inputHandler, COLOR.white);
    } else if (error){
      fen_error_msg = error;
    } else {
      fen_error_msg = "invalid fen"
    }
  }

  async function makeEngineMove(chessboard: any) {
    let board_fen = chess.fen();
    let move_str: string = await invoke("run_engine", {
      currentFen: board_fen,
    });
    let move_from = move_str.substring(0, 2);
    let move_to = move_str.substring(2);

    let move_obj = { from: move_from, to: move_to };
    console.log(move_obj);
    chess.move(move_obj);
    chessboard.setPosition(chess.fen(), true);
    chessboard.enableMoveInput(inputHandler, COLOR.white);
  }

  async function inputHandler(event: any) {
    console.log("inputHandler", event);
    if (event.type !== INPUT_EVENT_TYPE.moveInputFinished) {
      event.chessboard.removeMarkers(MARKER_TYPE.dot);
      event.chessboard.removeMarkers(MARKER_TYPE.bevel);
    }
    if (event.type === INPUT_EVENT_TYPE.moveInputStarted) {
      const moves = chess.moves({ square: event.squareFrom, verbose: true });
      for (const move of moves) {
        // draw dots on possible squares
        if (move.promotion && move.promotion !== "q") {
          continue;
        }
        if (event.chessboard.getPiece(move.to)) {
          event.chessboard.addMarker(MARKER_TYPE.bevel, move.to);
        } else {
          event.chessboard.addMarker(MARKER_TYPE.dot, move.to);
        }
      }
      return moves.length > 0;
    } else if (event.type === INPUT_EVENT_TYPE.validateMoveInput) {
      const move = {
        from: event.squareFrom,
        to: event.squareTo,
        promotion: event.promotion,
      };
      
      let result = undefined;
      try {
        result = chess.move(move);
      } catch(e) {
        console.log(e);
      }

      if (result) {
        board.state.moveInputProcess.then(() => {
          // wait for the move input process has finished
          board.setPosition(chess.fen(), true).then(() => {
            // update position, maybe castled and wait for animation has finished
            curr_engine_state_emoj = engine_state_emoji.Pondering;
            makeEngineMove(event.chessboard).then(() => {
              curr_engine_state_emoj = engine_state_emoji.Waiting;
            });
          });
        });
      } else {
        // promotion?
        let possibleMoves = chess.moves({
          square: event.squareFrom,
          verbose: true,
        });
        for (const possibleMove of possibleMoves) {
          if (possibleMove.promotion && possibleMove.to === event.squareTo) {
            event.chessboard.showPromotionDialog(
              event.squareTo,
              COLOR.white,
              (result) => {
                console.log("promotion result", result);
                if (result) {
                  chess.move({
                    from: event.squareFrom,
                    to: event.squareTo,
                    promotion: result.piece.charAt(1),
                  });
                  event.chessboard.setPosition(chess.fen(), true);
                  makeEngineMove(event.chessboard).then(() => {});
                } else {
                  // promotion canceled
                  event.chessboard.enableMoveInput(inputHandler, COLOR.white);
                  event.chessboard.setPosition(chess.fen(), true);
                }
              }
            );
            return true;
          }
        }
      }
      return result;
    } else if (event.type === INPUT_EVENT_TYPE.moveInputFinished) {
      if (event.legalMove) {
        event.chessboard.disableMoveInput();
      }
    }
  }

  // Mount board when ready
  onMount(() => {
    board = new Chessboard(board_el, board_config);
    board.enableMoveInput(inputHandler, COLOR.white);
    console.log(board);
  });
</script>

<div class="p-2 flex flex-col items-center justify-center h-screen">
  <div><p>Engine state: {curr_engine_state_emoj}</p></div>
  <div class="w-1/2" bind:this={board_el} />

  <div class="flex space-x-2 py-2">
    <p class="">FEN:</p>
    <input
      class="shadow appearance-none border rounded w-3/4 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
      bind:value={fen}
    />

    <button
      class="bg-blue-500 hover:bg-blue-700 text-white font-bold px-2 rounded"
      on:click={reload_chess}>Reload</button
    >
  </div>
  <p class="font-extralight text-red-300">{fen_error_msg}</p>
  {#if fen_error_msg != ""}
    <button class="bg-red-500 hover:bg-red-700 text-white font-bold px-2 rounded" on:click={() => {fen = DEFAULT_POSITION; reload_chess()}}>Total FEN reset?</button>
  {/if}
</div>

<!-- <style>
  #filldiv {
    
  }
</style> -->
