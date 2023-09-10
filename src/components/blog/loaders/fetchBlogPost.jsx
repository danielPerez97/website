import loadBlogPosts from './loadBlogPosts';

export const blogPostLoader = async ({ params }) => {
  const posts = await loadBlogPosts();
  return posts.find((post) => post.id === parseInt(params.postId.params.id, 10)).body;
};

export default blogPostLoader;
