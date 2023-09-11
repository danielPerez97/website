import { loadBlogPosts as postsLoader } from './loadBlogPosts';

export const blogPostLoader = async ({ params }) => {
  const posts = (await postsLoader()).blogs;
  return posts.find((post) => post.id === parseInt(params.postId, 10));
};

export default blogPostLoader;
