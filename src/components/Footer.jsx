import React from 'react';
import styled from 'styled-components';

const Footer = () => {
  const FooterContainer = styled.div`
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

  return (
    <FooterContainer>
      <StyledUnorderedList>
        Footer
      </StyledUnorderedList>
    </FooterContainer>
  );
};

export default Footer;
