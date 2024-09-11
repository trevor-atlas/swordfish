// import { emit, listen } from '@tauri-apps/api/event';

const { emit, listen } = (window.__TAURI__ as any).event;
const { invoke } = window.__TAURI__;

console.log(window.__TAURI__);
function listener({ payload }: { payload: string }) {
  try {
    const script = document.createElement('script');
    script.onload = script.onerror = function () {
      this.remove();
    };
    script.type = 'module';
    script.text = `
      const { emit, listen } = window.__TAURI__.event;
      const { invoke } = window.__TAURI__;
      emit('ScriptResult', 'wow cool!');
      ${payload}
      `;
    document.body.appendChild(script);
  } catch (e) {
    console.log(e);
  }
}

const unlisten = listen('RunScript', listener);

// import { emit, listen } from '@tauri-apps/api/event';

// function listener({ payload }: { payload: string }) {
//   try {
//     const callback = Function(
//       `'use strict';
//         return (${payload})`,
//     );
//     console.log(callback(emit));
//   } catch (e) {
//     console.log(e);
//   }
// }

// const unlisten = listen('RunScript', listener);
