import { QUERY_MODES } from '../../constants';
import { useStore } from '../../react/reactStore';

const actions = QUERY_MODES.map((str) => ({
  title: str,
}));

export function ActionSelector() {
  const { mode } = useStore();
  return (
    <div className="action-selector">
      {actions.map((item) => (
        <div
          key={item.title}
          className={`${QUERY_MODES[mode] === item.title && 'active'} action `}
        >
          <span>{item.title}</span>
        </div>
      ))}
    </div>
  );
}
