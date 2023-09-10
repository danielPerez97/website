import React from 'react';
import { Route, Routes } from 'react-router-dom';
import Header from '../components/Header';
import App from '../App';
import Footer from '../components/Footer';

function Root() {
  return (
    <React.StrictMode>
      <Header />
      <Routes>
        <Route path="/" element={<App />} />
      </Routes>
      <Footer />
    </React.StrictMode>
  );
}

export default Root;
