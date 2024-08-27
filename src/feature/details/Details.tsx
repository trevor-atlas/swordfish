import {
  BROWSER_HISTORY_RESULT,
  CALCULATOR_RESULT,
  FILE_RESULT,
  SCRIPT_RESULT,
} from '../../types';
import { getSelectedResult } from '../../react/reactStore';
import { ResultPreview } from '../../types/ResultPreview';
import ScriptDetails from '../../feature/details/ScriptDetails';
import { FileDetails } from '../../feature/details/FileDetails';

const isCalculator = (
  preview: ResultPreview,
): preview is Extract<ResultPreview, { type: typeof CALCULATOR_RESULT }> =>
  preview?.type === CALCULATOR_RESULT;

const isFile = (
  preview: ResultPreview,
): preview is Extract<ResultPreview, { type: typeof FILE_RESULT }> =>
  preview?.type === FILE_RESULT;

const isBrowserHistory = (
  preview: ResultPreview,
): preview is Extract<ResultPreview, { type: typeof BROWSER_HISTORY_RESULT }> =>
  preview?.type === BROWSER_HISTORY_RESULT;

const isScript = (
  preview: ResultPreview
): preview is Extract<ResultPreview, { type: typeof SCRIPT_RESULT }> => preview?.type === SCRIPT_RESULT

export default function Details() {
  const result = getSelectedResult();

  if (!result?.preview) {
    return null;
  }

  const preview = result.preview;

  return (
    <div className="preview-container grow">
      {isFile(preview) ? (
        <FileDetails preview={preview} />
      ): null}
      {isCalculator(preview) && (
        <div
          dangerouslySetInnerHTML={{ __html: preview?.parsedContent }}
        />
      )}
      {isScript(preview) && <ScriptDetails {...preview} />}
    </div>
  );
}
