import DaggerByExample from '../../../posts/PragmaticDaggerArticle1.md';

export async function loadBlogPosts() {
  const daggerByExampleText = (await (await fetch(DaggerByExample)).text());
  // eslint-disable-next-line no-console
  const blogs = [
    {
      id: 1,
      title: 'A Pragmatic Introduction To Dagger on Android',
      shortDescription: 'Learning to use Dagger for the first time, thermosiphon and car engine free.',
      publishDate: '9/19/2023',
      body: daggerByExampleText,
    },
  ];

  return { blogs };
}

export default loadBlogPosts;
