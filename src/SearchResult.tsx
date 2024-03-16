import { Show, createEffect, createSignal } from 'solid-js';
import { useStore } from './store';
import { throttle } from '@solid-primitives/scheduled';

interface ResultProps {
  heading: string;
  subtext: string;
  index: number;
}

const lastMousePos = { x: 0, y: 0 };

export default function Result({ heading, subtext, index }: ResultProps) {
  const [state, { setCursor }] = useStore();

  let ref!: HTMLLIElement;

  const handleMouseEvent = (event: MouseEvent) => {
    if (
      event.screenX !== lastMousePos.x ||
      (event.screenY !== lastMousePos.y && state.cursor !== index)
    ) {
      setCursor(index);
    }
    lastMousePos.x = event.screenX;
    lastMousePos.y = event.screenY;
  };
  const active = state.cursor === index;

  return (
    <li
      ref={ref}
      class={`${active ? 'active' : ''} query-result query-result-${index}`}
      onMouseEnter={handleMouseEvent}
      onMouseMove={handleMouseEvent}
    >
      <span class="result-heading">{heading}</span>
      <span class="result-subtext">{subtext}</span>
      <ActiveIndicator active={active} />
      <span class="keyboard-shortcut">
        <kbd>⌘</kbd>
        <kbd>⇧</kbd>
        <kbd>C</kbd>
      </span>
    </li>
  );
}

function ActiveIndicator({ active }: { active: boolean }) {
  return (
    <Show when={active}>
      <div class="leading-1 ml-2 flex h-6 w-6 items-center justify-center rounded bg-text-base bg-opacity-10 fill-current text-xs font-bold text-primary/90 transition ease-in hover:bg-opacity-20 hover:text-primary/90" />
    </Show>
  );
}
