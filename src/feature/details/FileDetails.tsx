import {
  FILE_RESULT,
} from '../../types';
import { ResultPreview } from '../../types/ResultPreview';
import { convertFileSrc } from '@tauri-apps/api/tauri';

type FileDetailsProps = {
  details: Extract<ResultPreview, { type: typeof FILE_RESULT }>
}

export function FileDetails({ details }: FileDetailsProps) {
  if (!details) {
    return <>What the fuck</>
  }
const lastModified = details.lastModified ? new Date(parseInt(details.lastModified, 10) * 1000).toLocaleString() : null
  const content = (() => {
    switch (details.fileType) {
      case 'Image':
        return <div>
        <img src={convertFileSrc(details.path)} style={{
          maxHeight: '100%',
          maxWidth: '100%',
          width: 'auto'
        }} />
          <div>{details.path}</div>
          <div>{details.size}</div>
          {lastModified ? <div>last modified: {lastModified}</div> : null}

        </div>
      case 'Pdf':
        return <iframe src={convertFileSrc(details.path)}
        style={{
          minHeight: '100%',
          flexGrow: 1
        }}
        />;
      default:
      return (
        <>
          <div>{details.path}</div>
          <div>{details.size}</div>
          <div>{details.type}</div>
          <pre>{JSON.stringify(details, null, 2)}</pre>
        </>
      )
    }
  })();

  return (
    <div className="flex flex-col h-full">
      {content}
    </div>
  );
}
