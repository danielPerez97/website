import React from 'react';
import ReactMarkdown from 'react-markdown';
import PropTypes from 'prop-types';
import styled from 'styled-components';

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

const BlogPost = ({ body }) => (
  <BlogPostContainer>
    <MarkDownContainer>
      <ReactMarkdown>
        {body}
      </ReactMarkdown>
    </MarkDownContainer>
  </BlogPostContainer>
);

BlogPost.propTypes = {
  body: PropTypes.string.isRequired,
};

export default BlogPost;
