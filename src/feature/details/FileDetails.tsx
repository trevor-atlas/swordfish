import {
  FILE_RESULT,
} from '../../types';
import { ResultPreview } from '../../types/ResultPreview';
import { convertFileSrc } from '@tauri-apps/api/tauri';

type FileDetailsProps = {
  preview: Extract<ResultPreview, { type: typeof FILE_RESULT }>
}

export function FileDetails({preview}: FileDetailsProps) {
  if (!preview) {
    console.log(preview)
    return <>What the fuck</>
  }
  return <div className="flex flex-col">
    <div>{preview.path}</div>
    <div>{preview.size}</div>
    <div>{preview.type}</div>
    {preview.fileType === 'image' ? (
<img src={convertFileSrc(preview.path)} />
    ) : null}
                
    <div>{JSON.stringify(preview, null, 2)}</div>
  </div>
}
