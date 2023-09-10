import GradleBasics from '../../../posts/GradleBasics.md';
import JetpackMusings from '../../../posts/JetpackMusings.md';

export async function loadBlogPosts() {
  const gradleBasicsText = (await (await fetch(GradleBasics)).text());
  const jetpackMusingsText = (await (await fetch(JetpackMusings)).text());
  // eslint-disable-next-line no-console
  console.log('loadBlogPosts() called');
  const blogs = [
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
  ];

  return { blogs };
}

export default loadBlogPosts;
