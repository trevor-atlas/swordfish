import { useCallback, useState } from 'react';

type TooltipProps = {
  title: string;
  content: string;
  children: () => any;
};

export function Tooltip({ title, content }: TooltipProps) {
  const [showTooltip, setShowTooltip] = useState(false);
  const onEnter = useCallback(() => setShowTooltip(true), []);
  const onLeave = useCallback(() => setShowTooltip(false), []);

  return (
    <div
      role="tooltip"
      className="relative"
      onMouseEnter={onEnter}
      onMouseLeave={onLeave}
    >
      {showTooltip && (
        <>
          <div className="tooltip bottom-12 z-10 absolute inline-block px-3 py-2 text-sm font-medium text-white duration-300 bg-gray-900 rounded-lg shadow-sm dark:bg-gray-700">
            {title}
            <div className="tooltip-arrow" />
          </div>
          {content}
        </>
      )}
    </div>
  );
}
