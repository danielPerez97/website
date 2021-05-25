import React from 'react';
import ReactMarkdown from 'react-markdown';
import PropTypes from 'prop-types';
import styled from 'styled-components';

const BlogPost = ({ body }) => {
  const BlogPostContainer = styled.div`
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: 5px;
  `;

  const MarkDownContainer = styled.div`
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    font-family: monospace;
  `;

  return (
    <BlogPostContainer>
      <MarkDownContainer>
        <ReactMarkdown>
          {body}
        </ReactMarkdown>
      </MarkDownContainer>
    </BlogPostContainer>
  );
};

BlogPost.propTypes = {
  body: PropTypes.string.isRequired,
};

export default BlogPost;
