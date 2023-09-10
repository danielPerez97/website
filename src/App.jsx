import React from 'react';
import styled from 'styled-components';
import ProjectCard from './components/ProjectCard';

export const ContentContainer = styled.div`
  //flex: 1 0 auto;
  //height: 100%;
  //justify-content: start;
  //align-items: start;
  //width: calc(100vw - 80px);
`;

const AppContainer = styled.div`
  //display: flex;
  //flex-direction: column;
  //justify-content: space-between;
  min-height: 100vh;
`;

function App() {
  return (
    <AppContainer>
      <ContentContainer>
        <div>hello</div>
        <ProjectCard
          projectName="Evence"
          projectDescription="Evence is an Android app to quickly create and scan QR codes for event details."
        />
      </ContentContainer>
    </AppContainer>
  );
}

export default App;
