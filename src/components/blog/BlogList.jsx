import React from 'react';
import { Link, useLoaderData } from 'react-router-dom';
import styled from 'styled-components';
import BlogCard from './BlogCard';

const UnorderedList = styled.ul`
  list-style-type: none;
`;

function BlogList() {
  const { blogs } = useLoaderData();
  console.log(`blogs: ${blogs}`);
  return (
    <div>
      <UnorderedList>
        {
          blogs.map(({
            id,
            title,
            shortDescription,
          }) => (
            <BlogCard key={id} title={title} shortDescription={shortDescription}>
              <Link to={`/blog/${id}`}>{title}</Link>
            </BlogCard>
          ))
                  }
      </UnorderedList>
    </div>
  );
}

export default BlogList;
