import { createEffect, createSignal } from 'solid-js';
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

  return (
    <li
      ref={ref}
      class={`${state.cursor === index ? 'active' : ''} query-result-${index}`}
      onMouseEnter={handleMouseEvent}
      onMouseMove={handleMouseEvent}
    >
      <span class="result-heading">{heading}</span>
      <span class="result-subtext">{subtext}</span>
    </li>
  );
}
