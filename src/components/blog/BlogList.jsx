import React from 'react';
import { Link, useLoaderData } from 'react-router-dom';
import styled from 'styled-components';
import { ContentContainer } from '../../App';

const UnorderedList = styled.ul`
  list-style-type: none;
`;

const ListItem = styled.li`
  list-style-type: none;
  display: flex;
  font-family: sans-serif;
  padding-top: 8px;
  padding-bottom: 8px;
`;

function BlogList() {
  const { blogs } = useLoaderData();
  console.log(`blogs: ${blogs}`);
  return (
    <ContentContainer>
      <div>
        <UnorderedList>
          {
                      blogs.map(({
                        id,
                        title,
                      }) => (
                        <ListItem key={id}>
                          <Link to={`/blog/${id}`}>{title}</Link>
                        </ListItem>
                      ))
                  }
        </UnorderedList>
      </div>
    </ContentContainer>
  );
}

export default BlogList;
