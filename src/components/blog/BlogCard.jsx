import React from 'react';
import PropTypes from 'prop-types';
import styled from 'styled-components';

const ListItem = styled.li`
  list-style-type: none;
  display: flex;
  font-family: sans-serif;
  padding-top: 8px;
  padding-bottom: 8px;
`;

function BlogCard({ title, shortDescription }) {
  return (
    <ListItem>
      <div>{title}</div>
      <div>{shortDescription}</div>
    </ListItem>
  );
}

BlogCard.propTypes = {
  title: PropTypes.string.isRequired,
  shortDescription: PropTypes.string.isRequired,
};

export default BlogCard;
