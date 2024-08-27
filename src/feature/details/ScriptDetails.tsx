import React, { CSSProperties } from 'react';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { oneDark } from 'react-syntax-highlighter/dist/esm/styles/prism';
import { ResultPreview } from '../../types/ResultPreview';
import classes from './scriptDetails.module.scss';

const ScriptDetails: React.FC<Extract<ResultPreview, { type: 'Script' }>> = ({
  path,
  language,
  parsedContent,
  lastModified
}) => {
  return (
    <div className={classes.scriptDetails}>
      <h3>{path}</h3>
      <p className="mb-3">Path: {path}</p>
      <p className="mb-3">Last modified: {lastModified}</p>
      {Boolean(parsedContent) ? (
        <SyntaxHighlighter
            language={language || 'text'}
            style={oneDark}
            wrapLongLines
            wrapLines
            customStyle={{
              margin: 0,
              borderRadius: 0,
              userSelect: 'initial',
              '-webkit-user-select': 'initial'
            }as CSSProperties}
          >
          {parsedContent as string}
        </SyntaxHighlighter>
      ) : null}
    </div>
  );
};

export default ScriptDetails;
