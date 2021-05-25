import React from 'react';
import { Link } from 'react-router-dom';
import PropTypes from 'prop-types';
import styled from 'styled-components';

const BlogList = ({ blogs }) => {
  const ulStyle = {
    'list-style-type': 'none',
  };

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

  return (
    <div>
      <UnorderedList style={ulStyle}>
        {
          blogs.map(({ id, title }) => (
            <ListItem>
              <Link to={`/blog/${id}`}>{title}</Link>
            </ListItem>
          ))
        }
      </UnorderedList>
    </div>
  );
};

BlogList.propTypes = {
  blogs: PropTypes.arrayOf(PropTypes.shape({
    id: PropTypes.number,
    title: PropTypes.string,
  })).isRequired,
};

export default BlogList;
