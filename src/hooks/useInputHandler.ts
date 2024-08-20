import { useEffect } from 'react';

export function useInputHandler(onPress: (event: KeyboardEvent) => void) {
  useEffect(() => {
    window.addEventListener('keydown', onPress);
    return () => window.removeEventListener('keydown', onPress);
  }, [onPress]);
}
