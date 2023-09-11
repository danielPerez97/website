import React from 'react';
import ReactMarkdown from 'react-markdown';
import styled from 'styled-components';
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
        <ReactMarkdown>
          {body}
        </ReactMarkdown>
      </MarkDownContainer>
    </BlogPostContainer>
  );
}

export default BlogPost;
