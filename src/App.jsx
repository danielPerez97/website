import React from 'react';
import styled from 'styled-components';
import AboutMe from './components/AboutMe';

const Container = styled.div`
  padding: 10px
`;

function App() {
  return (
    <Container>
      <div>SITE UNDER CONSTRUCTION</div>
      <div>My name is Daniel Perez. I am an Android Developer.</div>
      <AboutMe />
    </Container>
  );
}

export default App;
