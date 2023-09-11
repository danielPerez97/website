/* eslint-disable */
import React from 'react';
import ReactMarkdown from 'react-markdown';
import styled from 'styled-components';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { okaidia } from 'react-syntax-highlighter/dist/esm/styles/prism';
import { useLoaderData } from 'react-router-dom';

const BlogPostContainer = styled.div`
  left: 10pc;
  display: flex;
  width: 100%;
  flex-direction: row;
  justify-content: start;
  align-items: flex-start;
  padding: 10px;
`;

const MarkDownContainer = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: flex-start;
  font-family: monospace;
`;

function BlogPost() {
  const { body } = useLoaderData();
  return (
    <BlogPostContainer>
      <MarkDownContainer>
        <ReactMarkdown components={{
          // eslint-disable-next-line react/no-unstable-nested-components
          code({
            node, inline, className, children, ...props
          }) {
            const match = /language-(\w+)/.exec(className || '');
            return !inline && match ? (
              <SyntaxHighlighter
                {...props}
                children={String(children).replace(/\n$/, '')}
                style={okaidia}
                language={match[1]}
                PreTag="div"
              />
            ) : (
              <code {...props} className={className}>
                {children}
              </code>
            );
          },
        }}
        >
          {body}
        </ReactMarkdown>
      </MarkDownContainer>
    </BlogPostContainer>
  );
}

export default BlogPost;
