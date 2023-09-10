import React from 'react';
import * as ReactDOM from 'react-dom/client';
import {
  createBrowserRouter, RouterProvider,
} from 'react-router-dom';
import './index.css';
import Root from './routes/root';
import BlogList from './components/blog/BlogList';
import { loadBlogPosts as postsLoader } from './components/blog/loaders/loadBlogPosts';
import ErrorPage from './ErrorPage';
import App from './App';

const container = document.getElementById('#root');
const router = createBrowserRouter([
  {
    path: '/',
    element: <Root />,
    errorElement: <ErrorPage />,
    children: [
      {
        path: '/blog',
        element: <BlogList />,
        loader: postsLoader,
        errorElement: <ErrorPage />,
      },
      {
        path: '/',
        element: <App />,
        errorElement: <ErrorPage />,
      },
    ],
  },

]);

ReactDOM.createRoot(container).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
);
