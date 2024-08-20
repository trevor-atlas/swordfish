import {
  BROWSER_HISTORY_RESULT,
  CALCULATOR_RESULT,
  FILE_RESULT,
} from '../types';
import { getSelectedResult } from '../react/reactStore';
import { ResultPreview } from '../types/ResultPreview';

const isCalculator = (
  preview: any,
): preview is Extract<ResultPreview, { type: 'Calculator' }> => {
  return preview && preview.type === CALCULATOR_RESULT;
};

const isFile = (
  preview: any,
): preview is Extract<ResultPreview, { type: 'File' }> => {
  return preview && preview.type === FILE_RESULT;
};

const isBrowserHistory = (
  preview: any,
): preview is Extract<ResultPreview, { type: 'BrowserHistory' }> => {
  return preview && preview.type === BROWSER_HISTORY_RESULT;
};

export default function Preview() {
  const result = getSelectedResult();

  return (
    <div className="preview-container">
      {result && isBrowserHistory(result.preview) && (
        <iframe
          width="300"
          height="200"
          frameBorder="0"
          sandbox="allow-scripts allow-same-origin allow-cross-origin"
          src={result.subheading}
          referrerPolicy="no-referrer"
          style={{
            width: '100%',
            height: '100%',
            maxWidth: '100%',
            maxHeight: '100%',
            border: 0,
          }}
        />
      )}
      {result && isFile(result.preview) && (
        <div className="flex flex-col">
          <div>{result.preview.path}</div>
          <div>{JSON.stringify(result.preview, null, 2)}</div>
        </div>
      )}
      {result && isCalculator(result.preview) && (
        <div
          dangerouslySetInnerHTML={{ __html: result?.preview?.parsedContent }}
        />
      )}
    </div>
  );
}
