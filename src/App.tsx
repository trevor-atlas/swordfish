import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App(props) {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("Trevor");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  let ref;

  let input

  return (
    <div ref={ref} class="search-container draggable-area">
      <div class="search-input-container">
        <input ref={input} autofocus={true} type="text" spellcheck={false} class="search-input" value={greetMsg()}/>
      </div>
      <div class="details-container">
        <div class="result-container">
          <ul>
            <li>one</li>
            <li>two</li>
          </ul>
        </div>
        <div class="preview-container">
          <img src="https://placehold.it/300/300"/>
        </div>
      </div>
    </div>
  );
}

export default App;
