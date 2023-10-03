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
            <li class="active">
              <span class="result-heading">Exodia</span>
              <span class="result-subtext">The forbidden one</span>
            </li>
            <li>
              <span class="result-heading">Aqua Teen Hunger Force</span>
              <span class="result-subtext">Look at him and tell me there's a god</span>
            </li>
            <li>
              <span class="result-heading">Ted Lasso</span>
              <span class="result-subtext">Decent until season three, then bad</span>
            </li>
            <li>
              <span class="result-heading">Sublime Text</span>
              <span class="result-subtext">OG hacker tool</span>
            </li>
            <li>
              <span class="result-heading">pkgx</span>
              <span class="result-subtext">run stuff</span>
            </li>
            <li>
              <span class="result-heading">Neovim</span>
              <span class="result-subtext">Great example of how an application should be extensible</span>
            </li>
            <li>
              <span class="result-heading">Tauri</span>
              <span class="result-subtext">Make cross-platform apps great again</span>
            </li>
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
