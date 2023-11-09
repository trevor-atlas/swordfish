import { For, Show } from 'solid-js';

function useGPT() {
  return [
    { from: 'me', text: 'Hello' },
    { from: 'them', text: 'Hi there, how can I help?' },
  ];
}

export function Chat() {
  const messages = useGPT();

  return (
    <div class="messages-container flex flex-col mt-5">
      <For each={messages}>
        {(message) => (
          <>
            <Show when={message.from === 'me'}>
              <div class={`flex justify-end mb-4 message ${message.from}`}>
                <div class="mr-2 py-3 px-4 bg-blue-400 rounded-bl-3xl rounded-tl-3xl rounded-tr-xl text-white">
                  {message.text}
                </div>
              </div>
            </Show>
            <Show when={message.from !== 'me'}>
              <div class={` flex justify-start mb-4 message ${message.from}`}>
                <div class="ml-2 py-3 px-4 bg-neutral-700 rounded-br-3xl rounded-tr-3xl rounded-tl-xl text-white">
                  {message.text}
                </div>
              </div>
            </Show>
          </>
        )}
      </For>
    </div>
  );
}
