import { createEffect } from 'solid-js';

interface ResultProps {
  heading: string;
  subtext: string;
  selected: boolean;
}

export default function Result({ heading, subtext, selected }: ResultProps) {
  let ref!: HTMLLIElement;

  createEffect(() => {
    if (ref && selected) {
      ref.scrollIntoView({
        behavior: 'auto',
        block: 'center',
      });
    }
  });

  return (
    <li ref={ref} class={selected ? 'active' : ''}>
      <span class="result-heading">{heading}</span>
      <span class="result-subtext">{subtext}</span>
    </li>
  );
}
