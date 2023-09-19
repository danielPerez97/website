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

function AboutMe() {
  return (
    <Container>
      <Content>
        <StyledImg src={ProfilePicture} alt="Daniel Perez" />
        <div style={{ wordWrap: 'break-word', textAlign: 'center' }}>
          My name is Daniel Perez and I am a Java and Android Developer based out of Colorado.
        </div>
      </Content>
    </Container>
  );
}

// noinspection JSUnusedGlobalSymbols
export default AboutMe;
