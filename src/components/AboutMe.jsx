import React from 'react';
import styled from 'styled-components';
import ProfilePicture from '../images/DanielPerez.jpeg';

const Container = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: center;
  padding-top: 10px;
  width: 100vw;
`;

const StyledImg = styled.img`
  width: 400px;
`;

const Content = styled.div`
  width: 400px;
`;

const CenteredText = styled.div`
  word-wrap: break-word;
  text-align: center;
`;

function AboutMe() {
  return (
    <Container>
      <Content>
        <StyledImg src={ProfilePicture} alt="Daniel Perez" />
        <CenteredText>
          My name is Daniel Perez and I am a Java and Android Developer based out of Denver,
          Colorado.
        </CenteredText>
      </Content>
    </Container>
  );
}

// noinspection JSUnusedGlobalSymbols
export default AboutMe;
