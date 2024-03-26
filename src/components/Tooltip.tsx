import { Show, children, createSignal } from 'solid-js';

export function Tooltip(props) {
  const [showTooltip, setShowTooltip] = createSignal(false);
  const c = children(() => props.children);
  const content = children(() => props.content);

  return (
    <div
      role="tooltip"
      class="relative"
      onMouseEnter={() => setShowTooltip(true)}
      onMouseLeave={() => setShowTooltip(false)}
    >
      <Show when={showTooltip()}>
        <div class="tooltip bottom-12 z-10 absolute inline-block px-3 py-2 text-sm font-medium text-white duration-300 bg-gray-900 rounded-lg shadow-sm dark:bg-gray-700">
          {content()}
          <div class="tooltip-arrow" />
        </div>
      </Show>
      {c()}
    </div>
  );
}
