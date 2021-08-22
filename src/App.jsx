import React, { useState, useEffect } from 'react';
import { Route, BrowserRouter } from 'react-router-dom';
import styled from 'styled-components';
import GradleBasics from './posts/GradleBasics.md';
import JetpackMusings from './posts/JetpackMusings.md';
import Header from './components/Header';
import BlogList from './components/BlogList';
import BlogPost from './components/BlogPost';
import Footer from './components/Footer';
import ProjectCard from './components/ProjectCard';

const ContentContainer = styled.div`
  display: flex;
  flex-flow: column;
  flex-direction: column;
  height: calc(100vh - 120px);
  justify-content: start;
  align-items: start;
  width: calc(100vw - 80px);
`;

const AppContainer = styled.div`
  display: flex;
  flex-direction: column;
  min-height: 100vh;
`;

function App() {
  const [posts, setPosts] = useState([]);

  useEffect(() => {
    const loadBlogPosts = async () => {
      const gradleBasicsText = (await (await fetch(GradleBasics)).text());
      const jetpackMusingsText = (await (await fetch(JetpackMusings)).text());

      setPosts([
        {
          id: 1,
          title: 'Gradle Basics',
          body: gradleBasicsText,
        },
        {
          id: 2,
          title: 'Musings on Jetpack Compose',
          body: jetpackMusingsText,
        },
      ]);
    };

    loadBlogPosts();
  }, []);

  return (
    <AppContainer>
      <BrowserRouter>
        <Header />
        <ContentContainer>
          <Route
            path="/"
            exact
            render={() => (
              <ProjectCard
                projectName="Evence"
                projectDescription="Evence is an Android app to quickly create and scan QR codes for event details."
              />
            )}
          />
          <Route
            path="/blog"
            exact
            render={() => <BlogList blogs={posts} />}
          />
          <Route
            exact
            path="/blog/:id"
            render={({ match }) => (
              <BlogPost
                body={posts.find((post) => post.id === parseInt(match.params.id, 10)).body}
              />
            )}
          />
        </ContentContainer>
        <Footer />
      </BrowserRouter>
    </AppContainer>
  );
}

export default App;
