import React from 'react';
import { Link } from 'react-router-dom';
import styled, { css } from 'styled-components';

const HeaderContainer = styled.div`
  position: sticky;
  top: 0;
  height: 60px;
`;

const StyledUnorderedList = styled.ul`
  display: flex;
  flex-direction: row;
  list-style-type: none;
  margin: 0;
  padding: 0;
  overflow: hidden;
  background-color: #333333;
  height: 100%;
`;

const ListItem = styled.li`
  float: left;
  height: 100%;
`;

const BaseStyledLinkAndP = css`
  display: block;
  height: 60px;
  color: white;
  text-align: center;
  margin-left: 16px;
  margin-right: 16px;
  text-decoration: none;
  font-family: monospace;
  line-height: 60px;
  font-size: 20px;
`;

const StyledA = styled.a`
  ${BaseStyledLinkAndP};
`;

const StyledLink = styled(Link)`
  ${BaseStyledLinkAndP};
`;

function Header() {
  return (
    <HeaderContainer>
      <StyledUnorderedList>
        <ListItem>
          <StyledLink to="/">Daniel Perez</StyledLink>
        </ListItem>
        <ListItem>
          <StyledLink to="/">About Me</StyledLink>
        </ListItem>
        <ListItem>
          <StyledLink to="/blog">Blog</StyledLink>
        </ListItem>
        <ListItem>
          <StyledA href="https://www.github.com/danielPerez97">GitHub</StyledA>
        </ListItem>
      </StyledUnorderedList>
    </HeaderContainer>
  );
}

export default Header;
