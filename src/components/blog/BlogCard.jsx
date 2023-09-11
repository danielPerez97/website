import React from 'react';
import PropTypes from 'prop-types';
import styled from 'styled-components';
import { Link } from 'react-router-dom';

const ListItem = styled.a`
  list-style-type: none;
  display: flex;
  flex-direction: column;
  font-family: sans-serif;
  padding-top: 8px;
  padding-bottom: 8px;
`;

function BlogCard({ id, title, shortDescription }) {
  return (
    <ListItem>
      <Link to={`/blog/${id}`}>
        <h2>{title}</h2>
        <div>{shortDescription}</div>
      </Link>
    </ListItem>
  );
}

BlogCard.propTypes = {
  id: PropTypes.number.isRequired,
  title: PropTypes.string.isRequired,
  shortDescription: PropTypes.string.isRequired,
};

export default BlogCard;
