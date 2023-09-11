import React from 'react';
import { Outlet } from 'react-router-dom';
import styled from 'styled-components';
import Header from '../components/Header';
import Footer from '../components/Footer';

const AppContainer = styled.div`
  min-height: 100vh;
`;

function Root() {
  return (
    <React.StrictMode>
      <Header />
      <AppContainer>
        <Outlet />
      </AppContainer>
      <Footer />
    </React.StrictMode>
  );
}

export default Root;
