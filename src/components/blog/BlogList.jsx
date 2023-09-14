import React from 'react';
import { useLoaderData } from 'react-router-dom';
import styled from 'styled-components';
import BlogCard from './BlogCard';

const UnorderedList = styled.ul`
  list-style-type: none;
`;

function BlogList() {
  const { blogs } = useLoaderData();
  return (
    <div>
      <UnorderedList>
        {
          blogs.map(({
            id,
            title,
            shortDescription,
          }) => (
            <BlogCard key={id} id={id} title={title} shortDescription={shortDescription} />
          ))
                  }
      </UnorderedList>
    </div>
  );
}

export default BlogList;
