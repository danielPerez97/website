import React from 'react';
import { Outlet } from 'react-router-dom';
import styled from 'styled-components';
import Header from '../components/Header';
import Footer from '../components/Footer';

const PageContainer = styled.div`
  display: flex;
  flex-direction: column;
  height: 100vh;
`;

const ContentContainer = styled.div`
  flex-grow: 1;
  width: 100%;
`;

const StyledFooter = styled(Footer)`
  content: '\\00a0';
  display: block;
  margin-top: var(--space);
  height: 0;
  visibility: hidden;
`;

function Root() {
  return (
    <React.StrictMode>
      <PageContainer>
        <Header />
        <ContentContainer>
          <Outlet />
        </ContentContainer>
        <StyledFooter />
      </PageContainer>
    </React.StrictMode>
  );
}

export default Root;
