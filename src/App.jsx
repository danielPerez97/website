import React, { useState, useEffect } from 'react';
import { Route, BrowserRouter } from 'react-router-dom';
import styled from 'styled-components';
import GradleBasics from './posts/GradleBasics.md';
import JetpackMusings from './posts/JetpackMusings.md';
import AboutMe from './components/AboutMe';
import Header from './components/Header';
import BlogList from './components/BlogList';
import BlogPost from './components/BlogPost';
import Footer from './components/Footer';

const Container = styled.div`
    display: flex;
    flex-Direction: column;
    height: calc(100vh - 120px);
    justify-content: center;
    align-items: center;
    width: calc(100vw - 80px);
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
    <div>
      <BrowserRouter>
        <Header />
        <Container>
          <Route path="/" exact component={AboutMe} />
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
        </Container>
        <Footer />
      </BrowserRouter>
    </div>
  );
}

export default App;
