import { useStore } from './reactStore';

export function QueryInput({
  inputRef,
}: {
  inputRef: React.RefObject<HTMLInputElement>;
}) {
  const { search_string, setSearchString } = useStore();
  return (
    <div
      className="search-input-container draggable-area"
      data-tauri-drag-region
    >
      <input
        className="search-input"
        ref={inputRef}
        onKeyDown={(event) => {
          if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
            event.preventDefault();
          }
        }}
        autoFocus={true}
        type="text"
        spellCheck={false}
        value={search_string}
        onInput={(e) => {
          setSearchString(e.currentTarget.value);
        }}
      />
    </div>
  );
}
