import React from 'react';
import { Link } from 'react-router-dom';
import PropTypes from 'prop-types';
import styled from 'styled-components';

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

const BlogList = ({ blogs }) => (
  <div>
    <UnorderedList>
      {
          blogs.map(({ id, title }) => (
            <ListItem key={id}>
              <Link to={`/blog/${id}`}>{title}</Link>
            </ListItem>
          ))
        }
    </UnorderedList>
  </div>
);

BlogList.propTypes = {
  blogs: PropTypes.arrayOf(PropTypes.shape({
    id: PropTypes.number,
    title: PropTypes.string,
  })).isRequired,
};

export default BlogList;
