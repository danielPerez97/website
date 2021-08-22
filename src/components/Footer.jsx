import React from 'react';
import styled, { css } from 'styled-components';

const FooterContainer = styled.div`
  position: sticky;
  bottom: 0;
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
  padding-left: 16px;
  padding-right: 16px;
  text-decoration: none;
  font-family: monospace;
  line-height: 60px;
  margin: 0;
`;

const StyledA = styled.a`
  ${BaseStyledLinkAndP};
  font-size: 20px;
`;

// const StyledLink = styled(Link)`
//     ${BaseStyledLinkAndP}
//   `;

const Footer = () => (
  <FooterContainer>
    <StyledUnorderedList>
      <ListItem>
        <StyledA>Contact Me</StyledA>
      </ListItem>
    </StyledUnorderedList>
  </FooterContainer>
);

export default Footer;
